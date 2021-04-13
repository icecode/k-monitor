use tokio::net::UdpSocket;
use std::collections;
use std::str;
use crate::log_parse;
use lazy_static::lazy_static;

use prometheus::{IntCounter, register_int_counter, register_int_counter_vec, register_histogram_vec};

lazy_static! {

    static ref UDP_RECEIVE_COUNTER:IntCounter = register_int_counter!("syslog_udp_receive", "receive syslog udp log counter").unwrap();

    static ref UDP_PARSE_COUNTER:IntCounter = register_int_counter!("syslog_udp_parse", "receive syslog udp parse success counter").unwrap();


}

pub async fn udp_listen(adds: String) -> std::io::Result<()> {
    let sock = UdpSocket::bind(adds.clone()).await?;
    let mut buf = [0u8; 4096];
    let mut service_counter_map = collections::HashMap::new();
    let mut service_duration_map = collections::HashMap::new();
    println!("syslog udp://{}", &adds);
    loop {
        let (len, _) = sock.recv_from(&mut buf).await?;
        UDP_RECEIVE_COUNTER.inc();
        if let Ok(s) = str::from_utf8(&buf[..len]) {
            if let Some(log) = log_parse::try_parse_from_json(s) {
                UDP_PARSE_COUNTER.inc();
                let req_counter_name = log.service.clone() + &"_request".to_string();
                if !service_counter_map.contains_key(&req_counter_name) {
                    let service_counter = register_int_counter_vec!(req_counter_name.as_str(), "The HTTP request counter", &["domain", "backend"]).unwrap();
                    service_counter_map.insert(req_counter_name.clone(), service_counter);
                }
                service_counter_map.get(&req_counter_name).map( | counter | {
                    counter.with_label_values(&["all", "all"]).inc();
                    counter.with_label_values(&[log.domain.as_str(), "127.0.0.1"]).inc();
                });

                let req_duration_name = log.service.clone() + &"_duration".to_string();
                if !service_duration_map.contains_key(&req_duration_name) {
                    let service_duration = register_histogram_vec!(req_duration_name.as_str(), "The HTTP request latencies in seconds.", &["domain", "backend"]).unwrap();
                    service_duration_map.insert(req_duration_name.clone(), service_duration);
                }
                service_duration_map.get(&req_duration_name).map ( | service_duration | {
                    let dur_sec = log.request_time as f64 / 1000_f64;
                    service_duration.with_label_values(&["all", "all"]).observe(dur_sec);
                    service_duration.with_label_values(&[log.domain.as_str(), "127.0.0.1"]).observe(dur_sec);
                });
            }
        }
    }
}