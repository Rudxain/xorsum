#![cfg_attr(not(test), no_std)]
pub mod utils;
pub use crate::utils::hasher;

pub struct Hasher;

impl Hasher {
	// for streaming support.
	fn update(&self) {}
	// wouldn't an iterator be more elegant?
}
