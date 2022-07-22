//I don't want to pollute the global scope, so I'll use `use` sparingly
use clap::{ArgGroup, Parser};
use std::{
	io::{stdin, stdout, Read, Write},
	path::{Path, PathBuf},
};
use xorsum::*;

const NAME: &str = "xorsum";
const DEFAULT_SIZE: usize = 8;

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
	#[clap(short, long, default_value_t = DEFAULT_SIZE, value_parser)]
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

fn main() -> std::io::Result<()> {
	let cli = Cli::parse();

	if (cli.full && cli.brief) || (cli.lower && cli.upper) || (cli.hex && cli.raw) {
		unreachable!()
	}

	if cli.hell {
		println!("{}", HELL_MSG[rng(HELL_MSG.len())]);
		return Ok(());
	}
	if cli.heaven {
		println!("{}", HEAVEN_MSG[rng(HEAVEN_MSG.len())]);
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

	if cli.file.len() == 0 {
		xor_hasher(stdin().bytes(), &mut sbox);
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
					xor_hasher(stdin().bytes(), &mut sbox)
				} else {
					xor_hasher(
						std::io::BufReader::new(std::fs::File::open(&path)?).bytes(),
						&mut sbox,
					)
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
