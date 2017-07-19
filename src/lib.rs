#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(not(feature = "std"), feature(alloc))]

#[cfg(feature = "serde1")]
extern crate serde;

#[cfg(feature = "serde1")]
#[macro_use]
extern crate serde_derive;

#[cfg(not(feature = "std"))]
#[macro_use]
pub extern crate alloc;

#[cfg(not(feature = "std"))]
mod std {
    pub use core::{ops, hash, fmt, cmp, mem, slice, iter, borrow};
    pub use alloc::*;
}


pub mod flat_map;
pub use flat_map::Entry::*;
pub use flat_map::FlatMap;
