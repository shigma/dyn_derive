# dynex

Inherit and derive object-unsafe traits for dynamic rust.

## Introduction

[Object safety](https://doc.rust-lang.org/reference/items/traits.html#object-safety) is a property of traits in Rust that determines whether the trait can be used as a trait object. However, there are many useful traits that are not object-safe, such as `Clone` and `PartialEq`.

For example, you cannot simply write:

```rust
pub trait Meta: Debug + Clone + PartialEq {}

#[derive(Debug, Clone, PartialEq)]
pub struct Foo {
    meta: Box<dyn Meta>,        // The trait `Meta` cannot be made into an object.
}
```

This crate provides a procedural macro for deriving object-unsafe traits:

```rust
use dynex::*;

#[dyn_trait]
pub trait Meta: Debug + Clone + PartialEq {}

#[derive(Debug, Clone, PartialEqFix)]
pub struct Foo {
    meta: Box<dyn Meta>,        // Now it works!
}
```

Note: `PartialEqFix` has the exact same behavior as `PartialEq`, but it workarounds a strange behavior of the Rust compiler. For other traits, you can just derive the original trait name.

## Basic Example

Below is a basic example of how to use this crate:

```rust
use dynex::*;

#[dyn_trait]
pub trait Meta: Debug + Clone + PartialEq {
    fn answer(&self) -> i32 {
        42
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MetaImpl;

impl Meta for MetaImpl {}

#[derive(Debug, Clone, PartialEqFix)]
pub struct Foo {
    meta: Box<dyn Meta>,
}

fn main() {
    let foo1 = Foo { meta: Box::new(MetaImpl) };
    let foo2 = Foo { meta: Box::new(MetaImpl) };
    assert_eq!(foo1, foo2);
    let foo3 = foo1.clone();
    assert_eq!(foo3.meta.answer(), 42);
}
```

## Non-Derivable Traits

Taking the `Add` trait as an example:

```rust
use dynex::*;

#[dyn_trait]
pub trait Meta: Add {}

pub struct MetaImpl(String);

impl Meta for MetaImpl {}

impl Add for MetaImpl {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self(self.0 + rhs.0)
    }
}

pub struct Foo {
    meta: Box<dyn Meta>,
}

impl Add for Foo {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            // `Box<dyn Meta>` can be added!
            meta: self.meta + rhs.meta,
        }
    }
}

fn main() {
    let foo1 = Foo { meta: Box::new(MetaImpl("114".into())) };
    let foo2 = Foo { meta: Box::new(MetaImpl("514".into())) };
    let foo3 = foo1 + foo2;
    assert_eq!(foo3.meta.0, "114514");
}
```

## Credit

The crate is inspired by the following crates:

- [as-any](https://github.com/fogti/as-any)
- [dyn-clone](https://github.com/dtolnay/dyn-clone)
- [partial_eq_dyn](https://github.com/StamesJames/partial_eq_dyn)

## License

MIT.
