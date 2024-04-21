#![no_std]
/*pub use crate::module::hasher;*/

struct Hasher;

impl Hasher {
	// for streaming support.
	fn update(&self) {}
	// wouldn't an iterator be more elegant?
}
