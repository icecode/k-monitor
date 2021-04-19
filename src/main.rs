#[macro_use]
extern crate serde_derive;
extern crate lazy_static;
extern crate getopts;

mod web_prometheus;
mod syslog_server;
mod kong_monitor;

use getopts::Options;
use std::env;
use warp::{Filter};

#[tokio::main]
async fn main() -> () {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let mut opts = Options::new();
    opts.optopt(
        "U",
        "--syslog-udp-listen",
        "syslog udp listen address",
        "default is 0.0.0.0:8082",
    );
    opts.optopt(
        "W",
        "--http-listen",
        "http listen address, /prometheus ",
        "default is 0.0.0.0:8080",
    );

    opts.optflag("h", "help", "print this help menu");
    let matches = opts.parse(&args).unwrap();
    if matches.opt_present("h") {
        let brief = format!("Usage: {} [options]", program);
        print!("{}", opts.usage(&brief));
        return;
    }

    let http_listen_str = matches.opt_str("U").unwrap_or(String::from("0.0.0.0:8082"));
    tokio::spawn({
        syslog_server::udp_listen(http_listen_str.clone())
    });

    // let http_listen_str = matches.opt_str("W").unwrap_or(String::from("0.0.0.0:8080"));
    let prometheus_endpoint = warp::path!("prometheus")
        .map(|| {
            web_prometheus::end_point()
        });
    warp::serve(prometheus_endpoint)
        .run(([0, 0, 0, 0], 8080))
        .await
}


