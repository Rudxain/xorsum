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

use std::io;

/// best buffer capacity for most 64bit systems
pub const BUF_CAP: usize = 0x10000;

// why isn't this in `std`?
/// returns lowercase hex-encoded expansion of its arg
pub fn to_hex_clone(vec: &[u8]) -> String {
	use std::fmt::Write as _;

	let mut hex = String::with_capacity(vec.len() * 2);
	for b in vec {
		match write!(hex, "{b:02x}") {
			Ok(()) => (),
			_ => unreachable!("each hex-pair must be valid UTF-8"),
		};
	}
	hex
}

/// convert arg to its lowercase hex-encoded expansion
pub fn to_hex_inplace(mut v: Vec<u8>) -> String {
	const TABLE: [u8; 0x10] = *b"0123456789abcdef";

	let len = v.len();
	v.resize(len * 2, 0);

	for i in (0..len).rev() {
		// set 2nd target byte to LSBs from source byte
		v[i * 2 + 1] = TABLE[(v[i] & 0xf) as usize];
		// set 1st target byte to MSBs from source byte
		v[i * 2] = TABLE[(v[i] >> 4) as usize];
	}

	match String::from_utf8(v) {
		Ok(s) => s,
		_ => unreachable!("hex `String` must be valid UTF-8"),
	}
}

/// sums `stream` into `sbox` in-place.
/// if passing `stdin` as `stream`, it'll be double-buffered.
pub fn stream_digestor<T: io::Read>(stream: T, sbox: &mut [u8]) -> io::Result<()> {
	use io::BufRead as _;

	if sbox.is_empty() {
		return Ok(());
	};
	let mut i: usize = 0;

	// `main` should pre-alloc this.
	// this is inefficient.
	let mut reader = io::BufReader::with_capacity(BUF_CAP, stream);
	loop {
		let buf = reader.fill_buf()?;
		if buf.is_empty() {
			break;
		}
		for b in buf {
			sbox[i] ^= b;
			i += 1;
			if i >= sbox.len() {
				i = 0;
			}
		}
		// inlining it makes the compiler complain,
		// for some reason...
		let buf_len = buf.len();
		reader.consume(buf_len);
	}
	Ok(())
}

#[cfg(test)]
mod tests {
	use super::*;

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
