/*!
Convert values to color simply and securely.

## Installation

Add the dependency in the `Cargo.toml` file:

```toml
[dependencies]
coloring = "0.1.0"
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
}
```
*/

mod coloring;

pub use coloring::*;
