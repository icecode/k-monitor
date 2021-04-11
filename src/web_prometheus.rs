extern crate prometheus;

use prometheus::{Encoder, default_registry, TextEncoder};

pub fn end_point() -> String {
    let mut buffer = Vec::new();
    let r = default_registry();
    let encoder = TextEncoder::new();
    let metric_families = r.gather();
    encoder.encode(&metric_families, &mut buffer ).unwrap();
    String::from_utf8(buffer.clone()).unwrap()
}