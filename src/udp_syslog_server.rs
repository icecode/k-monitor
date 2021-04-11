use tokio::net::UdpSocket;
use std::collections;
use std::str;
use crate::log_parse;

use prometheus::{register_int_counter_vec, register_histogram_vec};

pub async fn listen(adds: &str) -> std::io::Result<()> {
    let sock = UdpSocket::bind(adds).await?;
    let mut buf = [0u8; 4096];
    let mut service_counter_map = collections::HashMap::new();
    let mut service_duration_map = collections::HashMap::new();
    loop {
        let (len, _) = sock.recv_from(&mut buf).await?;
        if let Ok(s) = str::from_utf8(&buf[..len]) {
            if let Some(log) = log_parse::try_parse_from_json(s) {
                println!("rev:{:?}", &log);
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