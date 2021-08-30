use curiefense::logs::Logs;
use curiefense::config::with_config;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let mut logs = Logs::default();
    with_config(path, &mut logs, |_, cfg| {
        println!("security policies:");
        for securitypolicy in &cfg.securitypolicies {
            println!("{:?}", securitypolicy);
        }
    });
    for l in logs.to_stringvec() {
        println!("{}", l);
    }
}
