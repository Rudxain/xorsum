#![deny(clippy::print_stdout, clippy::print_stderr)]
#![cfg_attr(not(test), no_std)]

/// sums `inp` into `sbox` in-place.
pub fn digestor<'a, T, I>(inp: I, sbox: &mut [T])
where
	I: IntoIterator<Item = &'a T>,
	T: core::ops::BitXorAssign<&'a T> + 'a,
{
	if sbox.is_empty() {
		return;
	};
	let mut i: usize = 0;
	for b in inp {
		sbox[i] ^= b;

		// rustc should easily optimize this
		//i = (i + 1) % sbox.len()
		i += 1;
		// is this branch worse than `%`?
		if i >= sbox.len() {
			i = 0;
		};
	}
}

pub struct Hasher;
impl Hasher {
	// for streaming support.
	pub fn update(&self) {}
	// wouldn't an iterator be more elegant?
}
