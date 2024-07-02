//! Dynamic (object-safe) version of std traits.
//! 
//! See: [dyn_derive](https://crates.io/crates/dyn_derive)

mod core;

pub mod any;
pub mod inst;
pub mod map;

pub use any::*;
pub use core::*;
pub use inst::*;
