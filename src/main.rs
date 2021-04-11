#[macro_use]
extern crate serde_derive;
extern crate lazy_static;


mod log_parse;
mod web_prometheus;
mod udp_syslog_server;

use warp::{Filter};

#[tokio::main]
async fn main()-> () {
    tokio::spawn({
        udp_syslog_server::listen("0.0.0.0:8082")
    });
    let prometheus_endpoint = warp::path!("prometheus")
        .map( || {
            web_prometheus::end_point()
        });
    warp::serve(prometheus_endpoint)
        .run(([127, 0, 0, 1], 8000))
        .await
}


