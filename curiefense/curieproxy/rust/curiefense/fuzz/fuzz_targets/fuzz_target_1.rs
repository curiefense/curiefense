#![no_main]
use libfuzzer_sys::fuzz_target;
use std::collections::HashMap;

use curiefense::config::with_config;
use curiefense::grasshopper::DynGrasshopper;
use curiefense::incremental::{add_body, add_header, finalize, inspect_init};
use curiefense::logs::{LogLevel, Logs};
use curiefense::utils::RequestMeta;

#[derive(arbitrary::Arbitrary, Debug)]
struct RequestFuzzData {
    meta: RequestMeta,
    headers: HashMap<String, String>,
    body: Option<Vec<u8>>,
}

fuzz_target!(|data: RequestFuzzData| {
    let mut logs = Logs::default();
    let loglevel = LogLevel::Debug;
    let mbody = data.body;
    let meta = data.meta;
    let headers = data.headers;
    let midata = with_config("/cf-config/current/config", &mut logs, |_, cfg| {
        inspect_init(cfg, loglevel, meta, 1).map(|o| {
            // we have to clone all this data here :(
            // that would not be necessary if we could avoid the autoreloading feature, but had a system for reloading the server when the configuration changes
            let gf = cfg.globalfilters.clone();
            let fl = cfg.flows.clone();
            (o, gf, fl)
        })
    });
    if let Some(Ok((idata, gf, fl))) = midata {
        if let Ok(idata) = add_header(idata, headers) {
            if let Some(body) = mbody {
                if let Ok(idata) = add_body(idata, body) {
                    async_std::task::block_on(finalize(idata, Some(DynGrasshopper {}), &gf, &fl, None));
                }
            } else {
                async_std::task::block_on(finalize(idata, Some(DynGrasshopper {}), &gf, &fl, None));
            }
        }
    }
});
