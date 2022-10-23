#![deny(clippy::unwrap_used)]
#![forbid(
	clippy::exit,
	clippy::mem_forget,
	clippy::large_include_file,
	clippy::fn_to_numeric_cast_any,
	clippy::cast_precision_loss,
	clippy::float_arithmetic,
	clippy::excessive_precision,
	clippy::lossy_float_literal,
	clippy::float_cmp,
	clippy::float_cmp_const
)]

///Calculates the quotient of `n` and `d`, rounding towards +infinity.
///
///`n` is the numerator/dividend
///
///`d` is the denominator/divisor
///
///# Panics
///If `d` is 0 (maybe also when overflow).
///
///# Examples
///Basic usage:
///```
///let a = 8;
///let b = 3;
///
///assert_eq!(div_ceil(a, b), 3);
///assert_eq!(div_ceil(b, a), 1);
///```
#[allow(clippy::inline_always)]
#[inline(always)]
const fn div_ceil(n: usize, d: usize) -> usize {
	match (n / d, n % d) {
		(q, 0) => q,
		(q, _) => q + 1,
	}
}

///Rounds `n` to nearest multiple of `d` (biased to +infinity)
///
///# Examples
///Basic usage:
///```
///let a = 8;
///let b = 3;
///
///assert_eq!(next_multiple(a, b), 9);
///assert_eq!(next_multiple(b, a), 8);
///```
#[inline]
const fn next_multiple(n: usize, d: usize) -> usize {
	match d {
		0 => d,
		_ => div_ceil(n, d) * d,
	}
}

//why isn't this in `std`?
///returns lowercase hex-encoded expansion of a byte-vector
pub fn u8vec_to_hex_outplace(vector: &Vec<u8>) -> String {
	use std::fmt::Write as _;

	let mut hex = String::with_capacity(vector.len() * 2);
	for byte in vector {
		let _ = write!(hex, "{:02x}", byte);
	}
	hex
}

//should we use this instead?
///convert a byte-vector to its hex-encoded expansion (lowercase)
pub fn u8vec_to_hex_inplace(mut vector: Vec<u8>) -> String {
	const TABLE: &[u8; 0x10] = b"0123456789abcdef";
	let len = vector.len();
	vector.resize(len * 2, 0);
	if len > 0 {
		let mut i = len;
		loop {
			i -= 1;
			//set 2nd target byte to LSBs from source byte
			vector[i * 2 + 1] = TABLE[(vector[i] & 0xf) as usize];
			//set 1st target byte to MSBs from source byte
			vector[i * 2] = TABLE[(vector[i] >> 4) as usize];
			if i == 0 {
				break;
			}
		}
	}
	#[allow(clippy::unwrap_used)]//reason = "it's guaranteed to succeed"
	String::from_utf8(vector).unwrap()
}

///digests a byte-array into an `sbox` in-place.
///
///`bytes` is the data to be hashed.
///
///`sbox` is a reference to a state-box in which the hash result is XOR-ed into.
fn xor_hasher(bytes: &[u8], sbox: &mut [u8]) {
	for chunk in bytes.chunks(sbox.len()) {
		chunk.iter().zip(&mut *sbox).for_each(|(&b, k)| *k ^= b);
	}
}

///`xor_hasher` wrapper that takes an arbitrary `stream`
pub fn stream_processor(stream: impl std::io::Read, sbox: &mut [u8]) -> std::io::Result<()> {
	let len = sbox.len();
	if len == 0 {
		return Ok(());
	}
	/*
	While `Stdin` just uses a `BufReader` internally, it uses the default length.
	The problem is that the buf length isn't guaranteed to be a multiple of `sbox.len()`,
	which means that we can get a wrong hash, caused by over-using the lower indices.

	To handle this, we'll create our own `BufReader` with a controlled
	length. It will result in double-buffering stdin, but we don't know a better way than that (yet).
	*/
	let buf_len = {
		///best buffer size for most systems
		const DEFAULT_BUF_LEN: usize = 0x10000;
		if DEFAULT_BUF_LEN > len {
			next_multiple(DEFAULT_BUF_LEN, len)
		} else {
			len
		}
	};

	let mut reader = std::io::BufReader::with_capacity(buf_len, stream);
	loop {
		use std::io::BufRead as _;

		let read_buf = reader.fill_buf()?;
		let read_len = read_buf.len();
		if read_len == 0 {
			break;
		}

		xor_hasher(read_buf, sbox);
		reader.consume(read_len);
	}

	Ok(())
}

#[cfg(test)]
mod tests {
	#[allow(clippy::wildcard_imports)]
	use crate::module::*;

	#[test]
	fn table_cmp() {
		assert_eq!(u8vec_to_hex_inplace(vec![1; 1]), u8vec_to_hex_outplace(&vec![1; 1]));
	}
}
