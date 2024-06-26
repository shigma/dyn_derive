//! Inherit and derive object-unsafe traits for dynamic rust.
//! 
//! ## Introduction
//! 
//! [Object safety](https://doc.rust-lang.org/reference/items/traits.html#object-safety) is a property of traits in Rust that determines whether the trait can be used as a trait object. However, there are many useful traits that are not object-safe, such as `Clone` and `PartialEq`.
//! 
//! For example, you cannot simply write:
//! 
//! ```rust
//! pub trait Meta: Clone + PartialEq {}
//! 
//! #[derive(Clone, PartialEq)]
//! pub struct Foo {
//!     meta: Box<dyn Meta>,        // The trait `Meta` cannot be made into an object.
//! }
//! ```
//! 
//! This crate provides a procedural macro for deriving object-unsafe traits:
//! 
//! ```rust
//! use dynex::*;
//! 
//! #[dyn_trait]
//! pub trait Meta: Clone + PartialEq {}
//! 
//! #[derive(Clone, PartialEqFix)]
//! pub struct Foo {
//!     meta: Box<dyn Meta>,        // Now it works!
//! }
//! ```
//! 
//! Note: `PartialEqFix` has the exact same behavior as `PartialEq`, but it workarounds a strange behavior of the Rust compiler. For other traits, you can just derive the original trait name.
//! 
//! ## Basic Example
//! 
//! Below is a basic example of how to use this crate:
//! 
//! ```rust
//! use std::fmt::Debug;
//! use dynex::*;
//! 
//! #[dyn_trait]
//! pub trait Meta: Debug + Clone + PartialEq {
//!     fn answer(&self) -> i32 {
//!         42
//!     }
//! }
//! 
//! #[derive(Debug, Clone, PartialEq)]
//! pub struct MetaImpl;
//! 
//! impl Meta for MetaImpl {}
//! 
//! #[derive(Debug, Clone, PartialEqFix)]
//! pub struct Foo {
//!     meta: Box<dyn Meta>,
//! }
//! 
//! fn main() {
//!     let foo1 = Foo { meta: Box::new(MetaImpl) };
//!     let foo2 = Foo { meta: Box::new(MetaImpl) };
//!     assert_eq!(foo1, foo2);
//!     let foo3 = foo1.clone();
//!     assert_eq!(foo3.meta.answer(), 42);
//! }
//! ```
//! 
//! ## Non-Derivable Traits
//! 
//! Taking the `Add` trait as an example:
//! 
//! ```rust
//! use std::fmt::Debug;
//! use std::ops::Add;
//! use dynex::*;
//! 
//! #[dyn_trait]
//! pub trait Meta: Debug + Add {}
//! 
//! #[derive(Debug)]
//! pub struct MetaImpl(String);
//! 
//! impl Meta for MetaImpl {}
//! 
//! impl Add for MetaImpl {
//!     type Output = Self;
//! 
//!     fn add(self, rhs: Self) -> Self {
//!         Self(self.0 + &rhs.0)
//!     }
//! }
//! 
//! pub struct Foo {
//!     pub meta: Box<dyn Meta>,
//! }
//! 
//! impl Add for Foo {
//!     type Output = Self;
//! 
//!     fn add(self, rhs: Self) -> Self {
//!         Self {
//!             // `Box<dyn Meta>` can be added!
//!             meta: self.meta + rhs.meta,
//!         }
//!     }
//! }
//! 
//! fn main() {
//!     let foo1 = Foo { meta: Box::new(MetaImpl("114".into())) };
//!     let foo2 = Foo { meta: Box::new(MetaImpl("514".into())) };
//!     let foo3 = foo1 + foo2;
//!     println!("{:?}", foo3.meta);    // MetaImpl("114514")
//! }
//! ```

use proc_macro::TokenStream;

mod derive;
mod dyn_impl;

/// This derive macro has the exact same behavior as `PartialEq`,
/// but it workarounds a strange behavior of the Rust compiler.
/// 
/// For other traits, you can just derive the original trait name.
/// 
/// ## Example
/// 
/// ```rust
/// use dynex::*;
/// 
/// #[dyn_trait]
/// pub trait Meta: Clone + PartialEq {}
/// 
/// #[derive(Clone, PartialEqFix)]
/// pub struct Foo {
///     meta: Box<dyn Meta>,
/// }
/// ```
#[proc_macro_derive(PartialEqFix)]
pub fn derive_partial_eq(input: TokenStream) -> TokenStream {
    derive::partial_eq::derive(input.into()).into()
}

/// This is a procedural macro for deriving object-unsafe traits.
/// 
/// ## Example
/// 
/// `Clone` is not object-safe, but with this macro, you can still use `dyn Meta`:
/// 
/// ```rust
/// use dynex::*;
/// 
/// #[dyn_trait]
/// pub trait Meta: Clone {}
/// 
/// #[derive(Clone)]
/// pub struct Foo {
///     meta: Box<dyn Meta>,
/// }
/// ```
#[proc_macro_attribute]
pub fn dyn_trait(attrs: TokenStream, input: TokenStream) -> TokenStream {
    if !attrs.is_empty() {
        panic!("dyn_impl attribute does not accept any arguments")
    }
    dyn_impl::main(input.into()).into()
}
