use crate::*;
use anyhow::Error;

pub struct TrFactory {}

#[derive(Clone, Debug)]
pub struct TR {
    prev_close: f64,
    is_new: bool,
}

impl TrFactory {
    pub fn new() -> Self {
        Self {}
    }

    pub fn build(self) -> Result<TR, Error> {
        Ok(TR {
            prev_close: 0.0,
            is_new: true,
        })
    }
}

impl<T: Close + High + Low> crate::Indicator<T> for TR {
    type Output = f64;
    fn next(&mut self, next: T) -> Self::Output {
        let tr = if self.is_new {
            self.is_new = false;
            next.high() - next.low()
        } else {
            (next.high().max(self.prev_close)) - (next.low().min(self.prev_close))
        };
        self.prev_close = next.close();
        tr
    }
}

impl<T: Close + High + Low> crate::indicators::Volatility<T> for TR {}

impl TR {
    pub fn factory() -> TrFactory {
        TrFactory::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::{indicators::TR, *};

    #[test]
    fn test_build() {
        TR::factory().build().unwrap();
    }

    #[test]
    fn test_next() {
        let mut tr = TR::factory().build().unwrap();

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
            tr.next(CHL {
                close: 8.0,
                high: 15.0,
                low: 5.0,
            }),
            10.0,
        );

        assert_eq!(
            tr.next(CHL {
                close: 15.0,
                high: 20.0,
                low: 7.0,
            }),
            13.0,
        );

        assert_eq!(
            tr.next(CHL {
                close: 22.0,
                high: 25.0,
                low: 15.0,
            }),
            10.0,
        );

        assert_eq!(
            tr.next(CHL {
                close: 25.0,
                high: 30.0,
                low: 14.0,
            }),
            16.0,
        );

        assert_eq!(
            tr.next(CHL {
                close: 20.0,
                high: 25.0,
                low: 15.9,
            }),
            9.1,
        );

        assert_eq!(
            tr.next(CHL {
                close: 17.0,
                high: 20.0,
                low: 10.0,
            }),
            10.0,
        );

        assert_eq!(
            tr.next(CHL {
                close: 10.0,
                high: 18.0,
                low: 5.0,
            }),
            13.0,
        );

        assert_eq!(
            tr.next(CHL {
                close: 10.0,
                high: 15.0,
                low: 5.0,
            }),
            10.0,
        );
    }
}
