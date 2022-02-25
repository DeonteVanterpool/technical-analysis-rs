use crate::{indicators::SD, *};
use anyhow::Error;

pub struct BollingerBandFactory<T>
where
    T: indicators::MovingAverage<f64> + Clone,
{
    standard_deviation: f64,
    middle_band: T,
}

pub struct BollingerBand<T>
where
    T: indicators::MovingAverage<f64> + Clone,
{
    standard_deviation: f64,
    middle_band: T,
    standard_deviation_indicator: SD<T>,
}

#[derive(PartialEq, Debug)]
pub struct BollingerBandResult {
    pub upper: f64,
    pub middle: f64,
    pub lower: f64,
}

impl<T: indicators::MovingAverage<f64> + Clone> BollingerBandFactory<T> {
    pub fn new() -> BollingerBandFactory<indicators::SMA> {
        BollingerBandFactory {
            standard_deviation: 2.0,
            middle_band: indicators::SMA::factory()
                .with_window_size(20)
                .build()
                .unwrap(),
        }
    }

    pub fn with_moving_average<U: indicators::MovingAverage<f64> + Clone>(
        self,
        moving_average: U,
    ) -> BollingerBandFactory<U> {
        BollingerBandFactory {
            standard_deviation: self.standard_deviation,
            middle_band: moving_average,
        }
    }

    pub fn with_standard_deviation(mut self, standard_deviation: f64) -> Self {
        self.standard_deviation = standard_deviation;
        self
    }

    pub fn build(self) -> Result<BollingerBand<T>, Error> {
        Ok(BollingerBand {
            standard_deviation: self.standard_deviation,
            middle_band: self.middle_band.clone(),
            standard_deviation_indicator: SD::<T>::factory()
                .with_moving_average(self.middle_band)
                .build()
                .unwrap(),
        })
    }
}

impl<T: indicators::MovingAverage<f64> + Clone, U: Close> Indicator<U> for BollingerBand<T> {
    type Output = BollingerBandResult;
    fn next(&mut self, next: U) -> Self::Output {
        let deviation = self.standard_deviation_indicator.next(next.close());
        let middle = self.middle_band.next(next.close());
        BollingerBandResult {
            upper: middle + deviation * self.standard_deviation,
            middle,
            lower: middle - deviation * self.standard_deviation,
        }
    }
}

impl<T: indicators::MovingAverage<f64> + Clone, U: indicator::Close> indicators::Trend<U>
    for BollingerBand<T>
{
}

impl<T: indicators::MovingAverage<f64> + Clone> BollingerBand<T> {
    pub fn factory() -> BollingerBandFactory<indicators::SMA> {
        BollingerBandFactory::<T>::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        indicators::SMA,
        indicators::{BollingerBand, BollingerBandResult},
        Indicator,
    };

    #[test]
    fn test_build() {
        BollingerBand::<SMA>::factory()
            .with_moving_average(SMA::factory().build().unwrap())
            .with_standard_deviation(1.5)
            .build()
            .unwrap();
    }

    #[test]
    fn test_next() {
        let mut bollinger_band = BollingerBand::<SMA>::factory()
            .with_moving_average(SMA::factory().with_window_size(5).build().unwrap())
            .build()
            .unwrap();

        assert_eq!(
            bollinger_band.next(10.0),
            BollingerBandResult {
                upper: 10.0,
                middle: 10.0,
                lower: 10.0,
            }
        );

        assert_eq!(
            bollinger_band.next(20.0),
            BollingerBandResult {
                upper: 25.0,
                middle: 15.0,
                lower: 5.0,
            }
        );

        assert_eq!(
            bollinger_band.next(15.0),
            BollingerBandResult {
                upper: 23.164965809277263,
                middle: 15.0,
                lower: 6.835034190722739,
            }
        );

        assert_eq!(
            bollinger_band.next(10.0),
            BollingerBandResult {
                upper: 22.0415619758885,
                middle: 13.75,
                lower: 5.4584380241115,
            }
        );

        assert_eq!(
            bollinger_band.next(10.0),
            BollingerBandResult {
                upper: 21.0,
                middle: 13.0,
                lower: 5.0,
            }
        );

        assert_eq!(
            bollinger_band.next(10.0),
            BollingerBandResult {
                upper: 21.0,
                middle: 13.0,
                lower: 5.0,
            }
        );

        assert_eq!(
            bollinger_band.next(12.5),
            BollingerBandResult {
                upper: 15.5,
                middle: 11.5,
                lower: 7.5,
            }
        );
    }
}
