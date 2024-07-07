# dyn_derive

Inherit and derive object-unsafe traits for dynamic Rust.

## Introduction

[Object safety](https://doc.rust-lang.org/reference/items/traits.html#object-safety) is a property of traits in Rust that determines whether the trait can be used as a trait object. However, the requirement for object safety is quite strict, limiting the expressiveness of the type system.

For example, you cannot simply write:

```rust compile_fail
// Clone is not object-safe
// PartialEq is not object-safe
pub trait Foo: Clone + PartialEq {
    // This method is not object-safe
    fn adjust(self) -> Self;
}

#[derive(Clone, PartialEq)]
pub struct Bar {
    meta: Box<dyn Foo>,         // The trait `Foo` cannot be made into an object.
}
```

This crate provides a procedural macro for transforming object-unsafe traits into object-safe ones:

```rust
use dyn_derive::*;

#[dyn_trait]
pub trait Foo: Clone + PartialEq {
    fn adjust(self) -> Self;
}

#[derive(Clone, PartialEq)]
pub struct Bar {
    meta: Box<dyn Foo>,         // Now it works!
}
```

Although there are still some limitations, this technique works smoothly in my scenarios.

## Supertraits

Supertraits is also required to be object-safe if the trait needs to be used as a trait object. However, many useful traits are not object-safe, such as `Clone` and `PartialEq`.

To tackle this issue, this crate transforms the supertraits into object-safe ones, so that they can be used as supertraits and be derived for your custom types.

### Basic Example

Below is a basic example of how to use this crate:

```rust
use std::fmt::Debug;
use dyn_derive::*;

#[dyn_trait]
pub trait Foo: Debug + Clone + PartialEq {
    fn answer(&self) -> i32 {
        42
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MetaImpl;

impl Foo for MetaImpl {}

#[derive(Debug, Clone, PartialEq)]
pub struct Bar {
    meta: Box<dyn Foo>,
}

fn main() {
    let foo1 = Bar { meta: Box::new(MetaImpl) };
    let foo2 = Bar { meta: Box::new(MetaImpl) };
    assert_eq!(foo1, foo2);
    let foo3 = foo1.clone();
    assert_eq!(foo3.meta.answer(), 42);
}
```

### Non-Derivable Traits

Taking the `Add` trait as an example:

```rust
use std::fmt::Debug;
use std::ops::Add;
use dyn_derive::*;

#[dyn_trait]
pub trait Foo: Sized + Debug + Add {}

#[derive(Debug)]
pub struct MetaImpl(String);

impl Foo for MetaImpl {}

impl Add for MetaImpl {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self(self.0 + &rhs.0)
    }
}

pub struct Bar {
    pub meta: Box<dyn Foo>,
}

impl Add for Bar {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            // `Box<dyn Foo>` can be added!
            meta: self.meta + rhs.meta,
        }
    }
}

fn main() {
    let foo1 = Bar { meta: Box::new(MetaImpl("114".into())) };
    let foo2 = Bar { meta: Box::new(MetaImpl("514".into())) };
    let foo3 = foo1 + foo2;
    println!("{:?}", foo3.meta);    // MetaImpl("114514")
}
```

### Supported Traits

The following std traits are supported:

- Clone
- Neg, Not
- Add, Sub, Mul, Div, Rem
- BitAnd, BitOr, BitXor, Shl, Shr
- AddAssign, SubAssign, MulAssign, DivAssign, RemAssign
- BitAndAssign, BitOrAssign, BitXorAssign, ShlAssign, ShrAssign
- PartialEq, Eq, PartialOrd, Ord

More std traits and custom traits may be supported in the future.

## Features

### `extra-cmp-impl`

There is a known issue with `PartialEq`: [rust-lang/rust#31740](https://github.com/rust-lang/rust/issues/31740). The crate provides two approaches to work around this issue:

- With feature `extra-cmp-impl`: the `dyn_trait` macro will implement extra `PartialEq<&Self>`.
- Without feature `extra-cmp-impl`: you can use `PartialEqFix` derive macro instead of `PartialEq`.

This feature is enabled by default.

## Credits

The crate is inspired by the following crates:

- [as-any](https://github.com/fogti/as-any)
- [dyn-clone](https://github.com/dtolnay/dyn-clone)
- [partial_eq_dyn](https://github.com/StamesJames/partial_eq_dyn)

## License

MIT.
