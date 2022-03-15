#![doc=include_str!("../README.md")]

mod entry;
mod macros;
#[cfg(test)]
mod tests;
mod trait_maps;
mod type_indexed;
mod typemap_trait;

pub use entry::*;
pub use trait_maps::*;
pub use typemap_trait::*;
