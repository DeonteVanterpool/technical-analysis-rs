use std::collections::VecDeque;

use crate::*;
use anyhow::Error;
pub struct SdFactory<T>
where
    T: indicators::MovingAverage<f64>,
{
    moving_average: T,
}

pub struct SD<T>
where
    T: indicators::MovingAverage<f64>,
{
    moving_average: T,
    window: VecDeque<f64>,
    window_size: usize,
}

impl<T: indicators::MovingAverage<f64>> SdFactory<T> {
    pub fn new() -> SdFactory<indicators::SMA> {
        SdFactory {
            moving_average: indicators::SMA::factory().build().unwrap(),
        }
    }

    pub fn with_moving_average<U: indicators::MovingAverage<f64>>(
        self,
        moving_average: U,
    ) -> SdFactory<U> {
        SdFactory { moving_average }
    }

    pub fn build(self) -> Result<SD<T>, Error> {
        Ok(SD {
            window_size: self.moving_average.window_size(),
            moving_average: self.moving_average,
            window: VecDeque::new(),
        })
    }
}

impl<T: indicators::MovingAverage<f64>, U: Close> Indicator<U> for SD<T> {
    type Output = f64;
    fn next(&mut self, next: U) -> Self::Output {
        let avg = self.moving_average.next(next.close());

        self.window.push_back(next.close());
        if self.window.len() > self.window_size {
            self.window.pop_front().unwrap();
        }

        (self
            .window
            .iter()
            .map(|close| (close - avg).powi(2))
            .sum::<f64>()
            / (self.window.len() as f64))
            .sqrt()
    }
}

impl<T: indicators::MovingAverage<f64>> SD<T> {
    pub fn factory() -> SdFactory<indicators::SMA> {
        SdFactory::<T>::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::{indicators::SD, indicators::SMA, Indicator};

    #[test]
    fn test_build() {
        SD::<SMA>::factory()
            .with_moving_average(SMA::factory().build().unwrap())
            .build()
            .unwrap();
    }

    #[test]
    fn test_next() {
        let mut sd = SD::<SMA>::factory()
            .with_moving_average(SMA::factory().with_window_size(5).build().unwrap())
            .build()
            .unwrap();

        assert_eq!(sd.next(10.0), 0.0);
        assert_eq!(sd.next(20.0), 5.0);
        assert_eq!(sd.next(15.0), 4.08248290463863);
        assert_eq!(sd.next(10.0), 4.14578098794425);
        assert_eq!(sd.next(10.0), 4.0);
        assert_eq!(sd.next(10.0), 4.0);
        assert_eq!(sd.next(12.5), 2.0);
    }
}
