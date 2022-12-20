use crate::helpers::check_window_size;
use anyhow::Error;

use crate::Close;

pub struct SmaFactory {
    window_size: usize,
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
            period: self.window_size,
            index: 0,
            count: 0,
            deque: vec![0.0; self.window_size].into_boxed_slice(),
            sum: 0.0,
        })
    }
}

impl<T: crate::Close> crate::indicators::MovingAverage<T> for SMA {
    fn window_size(&self) -> usize {
        self.period
    }
}

impl SMA {
    pub fn factory() -> SmaFactory {
        SmaFactory::new()
    }
}

#[derive(Clone, Debug)]
pub struct SMA {
    period: usize,
    index: usize,
    count: usize,
    sum: f64,
    deque: Box<[f64]>,
}

impl<T: Close> crate::Indicator<T> for SMA {
    type Output = f64;

    fn next(&mut self, input: T) -> Self::Output {
        let old_val = self.deque[self.index];
        self.deque[self.index] = input.close();

        self.index = if self.index + 1 < self.period {
            self.index + 1
        } else {
            0
        };

        if self.count < self.period {
            self.count += 1;
        }

        self.sum = self.sum - old_val + input.close();
        self.sum / (self.count as f64)
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
