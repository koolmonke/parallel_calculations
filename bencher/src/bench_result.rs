use std::{fmt, time::Duration};

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
