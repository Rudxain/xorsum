#![cfg_attr(not(test), no_std)]
#![allow(unused_imports)]
pub mod utils;
#[allow(clippy::wildcard_imports)]
/*pub*/
use crate::utils::hasher;

struct Hasher;

impl Hasher {
	// for streaming support.
	fn update(&self) {}
	// wouldn't an iterator be more elegant?
}
