use anyhow::Error;

use crate::*;

#[derive(Clone, Debug, PartialEq)]
pub enum PsarTrend {
    Up,
    Down,
}

#[derive(Clone, Debug)]
struct CHL {
    close: f64,
    high: f64,
    low: f64,
}

#[derive(PartialEq, Debug)]
pub struct PsarResult {
    trend: PsarTrend,
    sar: f64,
}

impl Close for CHL {
    fn close(&self) -> f64 {
        self.close
    }
}

impl High for CHL {
    fn high(&self) -> f64 {
        self.high
    }
}

impl Low for CHL {
    fn low(&self) -> f64 {
        self.low
    }
}

pub struct PsarFactory {
    af_max: f64,
    af_step: f64,
}

#[derive(Clone, Debug)]
pub struct PSAR {
    max_accel: f64,
    accel_step: f64,
    trend: PsarTrend,
    trend_inc: u32,
    low: f64,
    high: f64,
    sar: f64,
    prev_candle: CHL,
    prev_trend: PsarTrend,
}

impl PsarFactory {
    pub fn new() -> Self {
        Self {
            af_max: 0.2,
            af_step: 0.02,
        }
    }

    pub fn with_max_accel(mut self, max: f64) -> Self {
        self.af_max = max;
        self
    }

    pub fn with_accel_step(mut self, step: f64) -> Self {
        self.af_step = step;
        self
    }

    pub fn build(self) -> Result<PSAR, Error> {
        Ok(PSAR {
            max_accel: self.af_max,
            accel_step: self.af_step,
            trend: PsarTrend::Down,
            trend_inc: 0,
            low: std::f64::NAN,
            high: std::f64::NAN,
            sar: std::f64::NAN,
            prev_candle: CHL {
                close: 0.0,
                high: 0.0,
                low: 0.0,
            },
            prev_trend: PsarTrend::Down,
        })
    }
}

impl<T: Close + High + Low> crate::Indicator<T> for PSAR {
    type Output = PsarResult;
    fn next(&mut self, next: T) -> Self::Output {
        match self.trend {
            PsarTrend::Up => {
                if self.high < next.high() {
                    self.high = next.high();
                    self.trend_inc += 1;
                }
                if next.low() < self.sar {
                    self.trend = PsarTrend::Down;
                    self.low = next.low();
                    self.trend_inc = 1;
                    self.sar = self.high;
                }
            }
            PsarTrend::Down => {
                if self.low > next.low() {
                    self.low = next.low();
                    self.trend_inc += 1;
                }
                if next.high() > self.sar {
                    self.trend = PsarTrend::Up;
                    self.high = next.high();
                    self.trend_inc = 1;
                    self.sar = self.low;
                }
            }
        };

        let trend = self.trend.clone();

        let af = self
            .max_accel
            .min(self.accel_step * (self.trend_inc as f64));

        match self.trend {
            PsarTrend::Up => {
                self.sar = af.mul_add(self.high - self.sar, self.sar);
                self.sar = self.sar.min(next.low()).min(self.prev_candle.low());
            }
            PsarTrend::Down => {
                self.sar = af.mul_add(self.low - self.sar, self.sar);
                self.sar = self.sar.max(next.high()).max(self.prev_candle.high());
            }
        }

        self.prev_candle = CHL {
            close: next.close(),
            high: next.high(),
            low: next.low(),
        };

        self.prev_trend = trend;

        PsarResult {
            trend: self.trend.clone(),
            sar: self.sar,
        }
    }
}

impl<T: Close + High + Low> crate::indicators::Trend<T> for PSAR {}

impl PSAR {
    pub fn factory() -> PsarFactory {
        PsarFactory::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        indicators::trend::psar::PsarResult, indicators::trend::psar::CHL, indicators::PSAR,
        Indicator,
    };

    #[test]
    fn test_build() {
        PSAR::factory()
            .with_accel_step(0.02)
            .with_max_accel(0.2)
            .build()
            .unwrap();
    }

    #[test]
    fn test_next() {
        let mut psar = PSAR::factory().build().unwrap();
        assert_eq!(
            psar.next(CHL {
                close: 10.0,
                high: 15.0,
                low: 5.0,
            }),
            PsarResult {
                trend: crate::indicators::trend::psar::PsarTrend::Down,
                sar: 15.0,
            }
        );

        assert_eq!(
            psar.next(CHL {
                close: 15.0,
                high: 20.0,
                low: 10.0,
            }),
            PsarResult {
                trend: crate::indicators::trend::psar::PsarTrend::Up,
                sar: 5.0,
            }
        );

        assert_eq!(
            psar.next(CHL {
                close: 20.0,
                high: 25.0,
                low: 15.0,
            }),
            PsarResult {
                trend: crate::indicators::trend::psar::PsarTrend::Up,
                sar: 5.8,
            }
        );

        assert_eq!(
            psar.next(CHL {
                close: 25.0,
                high: 30.0,
                low: 20.0,
            }),
            PsarResult {
                trend: crate::indicators::trend::psar::PsarTrend::Up,
                sar: 7.252,
            }
        );

        assert_eq!(
            psar.next(CHL {
                close: 20.0,
                high: 25.0,
                low: 15.0,
            }),
            PsarResult {
                trend: crate::indicators::trend::psar::PsarTrend::Up,
                sar: 8.61688,
            }
        );

        assert_eq!(
            psar.next(CHL {
                close: 15.0,
                high: 20.0,
                low: 10.0,
            }),
            PsarResult {
                trend: crate::indicators::trend::psar::PsarTrend::Up,
                sar: 9.8998672,
            }
        );

        assert_eq!(
            psar.next(CHL {
                close: 10.0,
                high: 15.0,
                low: 5.0,
            }),
            PsarResult {
                trend: crate::indicators::trend::psar::PsarTrend::Down,
                sar: 29.5,
            }
        );

        assert_eq!(
            psar.next(CHL {
                close: 10.0,
                high: 15.0,
                low: 5.0,
            }),
            PsarResult {
                trend: crate::indicators::trend::psar::PsarTrend::Down,
                sar: 29.01,
            }
        );
    }
}
