use crate::{bench_result::*, common::get_duration};

pub fn bench_single_threaded<F: FnOnce() + Send>(f: F) -> BenchResultSingleThreaded {
    let duration = get_duration(f, 1);
    BenchResultSingleThreaded { duration }
}

pub fn bench<F: FnOnce() + Send>(
    f: F,
    thread_count: usize,
    single_threaded_result: &BenchResultSingleThreaded,
) -> BenchResultMultiThreaded {
    let duration = get_duration(f, thread_count);
    let speedup = single_threaded_result.duration.as_secs_f64() / duration.as_secs_f64();
    let effectiveness = speedup / (thread_count as f64);

    BenchResultMultiThreaded {
        duration,
        speedup,
        effectiveness,
        thread_count,
    }
}
