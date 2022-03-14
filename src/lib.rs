#![doc=include_str!("../README.md")]

mod base;
mod macros;
#[cfg(test)]
mod tests;
mod trait_maps;
mod typemap_trait;

pub use trait_maps::*;
pub use typemap_trait::*;
