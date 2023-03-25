use std::{
    fmt::{self, Display, Formatter},
    time::Duration,
};

#[derive(Debug)]
pub struct BenchResultSingleThreaded {
    pub duration: Duration,
}

impl Display for BenchResultSingleThreaded {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        writeln!(f, "1 поток")?;
        write!(
            f,
            "Время выполнения: {:.5} секунд",
            self.duration.as_secs_f64()
        )
    }
}
