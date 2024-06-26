# Enum Rotate

![Version](https://img.shields.io/badge/Version-0.1.0-red.svg)
![Minimum Rust version: 1.36](https://img.shields.io/badge/Minimum%20Rust%20Version-1.58.0-brightgreen.svg)

Treat your enums as iterators.

This crate provides the `EnumRotate` trait along with the accompanying derive macro allowing you to

- Get the next and previous variant for any variant of an enum
- Iterate over variants of an enum in a predefined (and customizable) order
- Iterate over variants of an enum starting from a particular variant

## Usage

```rust
use enum_rotate::EnumRotate;
use Enum::*;

#[derive(EnumRotate, PartialEq)]
enum Enum { A, B, C }

fn main() {
    assert_eq!(A.next(), B);
    assert_eq!(A.prev(), C);

    assert_eq!(
        Enum::iter().collect::<Vec<_>>(),
        vec![A, B, C],
    );
    
    assert_eq!(
        C.iter_from().collect::<Vec<_>>(),
        vec![C, A, B],
    );
}
```

It is also possible to specify a custom *iteration order* for the enum variants.

```rust
use enum_rotate::EnumRotate;
use Enum::*;

#[derive(EnumRotate, PartialEq)]
#[iteration_order(B, A, C)]
enum Enum { A, B, C }

fn main() {
    assert_eq!(
        Enum::iter().collect::<Vec<_>>(),
        vec![B, A, C],
    );
}
```
