use std::time::{Duration, SystemTime};

use rayon::ThreadPoolBuilder;

pub(crate) fn get_duration<F: FnOnce() + Send>(f: F, thread_count: usize) -> Duration {
    let start_time = SystemTime::now();
    make_pool(thread_count).install(f);
    let end_time = SystemTime::now();
    end_time.duration_since(start_time).unwrap()
}

pub(crate) fn make_pool(num_threads: usize) -> rayon::ThreadPool {
    ThreadPoolBuilder::new()
        .num_threads(num_threads)
        .build()
        .unwrap()
}
