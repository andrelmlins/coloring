# 🌈 Coloring

Convert values to color simply and securely.

- Convert in hexadecimal, rgb, hsl, cmyk, hsv and hwb formats.
- Case sensitive.
- Always returns the same result for a string (Pure function).

[![Crates.io](https://img.shields.io/crates/v/coloring)](https://crates.io/crates/coloring) &bull; [![Crates.io](https://img.shields.io/crates/l/coloring)](https://github.com/andrelmlins/coloring/blob/master/LICENSE) &bull; [![Build Status](https://travis-ci.com/andrelmlins/coloring.svg?branch=master)](https://travis-ci.com/andrelmlins/coloring) &bull; [![API](https://docs.rs/coloring/badge.svg)](https://docs.rs/coloring)

## Installation

Add the dependency in the `Cargo.toml` file:

```toml
[dependencies]
coloring = "0.3"
```

## Basic Use

```rust
extern crate coloring::Coloring;

fn main() {
  let coloring = Coloring::new("My String");

  println!("{}", coloring.to_hexadecimal());
  // #259f0c

  println!("{:?}", coloring.to_rgb());
  // [37, 159, 12]

  println!("{:?}", coloring.to_hsl());
  // [110.0, 86.0, 34.0]

  println!("{:?}", coloring.to_cmyk());
  // [77.0, 0.0, 92.0, 38.0]

  println!("{:?}", coloring.to_hsv());
  // [110.0, 86.0, 34.0]

  println!("{:?}", coloring.to_hwb());
  // [110.0, 5.0, 38.0]
}
```

## License

Coloring is open source software [licensed as MIT](https://github.com/andrelmlins/coloring/blob/master/LICENSE).
