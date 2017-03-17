#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(not(feature = "std"), feature(collections))]

#[cfg(any(feature = "std_serde", feature = "no_std_serde"))]
extern crate serde;

#[cfg(any(feature = "std_serde", feature = "no_std_serde"))]
#[macro_use]
extern crate serde_derive;

#[cfg(not(feature = "std"))]
#[macro_use]
extern crate collections;

#[cfg(feature = "std")]
mod core {
    pub use std::{ops, hash, fmt, cmp, mem, slice, iter, borrow};
}


pub mod flat_map;
pub use flat_map::Entry::*;
pub use flat_map::FlatMap;
