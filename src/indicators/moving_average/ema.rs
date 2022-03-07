use crate::*;
use helpers::*;
use anyhow::{Error, anyhow};

pub struct EmaFactory {
    window_size: usize,
    smoothing: f64,
}

#[derive(Clone, Debug)]
pub struct EMA {
    window_size: usize,
    k: f64,
    prev_ema: f64,
    is_new: bool,
}

impl EmaFactory {
    pub fn new() -> Self {
        Self {
            window_size: 14,
            smoothing: 2.0,
        }
    }

    pub fn with_window_size(mut self, window_size: usize) -> Self {
        self.window_size = window_size;
        self
    }

    pub fn with_smoothing(mut self, smoothing: f64) -> Self {
        self.smoothing = smoothing;
        self
    }

    pub fn build(self) -> Result<EMA, Error> {
        check_window_size(self.window_size)?;
        if self.smoothing <= 0.0 || !self.smoothing.is_finite() {
            return Err(anyhow!("Smoothing value must be greater than zero and a real number. You used {}", self.smoothing));
        }
        Ok(EMA {
            window_size: self.window_size,
            k: self.smoothing / (self.window_size as f64 + 1.0),
            prev_ema: 0.0,
            is_new: true,
        })
    }
}

impl<T: Close> crate::Indicator<T> for EMA {
    type Output = f64;
    fn next(&mut self, next: T) -> Self::Output {
        if self.is_new {
            self.is_new = false;
            self.prev_ema = next.close();
        } else {
            self.prev_ema = self.k * next.close() + (1.0 - self.k) * self.prev_ema;
        }
        self.prev_ema
    }
}

impl<T: Close> crate::indicators::MovingAverage<T> for EMA {
    fn window_size(&self) -> usize {
        self.window_size
    }
}

impl EMA {
    pub fn factory() -> EmaFactory {
        EmaFactory::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::{indicators::EMA, Indicator};

    #[test]
    fn test_build() {
        EMA::factory()
            .with_window_size(5)
            .with_smoothing(2.5)
            .build()
            .unwrap();
    }

    #[test]
    fn test_next() {
        let mut ema = EMA::factory()
            .with_window_size(5)
            .with_smoothing(2.0)
            .build()
            .unwrap();

        assert_eq!(ema.next(10.0), 10.0);
        assert_eq!(ema.next(20.0), 13.333333333333334);
        assert_eq!(ema.next(15.0), 13.888888888888891);
        assert_eq!(ema.next(10.0), 12.592592592592595);
        assert_eq!(ema.next(10.0), 11.7283950617284);
        assert_eq!(ema.next(10.0), 11.1522633744856);
        assert_eq!(ema.next(12.5), 11.601508916323734);
    }
}
