//I don't want to pollute the global scope, so I'll use `use` sparingly
use clap::{ArgGroup, Parser};
use std::io::{stdin, stdout, Read, Write};
use xorsum::*;

const NAME: &str = "xorsum";
const DEFAULT_SIZE: usize = 8;

const NO_FILE_MSG: &str = "No such file or directory";
const DIR_MSG: &str = "Is a directory";

#[derive(Parser)]
#[clap(
	name = NAME, version,
	about = "Print XOR (64-bit) checksums",
	long_about = "If no FILES are given, or if FILE is \"-\", reads Standard Input",
	group(ArgGroup::new("name").args(&["full", "brief"])),
	group(ArgGroup::new("case").args(&["lower", "upper"])),
	group(ArgGroup::new("code").args(&["hex", "raw"])),
	group(ArgGroup::new("mode").args(&["std", "quirky"]))
)]
struct Cli {
	/// Hash size in bytes (prior to hex-encoding)
	#[clap(short, long, default_value_t = DEFAULT_SIZE, value_parser)]
	length: usize,

	/// Revert --brief in --quirky mode
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

	/// Revert --raw in --quirky mode
	#[clap(short = 'x', long, action)]
	hex: bool,
	/// Print raw bytes, not hex
	#[clap(short, long, action)]
	raw: bool,

	/// Standard GNU core-utils compliant mode
	#[clap(long, action)]
	std: bool,
	/// Non-std compatibility-breaking enhanced functionality
	#[clap(long, action)]
	quirky: bool,

	/// UT typo easter egg
	#[clap(long, action)]
	hell: bool,

	/// Files to hash
	#[clap(value_parser)]
	file: Vec<String>
}

fn main() -> std::io::Result<()> {
	let cli = Cli::parse();

	if (cli.full && cli.brief) ||
	(cli.lower && cli.upper) ||
	(cli.hex && cli.raw) {
		unreachable!()
	}

	if cli.hell {
		println!("I can't go to hell. I'm all out of vacation days.");
		return Ok(());
	}

	//allocate once, reuse everywhere
	let mut sbox = vec![0; cli.length]; //state box, IV = 0

	if cli.file.len() == 0 {
		sbox = xor_hasher(stdin().bytes(), sbox);
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
		//lol, I noticed this says "pain"
		for p_a in cli.file {
			let path = std::path::Path::new(&p_a);
			if path.is_file() || p_a == "-" {
				sbox = if p_a == "-" {
					xor_hasher(stdin().bytes(), sbox)
				} else {
					//I hope this uses a buffer to prevent RAM from exploding
					xor_hasher(std::fs::File::open(&p_a)?.bytes(), sbox)
				};

				if cli.raw {
					stdout().write_all(&sbox).unwrap();
					println!("{}", if cli.brief { "" } else { " -" })
				} else {
					let hex = bytevec_tohex(&sbox, cli.upper);
					if cli.brief {
						println!("{hex}")
					} else {
						println!("{hex} {p_a}")
					}
				}
			} else {
				std::io::stderr()
					.write_all(
						{
							format!(
								"{NAME}: {p_a}: {}\n",
								if path.is_dir() { DIR_MSG } else { NO_FILE_MSG }
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
