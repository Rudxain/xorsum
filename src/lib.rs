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
	let l = key.len();
	if l > 0 {
		for (i, b) in bytes.enumerate() {
			key[i % l] ^= b.unwrap();
		}
	}
}
