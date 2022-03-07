# technical-analysis-rs

technical_analysis is a cool **unfinished** Rust library that allows you to use conveniently technical indicators.
Indicators imlement the Indicator trait, which allows you to call the `next(data_item: f64) -> Option<Self::Output>` method.
Use existing indicators like so:

## Example

```rust
use technical_analysis::{Indicator, indicators::SMA};

let mut sma = SMA::factory().with_window_size(5).build().unwrap();
assert_eq!(sma.next(10.0), 10.0);
assert_eq!(sma.next(20.0), 15.0);
assert_eq!(sma.next(15.0), 15.0);
assert_eq!(sma.next(10.0), 13.75);
assert_eq!(sma.next(10.0), 13.0);
assert_eq!(sma.next(10.0), 13.0);
assert_eq!(sma.next(12.5), 11.5);
```

Or create a custom one:

```rust
use technical_analysis::{Indicator};

struct MyIndicator {}

impl Indicator<f64> for MyIndicator {
     type Output = f64;

     fn next(&mut self, next: f64) -> Self::Output {
         next
     }
 }

 let mut my_indicator = MyIndicator{};
 assert_eq!(my_indicator.next(10.0), 10.0);
```

## Todo

- Documentation
- Add method checked_next to indicators, which input value checks
- Add VPT, ADX, Aroon, Stochastic Indicator, Stochastic RSI, Volume RSI, and more indicators
