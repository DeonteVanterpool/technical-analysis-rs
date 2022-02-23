use crate::*;
pub trait MovingAverage<T: Close>: crate::Indicator<T, Output = f64> {
    fn window_size(&self) -> usize;
}
