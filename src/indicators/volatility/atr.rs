use crate::*;
use anyhow::Error;

pub struct AtrFactory {
    window_size: usize,
}

pub struct ATR {
    window_size: usize,
    prev_atr: f64,
    tr_indicator: indicators::TR,
    is_new: bool,
}

impl AtrFactory {
    pub fn new() -> Self {
        AtrFactory { window_size: 14 }
    }

    pub fn with_window_size(mut self, window_size: usize) -> Self {
        self.window_size = window_size;
        self
    }

    pub fn build(self) -> Result<ATR, Error> {
        Ok(ATR {
            window_size: self.window_size,
            prev_atr: 0.0,
            tr_indicator: indicators::TR::factory().build().unwrap(),
            is_new: true,
        })
    }
}

impl<U: Close + High + Low> Indicator<U> for ATR {
    type Output = f64;
    fn next(&mut self, next: U) -> Self::Output {
        self.prev_atr = if self.is_new {
            self.is_new = false;
            self.tr_indicator.next(next)
        } else {
            (self.prev_atr * ((self.window_size as f64) - 1.0) + self.tr_indicator.next(next))
                / (self.window_size as f64)
        };
        self.prev_atr
    }
}

impl<U: indicator::Close + High + Low> indicators::Trend<U> for ATR {}

impl ATR {
    pub fn factory() -> AtrFactory {
        AtrFactory::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::{indicators::ATR, *};

    #[test]
    fn test_build() {
        ATR::factory().with_window_size(14).build().unwrap();
    }

    #[test]
    fn test_next() {
        let mut atr = ATR::factory().with_window_size(5).build().unwrap();

        #[derive(Clone, Debug)]
        struct CHL {
            close: f64,
            high: f64,
            low: f64,
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

        assert_eq!(
            atr.next(CHL {
                close: 8.0,
                high: 15.0,
                low: 5.0,
            }),
            10.0,
        );

        assert_eq!(
            atr.next(CHL {
                close: 15.0,
                high: 20.0,
                low: 7.0,
            }),
            10.6,
        );

        assert_eq!(
            atr.next(CHL {
                close: 22.0,
                high: 25.0,
                low: 15.0,
            }),
            10.48,
        );

        assert_eq!(
            atr.next(CHL {
                close: 25.0,
                high: 30.0,
                low: 14.0,
            }),
            11.584,
        );

        assert_eq!(
            atr.next(CHL {
                close: 20.0,
                high: 25.0,
                low: 15.9,
            }),
            11.0872,
        );

        assert_eq!(
            atr.next(CHL {
                close: 17.0,
                high: 20.0,
                low: 10.0,
            }),
            10.86976,
        );

        assert_eq!(
            atr.next(CHL {
                close: 10.0,
                high: 18.0,
                low: 5.0,
            }),
            11.295808,
        );

        assert_eq!(
            atr.next(CHL {
                close: 10.0,
                high: 15.0,
                low: 5.0,
            }),
            11.036646399999999,
        );
    }
}
