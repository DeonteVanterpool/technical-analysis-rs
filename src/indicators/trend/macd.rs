use crate::*;
use anyhow::Error;

pub struct MacdFactory<T, U, V>
where
    T: indicators::MovingAverage<f64>,
    U: indicators::MovingAverage<f64>,
    V: indicators::MovingAverage<f64>,
{
    fast_ma: T,
    slow_ma: U,
    signal_ma: V,
}

#[derive(PartialEq, Debug)]
pub struct MacdResult {
    pub macd: f64,
    pub signal: f64,
    pub histogram: f64,
}

pub struct MACD<T, U, V>
where
    T: indicators::MovingAverage<f64>,
    U: indicators::MovingAverage<f64>,
    V: indicators::MovingAverage<f64>,
{
    fast_ma: T,
    slow_ma: U,
    signal_ma: V,
}

impl<
        T: indicators::MovingAverage<f64>,
        U: indicators::MovingAverage<f64>,
        V: indicators::MovingAverage<f64>,
    > MacdFactory<T, U, V>
{
    pub fn new() -> MacdFactory<indicators::EMA, indicators::EMA, indicators::EMA> {
        MacdFactory {
            fast_ma: indicators::EMA::factory()
                .with_window_size(12)
                .build()
                .unwrap(),
            slow_ma: indicators::EMA::factory()
                .with_window_size(26)
                .build()
                .unwrap(),
            signal_ma: indicators::EMA::factory()
                .with_window_size(9)
                .build()
                .unwrap(),
        }
    }

    pub fn with_fast_ma<X: indicators::MovingAverage<f64>>(
        self,
        moving_average: X,
    ) -> MacdFactory<X, U, V> {
        MacdFactory {
            fast_ma: moving_average,
            slow_ma: self.slow_ma,
            signal_ma: self.signal_ma,
        }
    }

    pub fn with_slow_ma<X: indicators::MovingAverage<f64>>(
        self,
        moving_average: X,
    ) -> MacdFactory<T, X, V> {
        MacdFactory {
            fast_ma: self.fast_ma,
            slow_ma: moving_average,
            signal_ma: self.signal_ma,
        }
    }

    pub fn with_signal_ma<X: indicators::MovingAverage<f64>>(
        self,
        moving_average: X,
    ) -> MacdFactory<T, U, X> {
        MacdFactory {
            fast_ma: self.fast_ma,
            slow_ma: self.slow_ma,
            signal_ma: moving_average,
        }
    }

    pub fn build(self) -> Result<MACD<T, U, V>, Error> {
        Ok(MACD {
            fast_ma: self.fast_ma,
            slow_ma: self.slow_ma,
            signal_ma: self.signal_ma,
        })
    }
}

impl<
        T: indicators::MovingAverage<f64>,
        U: indicators::MovingAverage<f64>,
        V: indicators::MovingAverage<f64>,
        X: Close,
    > Indicator<X> for MACD<T, U, V>
{
    type Output = MacdResult;
    fn next(&mut self, next: X) -> Self::Output {
        let fast = self.fast_ma.next(next.close());
        let slow = self.slow_ma.next(next.close());

        let macd = fast - slow;
        let signal = self.signal_ma.next(macd);
        let histogram = macd - signal;

        MacdResult {
            macd,
            signal,
            histogram,
        }
    }
}

impl<
        T: indicators::MovingAverage<f64>,
        U: indicators::MovingAverage<f64>,
        V: indicators::MovingAverage<f64>,
        X: indicator::Close,
    > indicators::Trend<X> for MACD<T, U, V>
{
}

impl<
        T: indicators::MovingAverage<f64>,
        U: indicators::MovingAverage<f64>,
        V: indicators::MovingAverage<f64>,
    > MACD<T, U, V>
{
    pub fn factory() -> MacdFactory<indicators::EMA, indicators::EMA, indicators::EMA> {
        MacdFactory::<indicators::EMA, indicators::EMA, indicators::EMA>::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::{indicators::MacdResult, indicators::EMA, indicators::MACD, Indicator};

    #[test]
    fn test_build() {
        MACD::<EMA, EMA, EMA>::factory()
            .with_fast_ma(EMA::factory().with_window_size(12).build().unwrap())
            .with_slow_ma(EMA::factory().with_window_size(26).build().unwrap())
            .with_signal_ma(EMA::factory().with_window_size(9).build().unwrap())
            .build()
            .unwrap();
    }

    #[test]
    fn test_next() {
        let mut macd = MACD::<EMA, EMA, EMA>::factory().build().unwrap();

        assert_eq!(
            macd.next(10.0),
            MacdResult {
                macd: 0.0,
                signal: 0.0,
                histogram: 0.0
            }
        );

        assert_eq!(
            macd.next(20.0),
            MacdResult {
                macd: 0.7977207977207978,
                signal: 0.15954415954415957,
                histogram: 0.6381766381766383
            }
        );

        assert_eq!(
            macd.next(15.0),
            MacdResult {
                macd: 1.014764490547968,
                signal: 0.3305882257449213,
                histogram: 0.6841762648030467
            }
        );

        assert_eq!(
            macd.next(10.0),
            MacdResult {
                macd: 0.7743883012752608,
                signal: 0.4193482408509892,
                histogram: 0.3550400604242716
            }
        );

        assert_eq!(
            macd.next(10.0),
            MacdResult {
                macd: 0.5772344401496312,
                signal: 0.45092548071071764,
                histogram: 0.12630895943891357
            }
        );

        assert_eq!(
            macd.next(10.0),
            MacdResult {
                macd: 0.4161909940808375,
                signal: 0.44397858338474167,
                histogram: -0.02778758930390418
            }
        );

        assert_eq!(
            macd.next(12.5),
            MacdResult {
                macd: 0.4847046361171863,
                signal: 0.45212379393123064,
                histogram: 0.03258084218595564
            }
        );
    }
}
