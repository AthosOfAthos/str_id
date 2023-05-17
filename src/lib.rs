#![no_std]

mod name;
pub use name::Name;

#[cfg(feature = "serde")]
mod serialize;

