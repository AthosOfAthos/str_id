#![no_std]

mod name;
pub use name::Name;

mod long_name;
pub use long_name::LongName;

#[cfg(feature = "serde")]
mod serialize;

