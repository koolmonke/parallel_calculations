use std::{
    fmt,
    time::{Duration, SystemTime},
};

#[derive(Debug)]
pub struct BenchResult {
    pub thread_count: usize,
    pub duration: Duration,
    pub speedup: Option<f64>,
    pub effectiveness: Option<f64>,
}

impl fmt::Display for BenchResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{} потоков", self.thread_count)?;
        write!(
            f,
            "Время выполнения: {:.5} секунд",
            self.duration.as_secs_f64()
        )?;

        if let Some(speedup) = self.speedup {
            writeln!(f)?;
            write!(f, "Ускорение: {:.5}", speedup)?;
        }

        if let Some(effectiveness) = self.effectiveness {
            writeln!(f)?;
            write!(f, "Эффективность: {:.5}", effectiveness)?;
        }

        Ok(())
    }
}

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
        thread_count: 1,
        duration,
        speedup: None,
        effectiveness: None,
    }
}

pub fn bench_rayon<F: FnOnce() -> () + Send>(
    n: usize,
    f: F,
    duration_of_single_threaded: Duration,
) -> BenchResult {
    let start_time = SystemTime::now();
    make_pool(n).install(move || f());
    let end_time = SystemTime::now();
    let duration = end_time.duration_since(start_time).unwrap();
    let speedup = duration_of_single_threaded.as_secs_f64() / duration.as_secs_f64();
    let effectiveness = speedup / (n as f64);

    BenchResult {
        thread_count: n,
        duration,
        speedup: Some(speedup),
        effectiveness: Some(effectiveness),
    }
}
