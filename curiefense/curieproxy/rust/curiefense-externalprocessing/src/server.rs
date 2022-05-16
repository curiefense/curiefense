use curiefense::{
    config::{
        flow::{FlowElement, SequenceKey},
        globalfilter::GlobalFilterSection,
        with_config,
    },
    grasshopper::DynGrasshopper,
    incremental::{add_body, add_header, finalize, inspect_init, IData},
    interface::{jsonlog, Decision, Tags},
    logs::{LogLevel, Logs},
    utils::{RequestInfo, RequestMeta},
};
use elasticsearch::{http::transport::Transport, Elasticsearch};
use lazy_static::lazy_static;
use log::{debug, error, info, warn, LevelFilter};
use serde_json::Value;
use std::{collections::HashMap, sync::RwLock};
use structopt::StructOpt;
use syslog::{Facility, Formatter3164, LoggerBackend};
use tokio::{
    spawn,
    sync::mpsc::{self, error::SendError, Receiver, Sender},
};
use tokio_stream::wrappers::ReceiverStream;
use tonic::{transport::Server, Request, Status};

mod ext_proc;

use ext_proc::{
    external_processor_server::{ExternalProcessor, ExternalProcessorServer},
    processing_response, BodyResponse, HeaderMutation, HeaderValue, HeaderValueOption, HeadersResponse, HttpStatus,
    ImmediateResponse, ProcessingRequest, ProcessingResponse,
};

lazy_static! {
    static ref LOGGER: RwLock<Option<syslog::Logger<LoggerBackend, Formatter3164>>> = RwLock::new(None);
}

#[derive(Clone)]
pub struct MyEP {
    handle_replies: bool,
    reqchannel: Sender<CfgRequest>,
    logsender: Option<Sender<Value>>,
}

type CfgRequest = (
    RequestMeta,
    Sender<Option<Result<(IData, Vec<GlobalFilterSection>, HashMap<SequenceKey, Vec<FlowElement>>), String>>>,
);

/// this function loops and waits for configuration queries
/// it is done so that configuration requests are serialized
///
/// this potentially reduces paralellism, but also avoids the problem of queued configuration reloads
async fn configloop(rx: Receiver<CfgRequest>, configpath: &str, loglevel: LogLevel, trustedhops: u32) {
    let mut mrx = rx;
    loop {
        let (meta, sender) = match mrx.recv().await {
            None => {
                error!("should not happen, channel closed?");
                break;
            }
            Some(x) => x,
        };

        let mut logs = Logs::new(loglevel);
        let midata = with_config(configpath, &mut logs, |_, cfg| {
            inspect_init(cfg, loglevel, meta, trustedhops).map(|o| {
                // we have to clone all this data here :(
                // that would not be necessary if we could avoid the autoreloading feature, but had a system for reloading the server when the configuration changes
                let gf = cfg.globalfilters.clone();
                let fl = cfg.flows.clone();
                (o, gf, fl)
            })
        });
        show_logs(logs);
        match sender.send(midata).await {
            Ok(()) => (),
            Err(rr) => {
                error!("Could not send midata {}", rr);
                break;
            }
        }
    }
}

async fn logloop(rx: Receiver<Value>, client: Elasticsearch) {
    let mut mrx = rx;
    loop {
        match mrx.recv().await {
            None => {
                error!("should not happen, logging channel closed?");
                break;
            }
            Some(mut v) => {
                let now = chrono::Utc::now();
                if let Some(hm) = v.as_object_mut() {
                    // might be slow!
                    hm.insert("@timestamp".to_string(), serde_json::to_value(&now).unwrap());
                }
                let idx = now.format("curieaccesslog-%Y.%m.%d-000001").to_string();
                match client
                    .index(elasticsearch::IndexParts::Index(&idx))
                    .body(v)
                    .send()
                    .await
                {
                    Err(rr) => error!("When logging to ES: {}", rr),
                    Ok(response) => {
                        if !response.status_code().is_success() {
                            error!("When logging to ES: {:?}", response);
                        } else {
                            info!("{:?}", response);
                        }
                    }
                }
            }
        }
    }
}

impl MyEP {
    fn new(reqchannel: Sender<CfgRequest>, handle_replies: bool, logsender: Option<Sender<Value>>) -> Self {
        MyEP {
            handle_replies,
            reqchannel,
            logsender,
        }
    }

