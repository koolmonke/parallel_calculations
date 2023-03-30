use std::time::{Duration, SystemTime};

pub(crate) fn get_duration<F: FnOnce()>(f: F) -> Duration {
    let start_time = SystemTime::now();
    f();
    let end_time = SystemTime::now();
    end_time.duration_since(start_time).unwrap()
}
