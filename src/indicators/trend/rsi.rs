use crate::*;
use anyhow::Error;

pub struct RsiFactory<T>
where
    T: indicators::MovingAverage<f64>,
{
    up_indicator: T,
    down_indicator: T,
}

pub struct RSI<T>
where
    T: indicators::MovingAverage<f64>,
{
    up_indicator: T,
    down_indicator: T,
    prev: f64,
    is_new: bool,
}

impl<T: indicators::MovingAverage<f64>> RsiFactory<T> {
    pub fn new() -> RsiFactory<indicators::EMA> {
        RsiFactory {
            up_indicator: indicators::EMA::factory().build().unwrap(),
            down_indicator: indicators::EMA::factory().build().unwrap(),
        }
    }

    pub fn with_moving_average<U: Clone + indicators::MovingAverage<f64>>(
        self,
        moving_average: U,
    ) -> RsiFactory<U> {
        RsiFactory {
            up_indicator: moving_average.clone(),
            down_indicator: moving_average,
        }
    }

    pub fn build(self) -> Result<RSI<T>, Error> {
        Ok(RSI {
            up_indicator: self.up_indicator,
            down_indicator: self.down_indicator,
            prev: 0.0,
            is_new: true,
        })
    }
}

impl<T: indicators::MovingAverage<f64>, U: Close> Indicator<U> for RSI<T> {
    type Output = f64;
    fn next(&mut self, next: U) -> Self::Output {
        let mut up = 0.0;
        let mut down = 0.0;
        let new = next.close();

        if self.is_new {
            self.is_new = false;
        } else {
            if new > self.prev {
                up = new - self.prev;
            } else {
                down = self.prev - new;
            }
        }

        // avoid division by 0
        if down == 0.0 {
            if up == 0.0 {
                up = 4.94065645841e-324; // smallest non-zero f64 in scientific notation
            }
            down = 4.94065645841e-324;
        }

        self.prev = new;
        let up_ema = self.up_indicator.next(up);
        let down_ema = self.down_indicator.next(down);
        100.0 - (100.0 / (1.0 + (up_ema / down_ema)))
    }
}

impl<T: indicators::moving_average::MovingAverage<f64>, U: indicator::Close> indicators::Trend<U>
    for RSI<T>
{
}

impl<T: indicators::MovingAverage<f64>> RSI<T> {
    pub fn factory() -> RsiFactory<indicators::EMA> {
        RsiFactory::<T>::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::{indicators::RSI, indicators::SMA, Indicator};

    #[test]
    fn test_build() {
        RSI::<SMA>::factory()
            .with_moving_average(SMA::factory().build().unwrap())
            .build()
            .unwrap();
    }

    #[test]
    fn test_next() {
        let mut rsi = RSI::<SMA>::factory()
            .with_moving_average(SMA::factory().with_window_size(5).build().unwrap())
            .build()
            .unwrap();

        assert_eq!(rsi.next(10.0), 50.0);
        assert_eq!(rsi.next(20.0), 100.0);
        assert_eq!(rsi.next(15.0), 66.66666666666666);
        assert_eq!(rsi.next(10.0), 50.0);
        assert_eq!(rsi.next(10.0), 50.0);
        assert_eq!(rsi.next(10.0), 50.0);
        assert_eq!(rsi.next(12.5), 20.0);
    }
}
