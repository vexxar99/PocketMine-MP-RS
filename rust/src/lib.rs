//! Native implementations of PocketMine-MP hot paths, loaded into PHP as the `pmmpnative` extension.

pub mod overworld;
pub mod palette;
pub mod simplex;
pub mod terrain;

#[cfg(not(test))]
mod php;