    // the main content handling loop
    async fn handle(
        self,
        tx: &mut Sender<Result<ProcessingResponse, Status>>,
        msg: &mut tonic::Streaming<ProcessingRequest>,
    ) -> Result<(), String> {
        // currently, the first request is for headers, and then we might get body parts
        async fn next_message(m: &mut tonic::Streaming<ProcessingRequest>) -> Result<ProcessingRequest, String> {
            m.message()
                .await
                .map_err(|s| s.to_string())?
                .ok_or_else(|| "No processing request".to_string())
        }

        let mut meta: HashMap<String, String> = HashMap::new();
        let mut mheaders: HashMap<String, String> = HashMap::new();
        let headers_only = match next_message(msg).await?.request {
            Some(ext_proc::processing_request::Request::RequestHeaders(headers)) => {
                if let Some(hdrmap) = headers.headers {
                    for h in hdrmap.headers {
                        match h.key.strip_prefix(':') {
                            None => {
                                mheaders.insert(h.key, h.value);
                            }
                            Some(m) => {
                                meta.insert(m.to_string(), h.value);
                            }
                        }
                    }
                }
                headers.end_of_stream
            }
            something_else => return Err(format!("Expected a RequestHeaders, but got {:?}", something_else)),
        };

        let meta = match RequestMeta::from_map(meta) {
            Ok(m) => m,
            Err(rr) => {
                error!("Could not get request meta: {}", rr);
                return Ok(());
            }
        };

        // get configuration data from the dedicated task
        let (rtx, mut rrx) = mpsc::channel(1);
        self.reqchannel.send((meta, rtx)).await.unwrap();
        let midata = rrx.recv().await;

        let (idata, globalfilters, flows) = midata.unwrap().unwrap().unwrap();

        let mut idata = match add_header(idata, mheaders) {
            Ok(i) => i,
            Err((logs, dec)) => {
                self.send_action(ProcessingStage::Headers, tx, dec, logs).await;
                return Ok(());
            }
        };

        if !headers_only {
            stage_pass(ProcessingStage::Headers, tx).await;
            loop {
                match next_message(msg).await?.request {
                    Some(ext_proc::processing_request::Request::RequestBody(bdy)) => {
                        idata = match add_body(idata, bdy.body) {
                            Ok(i) => i,
                            Err((logs, dec)) => {
                                self.send_action(ProcessingStage::Body, tx, dec, logs).await;
                                return Ok(());
                            }
                        };
                        if bdy.end_of_stream {
                            break;
                        }
                    }
                    something_else => return Err(format!("Expected a RequestBody, but got {:?}", something_else)),
                }
            }
        }

        let (dec, logs) = finalize(idata, Some(DynGrasshopper {}), &globalfilters, &flows, None).await;
        self.send_action(
            if headers_only {
                ProcessingStage::Headers
            } else {
                ProcessingStage::Body
            },
            tx,
            dec,
            logs,
        )
        .await;

        if self.handle_replies {
            let code: Option<u32> = match next_message(msg).await?.request {
                Some(ext_proc::processing_request::Request::ResponseHeaders(hdrs)) => hdrs
                    .headers
                    .iter()
                    .flat_map(|hm| hm.headers.iter())
                    .filter_map(|hv| {
                        if hv.key == ":status" {
                            hv.value.parse().ok()
                        } else {
                            None
                        }
                    })
                    .next(),

                something_else => {
                    error!("Expected a ResponseHeaders, but got {:?}", something_else);
                    None
                }
            };
            stage_pass(ProcessingStage::RHeaders, tx).await;
            debug!("* return code: {:?}", code);
        }
        Ok(())
    }

    async fn send_action(
        &self,
        stage: ProcessingStage,
        tx: &mut Sender<Result<ProcessingResponse, Status>>,
        action: (Decision, Tags, RequestInfo),
        logs: Logs,
    ) {
        let (decision, tags, masked_rinfo) = action;
        match &decision {
            Decision::Pass => stage_pass(stage, tx).await,
            Decision::Action(a) => {
                if a.block_mode {
                    tx.send(Ok(ProcessingResponse {
                        response: Some(ext_proc::processing_response::Response::ImmediateResponse(
                            ImmediateResponse {
                                status: Some(HttpStatus { code: a.status as i32 }),
                                details: format!("{}", a.reason),
                                body: a.content.clone(),
                                headers: a.headers.clone().map(mutate_headers),
                                grpc_status: None,
                            },
                        )),
                        ..Default::default()
                    }))
                    .await
                    .unwrap();
                } else {
                    stage_pass(stage, tx).await
                }
            }
        }

        let v = jsonlog(&decision, masked_rinfo, tags);
        for l in logs.to_stringvec() {
            debug!("{}", l);
        }
        info!("CFLOG {}", v);
        if let Some(tx) = &self.logsender {
            if let Err(rr) = tx.send(v).await {
                error!("Could not log: {}", rr);
            }
        }
    }
}

fn mutate_headers(headers: HashMap<String, String>) -> HeaderMutation {
    HeaderMutation {
        set_headers: headers
            .into_iter()
            .map(|(key, value)| HeaderValueOption {
                header: Some(HeaderValue { key, value }),
                append: None,
                append_action: 0,
            })
            .collect(),
        remove_headers: Vec::new(),
    }
}

