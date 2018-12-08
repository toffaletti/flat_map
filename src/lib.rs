#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(not(feature = "std"), feature(alloc))]

#[cfg(feature = "serde1")]
extern crate serde;

#[cfg(not(feature = "std"))]
#[macro_use]
pub extern crate alloc;

#[cfg(not(feature = "std"))]
mod std {
    pub use alloc::*;
    pub use core::{borrow, cmp, fmt, hash, iter, marker, mem, ops, slice};
}

pub mod flat_map;
pub use crate::flat_map::Entry::*;
pub use crate::flat_map::FlatMap;
