#![cfg_attr(not(test), no_std)]
mod utils;
use crate::utils::DEFAULT_BUF_LEN;

/// digests `inp` into `sbox` in-place.
pub fn hasher<'a, T>(inp: &'a [T], sbox: &mut [T])
where
	T: core::ops::BitXorAssign<&'a T>,
{
	let len = sbox.len();
	if len == 0 {
		return;
	};
	// faster than `% len` indexing, because of data-parallelism (and avoids div).
	// however, if `len` is too big, `chunk` will be allowed to be big too.
	for chunk in inp.chunks(len) {
		// this is correct,
		// because the last chunk doesn't need to be isometric
		chunk.iter().zip(&mut *sbox).for_each(|(i, s)| *s ^= i);
	}
}

/// digests `inp` into `sbox` in-place.
pub fn hasher_alt<'a, T>(inp: &'a [T], sbox: &mut [T])
where
	T: core::ops::BitXorAssign<&'a T>,
{
	if sbox.is_empty() {
		return;
	};
	let mut i: usize = 0;
	// do we really need chunked iter?
	for chunk in inp.chunks(DEFAULT_BUF_LEN) {
		for b in chunk {
			sbox[i] ^= b;

			// rustc should easily optimize this
			//i = (i + 1) % sbox.len()
			i += 1;
			if i >= sbox.len() {
				i = 0;
			};
		}
	}
}

pub struct Hasher;
impl Hasher {
	// for streaming support.
	pub fn update(&self) {}
	// wouldn't an iterator be more elegant?
}
