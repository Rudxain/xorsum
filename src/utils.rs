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

// https://github.com/rust-lang/rust/issues/53964
#[allow(unused_extern_crates)]
extern crate std;
use std::{string::String, vec::Vec};
use xorsum::hasher;

/// default hash/digest/output length/size in bytes
pub const DEFAULT_LEN: usize = 8;
/// best buffer size for most systems
pub const DEFAULT_BUF_LEN: usize = 0x10000;

// why isn't this in `std`?
/// returns lowercase hex-encoded expansion of its arg
pub fn to_hex_clone(v: &[u8]) -> String {
	use std::fmt::Write as _;

	let mut hex = String::with_capacity(v.len() * 2);
	for byte in v {
		let _ = write!(hex, "{byte:02x}");
	}
	hex
}

/// convert arg to its lowercase hex-encoded expansion
pub fn to_hex_inplace(mut v: Vec<u8>) -> String {
	const TABLE: [u8; 0x10] = *b"0123456789abcdef";
	let len = v.len();
	v.resize(len * 2, 0);
	if len > 0 {
		let mut i = len;
		loop {
			i -= 1;
			// set 2nd target byte to LSBs from source byte
			v[i * 2 + 1] = TABLE[(v[i] & 0xf) as usize];
			// set 1st target byte to MSBs from source byte
			v[i * 2] = TABLE[(v[i] >> 4) as usize];
			if i == 0 {
				break;
			}
		}
	}
	match String::from_utf8(v) {
		Ok(s) => s,
		_ => unreachable!("String must be valid UTF-8"),
	}
}

/// `hasher` wrapper
pub fn stream_processor(
	stream: impl std::io::Read,
	mut sbox: Vec<u8>,
) -> std::io::Result<Vec<u8>> {
	let len = sbox.len();
	if len == 0 {
		return Ok(sbox);
	};
	/*
	While `Stdin` just uses a `BufReader` internally, it uses the default length.
	The problem is that the buf-len isn't guaranteed to be a multiple of `sbox.len()`,
	which means that we can get a wrong hash, caused by over-using the lower indices.

	To handle this, we'll create our own `BufReader` with a controlled
	length. It'll result in double-buffering stdin, but we don't know a better way than that (yet).
	*/
	let buf_len = if DEFAULT_BUF_LEN > len {
		DEFAULT_BUF_LEN.next_multiple_of(len)
	} else {
		len
	};
	/*
	What we should be doing instead of double-buffer,
	is to define a hasher that can be started at a non-zero index.
	That way, we can control exactly where the XORed bytes are placed in the digest.
	*/
	let mut reader = std::io::BufReader::with_capacity(buf_len, stream);
	loop {
		use std::io::BufRead as _;

		let read_buf = reader.fill_buf()?;
		let read_len = read_buf.len();
		if read_len == 0 {
			break;
		}

		hasher(read_buf, &mut sbox);
		reader.consume(read_len);
	}
	Ok(sbox)
}

#[cfg(test)]
mod tests {
	#[allow(clippy::wildcard_imports)]
	use super::*;

	#[test]
	fn test_hasher() {
		let zero = [0; DEFAULT_LEN];
		let mut hash = zero;
		hasher(&[0], &mut hash);
		assert_eq!(hash, zero);
	}

	#[test]
	#[allow(clippy::cast_possible_truncation)]
	fn hex_cmp() {
		let mut a = [0_u8; u8::MAX as usize + 1];
		for (i, v) in a.iter_mut().enumerate() {
			*v = i as u8;
		}
		assert_eq!(to_hex_inplace(Vec::from(a)), to_hex_clone(&a));
	}
}
