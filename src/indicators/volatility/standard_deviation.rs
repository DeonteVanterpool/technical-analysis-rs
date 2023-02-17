use crate::*;
use anyhow::Error;
use std::collections::VecDeque;
pub struct SdFactory<T>
where
    T: indicators::MovingAverage<f64> + Clone,
{
    moving_average: T,
}

pub struct SD<T>
where
    T: indicators::MovingAverage<f64> + Clone,
{
    moving_average: T,
    squares_average: T,
}

impl<T: indicators::MovingAverage<f64> + Clone> SdFactory<T> {
    pub fn new() -> SdFactory<indicators::SMA> {
        SdFactory {
            moving_average: indicators::SMA::factory().build().unwrap(),
        }
    }

    pub fn with_moving_average<U: indicators::MovingAverage<f64> + Clone>(
        self,
        moving_average: U,
    ) -> SdFactory<U> {
        SdFactory { moving_average }
    }

    pub fn build(self) -> Result<SD<T>, Error> {
        Ok(SD {
            moving_average: self.moving_average.clone(),
            squares_average: self.moving_average,
        })
    }
}

impl<T: indicators::MovingAverage<f64> + Clone, U: Close> Indicator<U> for SD<T> {
    type Output = f64;
    fn next(&mut self, next: U) -> Self::Output {
        let new = next.close();
        let average = self.moving_average.next(new);
        let squares_average = self.squares_average.next(new.powi(2));

        (squares_average - average.powi(2)).abs().sqrt()
    }
}

impl<T: indicators::MovingAverage<f64> + Clone> SD<T> {
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
        assert_eq!(sd.next(15.0), 4.082482904638629);
        assert_eq!(sd.next(10.0), 4.14578098794425);
        assert_eq!(sd.next(10.0), 4.0);
        assert_eq!(sd.next(10.0), 4.0);
        assert_eq!(sd.next(12.5), 2.0);
    }
}
