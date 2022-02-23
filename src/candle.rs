use anyhow::{anyhow, Error};

use crate::indicator::*;

#[derive(Clone, Debug)]
pub struct Candle {
    open: f64,
    close: f64,
    high: f64,
    low: f64,
    volume: u64,
}

impl Candle {
    pub fn new(open: f64, close: f64, high: f64, low: f64, volume: u64) -> Result<Candle, Error> {
        // Add cases for open > high, etc, boi
        if low <= high {
            Ok(Candle {
                open,
                close,
                high,
                low,
                volume,
            })
        } else {
            Err(anyhow!(
                "Low ({}) on candle greater than high ({})!",
                low,
                high
            ))
        }
    }
}

impl Open for Candle {
    fn open(&self) -> f64 {
        self.open
    }
}

impl Close for Candle {
    fn close(&self) -> f64 {
        self.close
    }
}

impl High for Candle {
    fn high(&self) -> f64 {
        self.high
    }
}

impl Low for Candle {
    fn low(&self) -> f64 {
        self.low
    }
}

impl Volume for Candle {
    fn volume(&self) -> u64 {
        self.volume
    }
}
