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

/// default hash/digest/output length/size in bytes
pub const DEFAULT_LEN: usize = 8;
/// best buffer size for most systems
const DEFAULT_BUF_LEN: usize = 0x10000;

/// Calculates the quotient of `n` and `d`, rounding towards +infinity.
///
/// `n`: numerator/dividend
///
/// `d`: denominator/divisor
///
/// # Panics
/// If `d` is 0 (maybe also when overflow).
///
/// # Examples
/// Basic usage:
/// ```
/// let a = 8;
/// let b = 3;
///
/// assert_eq!(div_ceil(a, b), 3);
/// assert_eq!(div_ceil(b, a), 1);
/// ```
#[allow(clippy::inline_always)]
#[inline(always)]
#[must_use]
const fn div_ceil(n: usize, d: usize) -> usize {
	n / d + (if n % d == 0 { 0 } else { 1 })
}

/// Rounds `n` to nearest multiple of `d` (biased to +infinity)
///
/// # Examples
/// Basic usage:
/// ```
/// let a = 8;
/// let b = 3;
///
/// assert_eq!(next_multiple(a, b), 9);
/// assert_eq!(next_multiple(b, a), 8);
/// ```
#[inline]
#[must_use]
const fn next_multiple(n: usize, d: usize) -> usize {
	match d {
		0 => d,
		_ => div_ceil(n, d) * d,
	}
}

// why isn't this in `std`?
/// returns lowercase hex-encoded expansion of a byte-vector
pub fn u8vec_to_hex_outplace(v: &Vec<u8>) -> String {
	use std::fmt::Write as _;

	let mut hex = String::with_capacity(v.len() * 2);
	for byte in v {
		let _ = write!(hex, "{:02x}", byte);
	}
	hex
}

/// convert a byte-vector to its hex-encoded expansion (lowercase)
pub fn u8vec_to_hex_inplace(mut v: Vec<u8>) -> String {
	const TABLE: &[u8; 0x10] = b"0123456789abcdef";
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
	#[allow(clippy::unwrap_used)]
	String::from_utf8(v).unwrap()
}

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

#[cfg(test)]
#[test]
fn test_hasher() {
	assert_eq!(hasher(&[0], vec![0; DEFAULT_LEN]), vec![0; DEFAULT_LEN]);
}

/// `hasher` wrapper
pub fn stream_processor(stream: impl std::io::Read, mut sbox: Vec<u8>) -> std::io::Result<Vec<u8>> {
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
		next_multiple(DEFAULT_BUF_LEN, len)
	} else {
		len
	};

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
	#[allow(clippy::cast_possible_truncation)] // reason = "`i as u8` doesn't truncate"
	fn hex_cmp() {
		const L: usize = 0x100;
		let mut v: Vec<u8> = vec![0; L];
		for (i, b) in v.iter_mut().enumerate() {
			*b = i as u8;
		}
		assert_eq!(u8vec_to_hex_inplace(v.clone()), u8vec_to_hex_outplace(&v));
	}
}
