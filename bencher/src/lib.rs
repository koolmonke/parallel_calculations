pub mod bench_result;

use bench_result::BenchResult;

use std::time::{Duration, SystemTime};

fn make_pool(num_threads: usize) -> rayon::ThreadPool {
    rayon::ThreadPoolBuilder::new()
        .num_threads(num_threads)
        .build()
        .unwrap()
}

pub fn bench_rayon_single_threaded<F: FnOnce() -> () + Send>(f: F) -> BenchResult {
    let start_time = SystemTime::now();
    make_pool(1).install(move || f());
    let end_time = SystemTime::now();
    let duration = end_time.duration_since(start_time).unwrap();
    BenchResult {
        duration,
        thread_count: 1,
        speedup: None,
        effectiveness: None,
    }
}

pub fn bench_rayon<F: FnOnce() -> () + Send>(
    n: usize,
    f: F,
    duration_of_single_threaded: &Duration,
) -> BenchResult {
    let start_time = SystemTime::now();
    make_pool(n).install(move || f());
    let end_time = SystemTime::now();
    let duration = end_time.duration_since(start_time).unwrap();
    let speedup = duration_of_single_threaded.as_secs_f64() / duration.as_secs_f64();
    let effectiveness = speedup / (n as f64);

    BenchResult {
        duration,
        thread_count: n,
        speedup: Some(speedup),
        effectiveness: Some(effectiveness),
    }
}
