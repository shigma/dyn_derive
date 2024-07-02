//! Dynamic (object-safe) version of std traits.
//! 
//! See: [dyn_derive](https://crates.io/crates/dyn_derive)

pub mod any;
pub mod clone;
pub mod cmp;
pub mod fmt;
pub mod inst;
pub mod map;
pub mod ops;

pub use any::*;
pub use inst::*;
