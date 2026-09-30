Creates distinct newtype wrappers around existing types.

A plain `type` alias does not create a new type, so two aliases of the same
underlying type are interchangeable. This macro instead generates a
`#[repr(transparent)]` newtype wrapper, so the resulting types are distinct
at compile time while remaining zero-cost at runtime.

## Syntax

```rust, norun
noalias! {
    $(#[$attr:meta])*
    pub type Name = InnerType [Trait1, Trait2, ...];
    ...
}
```

The `pub` keyword and the `[Trait1, ...]` list are both optional. Multiple
declarations can be placed in a single invocation. Trailing semicolons are
optional.

## Supported traits

The optional trait list accepts:

- `Display`
- `Debug`
- `PartialEq`
- `Eq`

Listing `Eq` requires also listing `PartialEq`. Any unknown trait name
triggers a compile-time error via [`compile_error!`].

## Generated items

For a declaration such as `pub type CChar = i8 [Display, PartialEq, Eq];`
the macro generates:

- `struct CChar(i8)` with `#[repr(transparent)]`
- `CChar::new(value: i8) -> CChar`
- `CChar::into_inner(self) -> i8` (a `const fn`)
- `impl AsRef<i8> for CChar`
- `impl From<i8> for CChar`
- `impl From<CChar> for i8`
- `impl Display for CChar`
- `impl PartialEq for CChar`, `impl PartialEq<i8> for CChar`,
  `impl PartialEq<CChar> for i8`
- `impl Eq for CChar`

## Examples

Basic usage without extra traits:

```rust, norun
use noalias::noalias;

noalias! {
    pub type CInt = i32;
    pub type CShort = i16;
}

let n = CInt::new(42);
assert_eq!(n.into_inner(), 42);
```

With trait implementations:

```rust, norun
use noalias::noalias;

noalias! {
    pub type CChar = i8 [Display, Debug, PartialEq, Eq];
}

let c = CChar::new(65);
assert_eq!(c, 65);
assert_eq!(format!("{c}"), "65");
```

Types are distinct even when the inner type is the same:

```rust, norun
use noalias::noalias;

noalias! {
    pub type CInt = i32;
    pub type CSize = i32;
}

fn takes_size(_: CSize) {}

let n = CInt::new(0);
// takes_size(n); // compile error: expected `CSize`, found `CInt`
```