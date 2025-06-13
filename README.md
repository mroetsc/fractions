# fractions

[![Crates.io](https://img.shields.io/crates/v/fractions.svg)](https://crates.io/crates/fractions)
[![Documentation](https://docs.rs/fractions/badge.svg)](https://docs.rs/fractions)
[![License](https://img.shields.io/crates/l/fractions.svg)](https://github.com/mroetsc/fractions#license)

A simple, lightweight Rust crate for fraction arithmetic with automatic reduction to lowest terms.

## Features

- Basic arithmetic operations (+, -, *, /)
- Automatic reduction to lowest terms
- Comparison operators
- Conversion to/from integers
- Proper error handling
- Zero-cost abstractions with no dependencies

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
fractions = "0.1"
```

## Examples

```rs
use fractions::Fraction;

fn main() -> Result<(), fractions::FractionError> {
    // Create fractions
    let half = Fraction::new(1, 2)?;
    let third = Fraction::new(1, 3)?;

    // Basic arithmetic
    let sum = half + third;               // 5/6
    let difference = half - third;        // 1/6
    let product = half * third;           // 1/6
    let quotient = (half / third)?;       // 3/2

    // Comparisons
    assert!(half > third);
    assert_eq!(half, Fraction::new(2, 4)?);

    // Convert to f64
    let value = half.to_f64();           // 0.5

    // From integer
    let five = Fraction::from(5);        // 5/1

    Ok(())
}
```

Run the calculator example:

```bash
cargo run --example calculator
```

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
