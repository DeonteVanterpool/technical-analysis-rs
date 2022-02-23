pub trait Indicator<T> {
    type Output;
    fn next(&mut self, next: T) -> Self::Output;
}

pub trait Open {
    fn open(&self) -> f64;
}

/// trait Close is used as an input for most indicators. It should be a close price
pub trait Close {
    fn close(&self) -> f64;
}

pub trait High {
    fn high(&self) -> f64;
}

pub trait Low {
    fn low(&self) -> f64;
}

pub trait Volume {
    fn volume(&self) -> u64;
}

impl Close for f64 {
    fn close(&self) -> f64 {
        *self
    }
}
