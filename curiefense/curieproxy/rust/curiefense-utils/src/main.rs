use curiefense::logs::Logs;
use curiefense::config::with_config;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let mut logs = Logs::default();
    with_config(path, &mut logs, |_, cfg| {
        println!("url maps:");
        for urlmap in &cfg.urlmaps {
            println!("{:?}", urlmap);
        }
    });
    for l in logs.to_stringvec() {
        println!("{}", l);
    }
}
