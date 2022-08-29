use std::io::{BufRead, BufReader, Read};

fn ceil_div(n: usize, d: usize) -> usize {
	match (n / d, n % d) {
		(q, 0) => q,
		(q, _) => q + 1,
	}
}

///round `n` to +Infinity, to nearest multiple of `d`
fn ceil_to_multiple(n: usize, d: usize) -> usize {
	ceil_div(n, d) * d
}

//why isn't this in `core`?
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

///a crappy non-seedable PRNG
fn rng(m: usize) -> usize {
	std::time::SystemTime::now()
		.duration_since(std::time::UNIX_EPOCH)
		.unwrap()
		.as_millis() as usize
		% m
}

///get a random string from a string-array
pub fn rand_pick<'a>(arr: &'a [&str]) -> &'a str {
	arr[rng(arr.len())]
}

fn xor_hasher(bytes: &[u8], key: &mut [u8]) {
	for chunk in bytes.chunks(key.len()) {
		chunk.iter().zip(&mut *key).for_each(|(&b, k)| *k ^= b);
	}
}

pub fn read_stream(stream: impl Read, sbox: &mut [u8]) -> std::io::Result<()> {
	let l = sbox.len();
	if l == 0 {
		return Ok(());
	} //avoid div by 0
  /*
  While Stdin just uses a BufReader internally, it uses the default length.
  The problem is that the sbox length is controllable by the user,
  so there's no guarantee that the buf length will be a multiple of sbox.len,
  which means that we could end up overusing the start of sbox
  instead of spreading the bytes as evenly as possible.

  To handle the length issue, we'll just create our own BufReader with a controlled
  length. It will result in double-buffering stdin, but we don't know a better way than that.
  */
	const DEFAULT_BUF_LEN: usize = 1 << 16;
	let buf_len = if DEFAULT_BUF_LEN > l {
		ceil_to_multiple(DEFAULT_BUF_LEN, l)
	} else {
		l
	};

	//We create the buffer in here so that the stdin read can be buffered in a way
	//because it lets us control the length of the buffer.
	let mut reader = BufReader::with_capacity(buf_len, stream);
	loop {
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
