use crate::helpers::check_window_size;
use anyhow::Error;
use std::collections::VecDeque;

use crate::Close;

pub struct SmaFactory {
    window_size: usize,
}

#[derive(Clone, Debug)]
pub struct SMA {
    window: VecDeque<f64>,
    window_size: usize,
    sum: f64,
}

impl SmaFactory {
    pub fn new() -> Self {
        Self { window_size: 14 }
    }
    pub fn with_window_size(self, window_size: usize) -> Self {
        Self { window_size }
    }
    pub fn build(self) -> Result<SMA, Error> {
        check_window_size(self.window_size)?;
        Ok(SMA {
            window: VecDeque::new(),
            window_size: self.window_size,
            sum: 0.0,
        })
    }
}

impl<T: Close> crate::Indicator<T> for SMA {
    type Output = f64;
    fn next(&mut self, next: T) -> Self::Output {
        self.window.push_back(next.close());
        if self.window.len() > self.window_size {
            self.sum -= self.window.pop_front().unwrap();
            self.sum += next.close();
        } else {
            self.sum += next.close();
        }

        self.sum / self.window.len() as f64
    }
}

impl<T: crate::Close> crate::indicators::MovingAverage<T> for SMA {
    fn window_size(&self) -> usize {
        self.window_size
    }
}

impl SMA {
    pub fn factory() -> SmaFactory {
        SmaFactory::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::{indicators::SMA, Indicator};

    #[test]
    fn test_build() {
        SMA::factory().with_window_size(5).build().unwrap();
    }

    #[test]
    fn test_next() {
        let mut sma = SMA::factory().with_window_size(5).build().unwrap();
        assert_eq!(sma.next(10.0), 10.0);
        assert_eq!(sma.next(20.0), 15.0);
        assert_eq!(sma.next(15.0), 15.0);
        assert_eq!(sma.next(10.0), 13.75);
        assert_eq!(sma.next(10.0), 13.0);
        assert_eq!(sma.next(10.0), 13.0);
        assert_eq!(sma.next(12.5), 11.5);
    }
}
