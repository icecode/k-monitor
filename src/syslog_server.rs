extern crate serde;
extern crate serde_json;

use tokio::net::UdpSocket;
use std::collections;
use std::str;
use lazy_static::lazy_static;

use prometheus::{IntCounter, register_int_counter, register_int_counter_vec, register_histogram_vec};

lazy_static! {

    static ref NGINX_SYSLOG_UDP_RECEIVE_COUNTER:IntCounter = register_int_counter!("nginx_syslog_udp_receive", "receive syslog udp log counter").unwrap();

    static ref NGINX_SYSLOG_UDP_PARSE_COUNTER:IntCounter = register_int_counter!("nginx_syslog_udp_parse", "receive syslog udp parse success counter").unwrap();

}

pub async fn udp_listen(adds: String) -> std::io::Result<()> {
    let sock = UdpSocket::bind(adds.clone()).await?;
    let mut buf = [0u8; 4096];
    let mut service_counter_map = collections::HashMap::new();
    let mut service_duration_map = collections::HashMap::new();
    println!("syslog udp://{}", &adds);
    loop {
        let (len, _) = sock.recv_from(&mut buf).await?;
        NGINX_SYSLOG_UDP_RECEIVE_COUNTER.inc();
        println!("syslog receive value:{}", str::from_utf8(&buf[..len]).unwrap());
        if len < 41 {
            println!("syslog receive illegal value:{}", str::from_utf8(&buf[..len]).unwrap());
            continue;
        }
        if let Ok(s) = str::from_utf8(&buf[41..len]) {
            if let Some(log) = try_parse_from_json(s) {
                NGINX_SYSLOG_UDP_PARSE_COUNTER.inc();
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
                    service_duration.with_label_values(&["all", "all"]).observe(log.request_time);
                    service_duration.with_label_values(&[log.domain.as_str(), "127.0.0.1"]).observe(log.request_time);
                });
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AccessLog {
    pub remote_addr: String,
    pub service: String,
    pub domain: String,
    pub request: String,
    pub status: u32,
    pub sent_body_bytes: u32,
    pub request_time: f64
}

pub fn try_parse_from_json(access_line: &str) -> Option<AccessLog> {
    match serde_json::from_str(access_line) {
        Ok(log) => Some(log),
        _ => None
    }
}


#[test]
fn parse_log() {
    let json_str = r#"{"timestamp": "19/Apr/2021:09:21:27 +0000", "remote_addr": "172.17.0.1", "service": "localhost", "domain": "127.0.0.1", "request": "GET /favicon.ico HTTP/1.1", "status": 500, "sent_body_bytes": 580, "agent": "Mozilla/5.0 (Macintosh; Intel Mac OS X 11_2_3) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/89.0.4389.128 Safari/537.36", "x_forwarded": "-", "up_addr": "-","up_host": "-","upstream_response_time": "-","request_time": 0.000}"#;
    let len = json_str.len();
    println!("{}", json_str);
    let log = try_parse_from_json(json_str).unwrap();
    assert_eq!("172.17.0.1", log.remote_addr);
    assert_eq!("localhost", log.service);
    assert_eq!("127.0.0.1", log.domain);
    assert_eq!(500, log.status);
    assert_eq!(580, log.sent_body_bytes);
    assert_eq!(0.000_f64, log.request_time);
}