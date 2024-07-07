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
pub trait Foo: Debug + Add {}

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

- `Clone`
- `Neg`, `Not`
- `Add`, `Sub`, `Mul`, `Div`, `Rem`
- `BitAnd`, `BitOr`, `BitXor`, `Shl`, `Shr`
- `AddAssign`, `SubAssign`, `MulAssign`, `DivAssign`, `RemAssign`
- `BitAndAssign`, `BitOrAssign`, `BitXorAssign`, `ShlAssign`, `ShrAssign`
- `PartialEq`, `Eq`, `PartialOrd`, `Ord`

More std traits and custom traits may be supported in the future.

## Methods

Note: This part is not yet complete.

In Rust, associate functions can be divided into two types: methods and non-methods, depending on whether their first parameter is named `self`.

```rust
trait Foo: Sized {
  // These are methods.
  fn method_1(&self);
  fn method_2(self) -> Self;
  // These are non-methods.
  fn method_3() -> Option<Vec<Self>>;
  fn method_4(this: &mut Self, that: Self);
}
```

This crate supports both methods and non-methods, but they are treated differently. Methods and non-methods are separated into two traits, namely *instance* and *constructor*. They are both object-safe.

```rust
trait FooInstance {
  fn method_1(&self);
  fn method_2(self: Box<Self>) -> Box<dyn FooInstance>;
}

trait FooConstructor {
  fn method_3(&self) -> Option<Vec<Box<dyn FooInstance>>>;
  fn method_4(&self, this: &mut dyn FooInstance, that: Box<dyn FooInstance>);
}
```

The original `Foo` trait (which may or may not be object-safe) can be wrapped by `Instance` and `Constructor` types in order to be used as an instance or constructor.

```rust ignore
impl FooInstance for ::dyn_std::Instance<Foo> {}
impl FooConstructor for ::dyn_std::Constructor<Foo> {}
```

If you are developing a library, you may write code like this:

```rust ignore
use std::collections::HashMap;
use dyn_std::Constructor;

struct Registry(HashMap<String, Box<dyn FooConstructor>>);

impl Registry {
    fn register<T: Foo>(&mut self, name: impl Into<String>) {
        self.0.insert(name.into(), Box::new(Constructor::<T>));
    }
}
```

And the user of your library may write code like this:

```rust ignore
let mut registry = Registry(HashMap::new());
registry.register::<CustomFooImpl>("custom");
```

## Specification

A trait should satisfy the all following requirements to be transformed into an object-safe trait by the `#[dyn_trait]` attribute.

### Supertraits

All supertraits must be:

- either object-safe,
- or one of the [above](#supported-traits) std traits.

`Sized` will be automatically removed from the supertraits for instance and constructor traits, but retained for the original trait.

### Associated Constants

It must not have any associated constants.

### Associated Types

It must not have any associated types with generics.

### Associated Functions

#### Receiver Types

Receiver types are types that can be used as the receiver of a method call. The following types can be used as receiver types:

- `Self`
- `&Self`
- `&mut Self`
- `Box<Self>`

Note that `Rc<Self>`, `Arc<Self>`, and `Pin<P>` (where `P` is receiver) are not currently supported.

#### Parameters Types

All the parameters must be of the following types:

- types that does not contain `Self`,
- receiver types,
- tuples of valid parameter types,
- monads such as `Option<T>`, `Result<T, E>`, `Vec<T>` (where `T`, `E` are valid parameter types),
- `&dyn`, `&mut dyn`, `Box<dyn>` of `Fn`, `FnMut`, `FnOnce` (where all the parameters are valid non-referencing parameter types).

The following types are valid parameter types:

```rust ignore
(Self, &Self, Box<Self>)
```
```rust ignore
HashMap<i32, HashMap<i32, &Self>>
```
```rust ignore
Result<Vec<Box<dyn Fn(Self) -> Self>>, Option<Self>>
```

The following types are **NOT** valid parameter types:

```rust ignore
&[Self]
```
```rust ignore
Pin<Arc<Self>>
```
```rust ignore
&dyn Fn(&mut Self)
```

#### Return Types

The return type must be a non-referencing parameter type.

#### Generics

Not have any type parameters (although lifetime parameters are allowed).

`impl Trait` is considered as a type parameter, thus it is not allowed.

## Credits

The crate is inspired by the following crates:

- [as-any](https://github.com/fogti/as-any)
- [dyn-clone](https://github.com/dtolnay/dyn-clone)
- [partial_eq_dyn](https://github.com/StamesJames/partial_eq_dyn)

## License

MIT.
