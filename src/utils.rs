#![deny(clippy::unwrap_used, clippy::print_stdout, clippy::print_stderr)]
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
use crate::hasher;
use std::{io, string::String, vec::Vec};

/// best buffer size for most systems
pub const DEFAULT_BUF_LEN: usize = 0x10000;

// why isn't this in `std`?
/// returns lowercase hex-encoded expansion of its arg
pub fn to_hex_clone(vec: &[u8]) -> String {
	use std::fmt::Write as _;

	let mut hex = String::with_capacity(vec.len() * 2);
	for b in vec {
		let _ = write!(hex, "{b:02x}");
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
pub fn stream_processor(stream: impl io::Read, sbox: &mut [u8]) -> io::Result<()> {
	use io::BufRead as _;

	let len = sbox.len();
	if len == 0 {
		return Ok(());
	};
	/*
	`stream` `buf.len()` isn't guaranteed to be a multiple of `sbox.len()`,
	so might get a wrong hash, caused by over-using the lower indices.

	That's why we create our own `BufReader` with a controlled
	len. It'll result in double-buffering.
	*/
	let buf_len = if DEFAULT_BUF_LEN > len {
		DEFAULT_BUF_LEN.next_multiple_of(len)
	} else {
		len
	};
	// We should define a `hasher` that can start at non-zero index.
	// That way, we can control exactly where the XORed bytes are placed in the digest.
	let mut reader = io::BufReader::with_capacity(buf_len, stream);
	loop {
		let read_buf = reader.fill_buf()?;
		let read_len = read_buf.len();
		if read_len == 0 {
			break;
		}
		// if sbox isn't `Copy`, then why does this compile?
		// it should move into `hasher`, requiring a fresh reborrow:
		// `&mut *sbox`
		hasher(read_buf, sbox);
		reader.consume(read_len);
	}
	Ok(())
}

#[cfg(test)]
mod tests {
	#[allow(clippy::wildcard_imports)]
	use super::*;

	#[test]
	fn test_hasher() {
		let zero = [0; 4];
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
