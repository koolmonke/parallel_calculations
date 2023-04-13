use rayon::ThreadPoolBuilder;

use crate::{bench_result::*, common::get_duration};

fn make_pool(num_threads: usize) -> rayon::ThreadPool {
    ThreadPoolBuilder::new()
        .num_threads(num_threads)
        .build()
        .unwrap()
}

fn bench_f<F: Fn() + Send + Sync>(f: F, thread_count: usize, repeat_count: i32) {
    let pool = make_pool(thread_count);
    for _ in 0..repeat_count {
        pool.install(&f);
    }
}

pub fn bench_single_threaded<F: Fn() + Send + Sync>(
    f: F,
    repeat_count: i32,
) -> BenchResultSingleThreaded {
    let duration = get_duration(move || bench_f(&f, 1, repeat_count));

    BenchResultSingleThreaded {
        duration,
        repeat_count,
    }
}

pub fn bench<F: Fn() + Send + Sync>(
    f: F,
    thread_count: usize,
    repeat_count: i32,
    single_threaded_result: &BenchResultSingleThreaded,
) -> BenchResultMultiThreaded {
    let duration = get_duration(move || bench_f(&f, thread_count, repeat_count));

    let speedup = single_threaded_result.duration.as_secs_f64() / duration.as_secs_f64();
    let effectiveness = speedup / (thread_count as f64);

    BenchResultMultiThreaded {
        duration,
        speedup,
        effectiveness,
        thread_count,
        repeat_count,
    }
}

#[macro_export]
macro_rules! generate_bench {
    ($repeat_count:expr, $thread_counts:expr, $bench_fn:ident, $($rest:tt)*) => {{
        let single_threaded_result = bench_single_threaded(
            move || {
                $bench_fn($($rest)*);
            },
            $repeat_count,
        );

        println!("{}\n", single_threaded_result);
        for thread_count in $thread_counts {
            let bench_result = bench(
                move || {
                    $bench_fn($($rest)*);
                },
                thread_count,
                $repeat_count,
                &single_threaded_result,
            );
            println!("{}\n", bench_result);
        }
    }};
}
