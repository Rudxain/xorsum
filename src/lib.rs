//why isn't this in `std`?
pub fn bytevec_tohex(vector: &Vec<u8>, upper: bool) -> String {
	let mut hex = String::new();
	for byte in vector {
		hex += &(if upper {
			format!("{byte:02X}")
		} else {
			format!("{byte:02x}")
		})
	}
	hex
}

pub fn xor_hasher(
	bytes: &[u8],
	key: &mut [u8],
) {
	for chunk in bytes.chunks(key.len()) {
		chunk.iter()
			.zip(&mut *key)
			.for_each(|(&b, k)| *k ^= b);
	}
}

fn rng(m: usize) -> usize {
	std::time::SystemTime::now()
		.duration_since(std::time::UNIX_EPOCH)
		.unwrap()
		.as_millis() as usize
		% m
}

pub fn rand_pick<'a>(arr: &'a [&str]) -> &'a str {
	arr[rng(arr.len())]
}
