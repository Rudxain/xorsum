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
#[inline]
const fn div_ceil(n: usize, d: usize) -> usize {
	match (n / d, n % d) {
		(q, 0) => q,
		(q, _) => q + 1,
	}
}

///Rounds `n` to nearest multiple of `d` (biased to +infinity)
///
///# Panics
///Never? I guess
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

//why isn't this in `core`?
///convert a byte-vector to its hex-encoded expansion
///
///`upper` makes the output uppercase/capitalized
pub fn u8vec_to_hex(vector: &Vec<u8>, upper: bool) -> String {
	use std::fmt::Write as _;

	let mut hex = String::with_capacity(vector.len() * 2);
	for byte in vector {
		let _ = if upper {
			write!(hex, "{:02X}", byte)
		} else {
			write!(hex, "{:02x}", byte)
		};
	}
	hex
}

///a crappy non-seedable PRNG based on sys time
///
///returns an int in the interval [`0`, `n`)
///
///returns `0` if the duration calculation panics
///# Panics
///if `n` is `0`
fn rng(n: usize) -> usize {
	use std::time::{SystemTime, UNIX_EPOCH};

	match SystemTime::now().duration_since(UNIX_EPOCH) {
		Ok(d) => d.as_nanos() as usize % n,
		Err(_) => 0,
	}
}

///get a pseudo-random value from an `Array`
pub fn rand_pick<T>(a: &[T]) -> &T {
	&a[rng(a.len())]
}

//to-do: avoid `&mut` by passing and returning `owned`
///digests a byte-array into a vector
///
///`bytes` is the data to be hashed
///
///`sbox` is a reference to a state-box in which the hash result is XOR-ed into
fn xor_hasher(bytes: &[u8], sbox: &mut [u8]) {
	for chunk in bytes.chunks(sbox.len()) {
		chunk.iter().zip(&mut *sbox).for_each(|(&b, k)| *k ^= b);
	}
}

///`xor_hasher` wrapper that takes an arbitrary `stream` to digest it into an `sbox`
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
