extern crate serde;
extern crate serde_json;


#[derive(Serialize, Deserialize, Debug)]
pub struct AccessLog {
    pub timestamp: i32,
    pub remote_addr: String,
    pub service: String,
    pub domain: String,
    pub request: String,
    pub status: u32,
    pub sent_body_bytes: u32,
    pub upstream_response_time: u32,
    pub request_time: u32
}


// log_format log_json '{ "timestamp": $time_local, '
// '"remote_addr": "$remote_addr", '
// '"referer": "$http_referer", '
// '"request": "$request", '
// '"status": $status, '
// '"sent_body_bytes": $body_bytes_sent, '
// '"agent": "$http_user_agent", '
// '"x_forwarded": "$http_x_forwarded_for", '
// '"up_addr": "$upstream_addr",'
// '"up_host": "$upstream_http_host",'
// '"upstream_response_time": $upstream_response_time,'
// '"request_time": $request_time'
// ' }';

pub fn try_parse_from_json(access_line: &str) -> Option<AccessLog> {
    match serde_json::from_str(access_line) {
        Ok(log) => Some(log),
        _ => None
    }
}


#[test]
fn test_parse_json() {
    let access_log = r#"
        {
            "timestamp": 123131231,
            "remote_addr": "127.0.0.1",
            "service": "test",
            "domain": "baidu.com",
            "request": "/query",
            "status": 200,
            "sent_body_bytes": 10,
            "upstream_response_time": 10,
            "request_time": 12
        }"#;
    let ret = LogParser::parse_from_json(access_log);
    assert_eq!(true, ret.is_some());
    if let Some(log) = ret {
        assert_eq!(12, log.request_time);
        assert_eq!(10, log.upstream_response_time);
        assert_eq!(10, log.sent_body_bytes);
        assert_eq!(200, log.status);
        assert_eq!("/query", log.request);
        assert_eq!("baidu.com", log.domain);
        assert_eq!("test", log.service);
        assert_eq!("127.0.0.1", log.remote_addr);
        assert_eq!(123131231, log.timestamp)
    }
}