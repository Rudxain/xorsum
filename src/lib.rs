#![no_std]

fn hash<'a, T>(inp: &'a [T], digest: &mut [T])
where
	T: core::ops::BitXorAssign<&'a T>,
{
	for (i, v) in inp.iter().enumerate() {
		digest[i % digest.len()] ^= v;
	}
}

struct Hasher;

impl Hasher {
	// for streaming support.
	fn update(&self) {}
	// wouldn't an iterator be more elegant?
}
