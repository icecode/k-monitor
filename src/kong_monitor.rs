extern crate serde;
extern crate serde_json;
extern crate prometheus;

use std::str;
use tokio::net::UdpSocket;
use std::collections;
use lazy_static::lazy_static;
use prometheus::{register_int_counter, IntCounter};

lazy_static! {

    static ref KONG_UDP_RECEIVE_COUNTER:IntCounter = register_int_counter!("kong_udp_receive", "receive syslog udp log counter").unwrap();

    static ref KONG_UDP_PARSE_COUNTER:IntCounter = register_int_counter!("kong_udp_parse", "receive syslog udp parse success counter").unwrap();

}


pub async fn udp_listen(adds: &str) -> std::io::Result<()> {
    let sock = UdpSocket::bind(adds.clone()).await?;
    let mut buf = [0u8; 4096];
    // let mut service_counter_map = collections::HashMap::new();
    // let mut service_duration_map = collections::HashMap::new();
    println!("kong udp://{}", &adds);
    loop {
        let (len, _) = sock.recv_from(&mut buf).await?;
        KONG_UDP_RECEIVE_COUNTER.inc();
        if let Ok(s) = str::from_utf8(&buf[..len]) {

        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct AccessLog {
    latencies: Latencies,
    service: Service,
    tries: Vec<Tries>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Latencies {
    request: i32,
    kong: i32,
    proxy: i32
}

#[derive(Serialize, Deserialize, Debug)]
struct Service {
    host:String,
    created_at: u32,
    path: String
}

#[derive(Serialize, Deserialize, Debug)]
struct Tries {
    client_ip: String,
    balancer: Balancer
}

#[derive(Serialize, Deserialize, Debug)]
struct Balancer {
    ip: String,
    port: u16
}

#[test]
fn parse_log() {
    let json_str = r#"
 {
  "latencies": {
    "request": 515,
    "kong": 58,
    "proxy": 457
  },
  "service": {
    "host": "httpbin.org",
    "created_at": 1614232642,
    "connect_timeout": 60000,
    "id": "167290ee-c682-4ebf-bdea-e49a3ac5e260",
    "protocol": "http",
    "read_timeout": 60000,
    "port": 80,
    "path": "/anything",
    "updated_at": 1614232642,
    "write_timeout": 60000,
    "retries": 5,
    "ws_id": "54baa5a9-23d6-41e0-9c9a-02434b010b25"
  },
  "request": {
    "querystring": {},
    "size": 138,
    "uri": "/log",
    "url": "http://localhost:8000/log",
    "headers": {
      "host": "localhost:8000",
      "accept-encoding": "gzip, deflate",
      "user-agent": "HTTPie/2.4.0",
      "accept": "*/*",
      "connection": "keep-alive"
    },
    "method": "GET"
  },
  "tries": [
    {
      "balancer_latency": 0,
      "port": 80,
      "balancer_start": 1614232668399,
      "ip": "18.211.130.98"
    }
  ],
  "client_ip": "192.168.144.1",
  "workspace": "54baa5a9-23d6-41e0-9c9a-02434b010b25",
  "upstream_uri": "/anything",
  "response": {
    "headers": {
      "content-type": "application/json",
      "date": "Thu, 25 Feb 2021 05:57:48 GMT",
      "connection": "close",
      "access-control-allow-credentials": "true",
      "content-length": "503",
      "server": "gunicorn/19.9.0",
      "via": "kong/2.2.1.0-enterprise-edition",
      "x-kong-proxy-latency": "57",
      "x-kong-upstream-latency": "457",
      "access-control-allow-origin": "*"
    },
    "status": 200,
    "size": 827
  },
  "route": {
    "id": "78f79740-c410-4fd9-a998-d0a60a99dc9b",
    "paths": [
      "/log"
    ],
    "protocols": [
      "http"
    ],
    "strip_path": true,
    "created_at": 1614232648,
    "ws_id": "54baa5a9-23d6-41e0-9c9a-02434b010b25",
    "request_buffering": true,
    "updated_at": 1614232648,
    "preserve_host": false,
    "regex_priority": 0,
    "response_buffering": true,
    "https_redirect_status_code": 426,
    "path_handling": "v0",
    "service": {
      "id": "167290ee-c682-4ebf-bdea-e49a3ac5e260"
    }
  },
  "started_at": 1614232668342
}
    "#;

    let mut ret:Result<AccessLog, serde_json::Error> = serde_json::from_str(json_str);
    assert_eq!(true, ret.is_ok());
    assert_eq!(515, ret.as_mut().unwrap().latencies.request);
    assert_eq!(58, ret.as_mut().unwrap().latencies.kong);
    assert_eq!(457, ret.as_mut().unwrap().latencies.proxy)
}