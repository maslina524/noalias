# Noalias

[![crates.io](https://img.shields.io/crates/v/noalias.svg)](https://crates.io/crates/noalias)
[![docs.rs](https://docs.rs/noalias/badge.svg)](https://docs.rs/noalias)
[![license](https://img.shields.io/crates/l/noalias.svg)](https://crates.io/crates/noalias)
[![no_std](https://img.shields.io/badge/no_std-supported-blue.svg)](https://docs.rs/noalias)

Macro crate for auto-generating type-safe wrappers fo types

It is useful when a plain type alias is not enough because type aliases do not create distinct types. The macro creates zero-cost `#[repr(transparent)]` wrappers with convenient conversions and optional trait implementations.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
noalias = "1.0.0"
```

## Usage

```rust
use noalias::noalias;

noalias! {
    pub type CChar = i8 [Display, Debug, PartialEq, Eq];
    pub type CInt = i32;
    pub type CShort = i16 [PartialEq];
}

fn main() {
    let ch = CChar::new(65);

    assert_eq!(ch, 65);
    assert_eq!(format!("{ch}"), "65");

    let raw: i8 = ch.into_inner();
    assert_eq!(raw, 65);
}
```