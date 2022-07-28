//I don't want to pollute the global scope, so I'll use `use` sparingly
use clap::{ArgGroup, Parser};
use std::{
	io::{stdin, stdout, BufRead, BufReader, Read, Write},
	path::{Path, PathBuf},
};
use xorsum::*;

const NAME: &str = "xorsum";
const DEFAULT_LEN: usize = 8;
const DEFAULT_BUF_LEN: usize = 1 << 16;

const HELL_MSG: [&str; 4] = [
	"I can't go to hell. I'm all out of vacation days.",
	"Highway to Hell!",
	"RIP N' TEAR",
	"Son't eorry evrryone makez nistakes while typong",
];
const HEAVEN_MSG: [&str; 4] = [
	"Locked Out of Heaven!",
	"Stairway to Heaven",
	"[Heaven], are you WATCHING?",
	"The Holy C",
];

#[derive(Parser)]
#[clap(
	version,
	about = "Print XOR (64-bit) checksums",
	long_about = "If no FILES are given, or if FILE is \"-\", reads Standard Input",
	group(ArgGroup::new("name").args(&["full", "brief"])),
	group(ArgGroup::new("case").args(&["lower", "upper"])),
	group(ArgGroup::new("code").args(&["hex", "raw"])),
	group(ArgGroup::new("mode").args(&["std", "quirky"]))
)]
struct Cli {
	/// Digest size in bytes (prior to hex-encoding)
	#[clap(short, long, default_value_t = DEFAULT_LEN, value_parser)]
	length: usize,

	/// Revert --brief in quirky mode
	#[clap(short, long, action)]
	full: bool,
	/// Only print hash, no filenames
	#[clap(short, long, action)]
	brief: bool,

	/// lowercase hex (default)
	#[clap(short = 'a', long, action)]
	lower: bool,
	/// UPPERCASE hex
	#[clap(short = 'A', long = "UPPER", action)]
	upper: bool,

	/// Revert --raw in quirky mode
	#[clap(short = 'x', long, action)]
	hex: bool,
	/// Print raw bytes, not hex
	#[clap(short, long, action)]
	raw: bool,

	/// Standard GNU core-utils compliant mode (default)
	#[clap(long, action)]
	std: bool,
	/// Non-std compatibility-breaking enhanced functionality
	#[clap(long, action)]
	quirky: bool,

	#[clap(long, action)]
	hell: bool,
	#[clap(long, action)]
	heaven: bool,

	#[clap(long, action)]
	hello: bool,
	#[clap(long = "olé!", action)]
	olé: bool,

	/// Files to hash
	#[clap(value_parser)]
	file: Vec<PathBuf>,
}

fn read_stream(stream: impl Read, sbox: &mut [u8]) -> std::io::Result<()> {
	let l = sbox.len();
	if l == 0 {return Ok(())} //avoid div by 0
	/*
	While Stdin just uses a BufReader internally, it uses the default length.
	The problem is that the sbox length is controllable by the user,
	so there's no guarantee that the buf length will be a multiple of sbox.len,
	which means that we could end up overusing the start of sbox
	instead of spreading the bytes as evenly as possible.

	To handle the length issue, we'll just create our own BufReader with a controlled
	length. It will result in double-buffering stdin, but we don't know a better way than that.
	*/
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

fn main() -> std::io::Result<()> {
	let cli = Cli::parse();

	if (cli.full && cli.brief) || (cli.lower && cli.upper) || (cli.hex && cli.raw) {
		unreachable!()
	}

	if cli.hell {
		println!("{}", rand_pick(&HELL_MSG));
		return Ok(());
	}
	if cli.heaven {
		println!("{}", rand_pick(&HEAVEN_MSG));
		return Ok(());
	}

	if cli.hello {
		println!("world!");
		return Ok(());
	}
	if cli.olé {
		println!("¡Ostia tío! ¿Cómo has logrado escribir eso?");
		return Ok(());
	}

	//allocate once, reuse everywhere
	let mut sbox = vec![0; cli.length]; //state box, IV = 0

	if cli.file.is_empty() {
		read_stream(stdin().lock(), &mut sbox)?;
		if cli.raw {
			stdout().write_all(&sbox).unwrap()
		} else {
			println!(
				"{}{}",
				bytevec_tohex(&sbox, cli.upper),
				if cli.brief { "" } else { " -" }
			)
		}
	} else {
		for path in cli.file {
			let h = Path::new("-");
			if path.is_file() || path == h {
				if path == h {
					//JIC, avoid creating multiple BRs on the same stdin
					read_stream(stdin().lock(), &mut sbox)?;
				} else {
					read_stream(std::fs::File::open(&path)?, &mut sbox)?;
				}

				if cli.raw {
					stdout().write_all(&sbox).unwrap();
					println!("{}", if cli.brief { "" } else { " -" })
				} else {
					let hex = bytevec_tohex(&sbox, cli.upper);
					if cli.brief {
						println!("{hex}")
					} else {
						println!("{hex} {}", path.display())
					}
				}
			} else {
				std::io::stderr()
					.write_all(
						{
							format!(
								"{NAME}: {}: {}\n",
								path.display(),
								if path.is_dir() {
									"Is a directory"
								} else {
									"No such file or directory"
								}
							)
						}
						.as_bytes(),
					)
					.unwrap();
			}
			sbox.fill(0) //reset
		}
	}
	Ok(())
}
