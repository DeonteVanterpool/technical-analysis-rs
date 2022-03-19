/// trait `Indicator<T>` has a function `next(T)` which moves the indicator forward
pub trait Indicator<T> {
    type Output;
    /// advance the given indicator with the given `input` value, which is of generic type `T`
    fn next(&mut self, next: T) -> Self::Output;
}

/// trait `Open` indicates an open price
pub trait Open {
    fn open(&self) -> f64;
}

/// trait `Close` indicates a close price. It is used as an input for most
/// `Indicator`s
pub trait Close {
    fn close(&self) -> f64;
}

/// trait `High` indicates a high price
pub trait High {
    fn high(&self) -> f64;
}

/// trait `Low` indicates a low price
pub trait Low {
    fn low(&self) -> f64;
}

/// trait `Volume` indicates a volume
pub trait Volume {
    fn volume(&self) -> u64;
}

impl Close for f64 {
    fn close(&self) -> f64 {
        *self
    }
}