async fn send_response(
    tx: &mut Sender<Result<ProcessingResponse, Status>>,
    r: processing_response::Response,
) -> Result<(), SendError<Result<ext_proc::ProcessingResponse, tonic::Status>>> {
    tx.send(Ok(ProcessingResponse {
        response: Some(r),
        ..Default::default()
    }))
    .await
}

#[derive(Clone, Copy)]
enum ProcessingStage {
    Headers,
    Body,
    RHeaders,
}

async fn stage_pass(stage: ProcessingStage, tx: &mut Sender<Result<ProcessingResponse, Status>>) {
    send_response(
        tx,
        match stage {
            ProcessingStage::Headers => {
                processing_response::Response::RequestHeaders(HeadersResponse { response: None })
            }
            ProcessingStage::Body => processing_response::Response::RequestBody(BodyResponse { response: None }),
            ProcessingStage::RHeaders => {
                processing_response::Response::ResponseHeaders(ext_proc::HeadersResponse { response: None })
            }
        },
    )
    .await
    .unwrap();
}

fn show_logs(logs: Logs) {
    let vlogs = logs.to_stringvec();
    if !vlogs.is_empty() {
        warn!("CONFIGURATION LOGS:");
        for l in vlogs {
            warn!("{}", l);
        }
    }
}

#[tonic::async_trait]
impl ExternalProcessor for MyEP {
    type ProcessStream = ReceiverStream<Result<ProcessingResponse, Status>>;
    async fn process(
        &self,
        request: Request<tonic::Streaming<ProcessingRequest>>,
    ) -> Result<tonic::Response<ReceiverStream<Result<ProcessingResponse, Status>>>, Status> {
        let (mut tx, rx) = mpsc::channel(4);
        let mut message = request.into_inner();

        let cep = self.clone();

        spawn(async move {
            if let Err(msg) = cep.handle(&mut tx, &mut message).await {
                error!("{}", msg);
                send_response(
                    &mut tx,
                    processing_response::Response::ImmediateResponse(ext_proc::ImmediateResponse {
                        status: Some(ext_proc::HttpStatus { code: 403 }),
                        headers: None,
                        body: String::new(),
                        grpc_status: None,
                        details: msg,
                    }),
                )
                .await
                .unwrap()
            }
            message.trailers().await.unwrap();
        });

        Ok(tonic::Response::new(ReceiverStream::new(rx)))
    }
}

#[derive(Debug, StructOpt)]
#[structopt(
    name = "cf-externalprocessing",
    about = "An envoy external processing server for curiefense."
)]

struct Opt {
    #[structopt(long, default_value = "0.0.0.0:50051")]
    listen: String,
    #[structopt(long)]
    configpath: String,
    #[structopt(long, default_value = "info")]
    loglevel: String,
    #[structopt(long, default_value = "1")]
    trustedhops: u32,
    #[structopt(long)]
    handle_replies: bool,
    #[structopt(long)]
    syslog: bool,
    #[structopt(long)]
    elasticsearch: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // note that there is a lot of performance left on the table because of the autoreload system
    // the reason is that with the asynchronous code, we can't borrow anything from the configuration,
    // but have to own everything, as there is no guarantee the configuration won't move under our feet.
    let opt = Opt::from_args();
    let addr = opt.listen.parse()?;
    let loglevel = opt.loglevel.parse()?;
    let level_filter = match &loglevel {
        LogLevel::Debug => LevelFilter::Debug,
        _ => LevelFilter::Info,
    };
    // initial configuration loading
    let mut logs = Logs::new(loglevel);
    with_config(&opt.configpath, &mut logs, |_, _| {});
    show_logs(logs);

    if opt.syslog {
        syslog::init_unix(Facility::LOG_USER, level_filter)?;
    } else {
        simplelog::TermLogger::init(
            level_filter,
            simplelog::Config::default(),
            simplelog::TerminalMode::Stdout,
            simplelog::ColorChoice::Auto,
        )?;
    };

    let (ctx, crx) = mpsc::channel(4);

    let _ = spawn(async move { configloop(crx, &opt.configpath, loglevel, opt.trustedhops).await });

    let mut logsender: Option<Sender<Value>> = None;

    if let Some(esurl) = opt.elasticsearch {
        let (logtx, logrx) = mpsc::channel(500);
        let transport = Transport::single_node(&esurl)?;
        let client = Elasticsearch::new(transport);
        logsender = Some(logtx);
        let _ = spawn(async move { logloop(logrx, client).await });
    }

    let ep = MyEP::new(ctx, opt.handle_replies, logsender);
    Server::builder()
        .accept_http1(true)
        .add_service(ExternalProcessorServer::new(ep))
        .serve(addr)
        .await?;

    Ok(())
}
