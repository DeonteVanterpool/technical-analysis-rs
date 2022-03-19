use anyhow::{anyhow, Error};

use crate::indicator::*;

#[derive(Clone, Debug)]
pub struct Candle {
    pub open: f64,
    pub close: f64,
    pub high: f64,
    pub low: f64,
    pub volume: u64,
}

/// `Candle`s are the default data item. They may be used as an input to any indicator, as they
/// implement `Open`, `Close`, `High`, `Low`, and `Volume`
impl Candle {
    pub fn new(open: f64, close: f64, high: f64, low: f64, volume: u64) -> Result<Candle, Error> {
        if [open, close, low].into_iter().all(|x| x <= high) && [open, close].into_iter().all(|x| x >= low) {
            Ok(Candle {
                open,
                close,
                high,
                low,
                volume,
            })
        } else {
            Err(anyhow!(
                "Unclean candle! Open: {}, Close: {}, High: {}, Low: {}, Volume: {}",
                open,
                close,
                low,
                high,
                volume,
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
