use std::{
    hint::black_box,
    time::{Duration, Instant},
};

pub(crate) fn get_duration<F: FnOnce()>(f: F) -> Duration {
    let instant = Instant::now();
    black_box(f());
    instant.elapsed()
}
