use std::time::{SystemTime, UNIX_EPOCH};

pub fn reverse_string(s: &str) -> String {
    s.chars().rev().collect()
}

pub fn string_to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}

pub fn datetime_now() -> u64 { // We lose some precision, but it's okay...
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

    (since_the_epoch.as_secs_f64() * 1000.0) as u64
}