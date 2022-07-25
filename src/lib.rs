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

pub fn xor_hasher<T: std::iter::Iterator<Item = Result<u8, std::io::Error>>>(
	bytes: T,
	key: &mut Vec<u8>,
) {
	(0..key.len())
		.cycle()
		.zip(bytes)
		.for_each(|(kb_idx, b)| {
			//Safety: We are using `kb_idx` to index into the key
			//and it is bound between 0 and key.len()
			*unsafe { key.get_unchecked_mut(kb_idx) } ^= b.as_ref().unwrap();
		})
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
