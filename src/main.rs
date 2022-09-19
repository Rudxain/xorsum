#![warn(
	clippy::cargo,
	clippy::pedantic,
	clippy::nursery,
	clippy::format_push_string
)]

use clap::{ArgGroup, Parser};
use std::{
	io::{stdin, stdout, Write},
	path,
};

mod module;
#[allow(clippy::wildcard_imports)]
use crate::module::*;

const NAME: &str = "xorsum";
const DEFAULT_LEN: usize = 8;

#[derive(Parser)]
#[clap(
	version,
	about = "Print XOR (64-bit) checksums",
	long_about = "If no FILES are given, or if FILE is \"-\", reads Standard Input",
	group(ArgGroup::new("name").args(&["full", "brief"])),
	group(ArgGroup::new("case").args(&["lower", "upper"])),
	group(ArgGroup::new("code").args(&["hex", "raw"])),
	group(ArgGroup::new("egg").args(&["hell", "heaven", "hello", "olé", "rick"])),
)]
struct Cli {
	///Digest size in bytes (prior to hex-encoding)
	#[clap(short, long, default_value_t = DEFAULT_LEN, value_parser)]
	length: usize,

	///Print hash + filename (default)
	#[clap(short, long, action)]
	full: bool,
	///Only print hash, no filenames
	#[clap(short, long, action)]
	brief: bool,

	///lowercase hex (default)
	#[clap(short = 'a', long, action)]
	lower: bool,
	///UPPERCASE hex
	#[clap(short = 'A', long = "UPPER", action)]
	upper: bool,

	///Print hexadecimal digest (default)
	#[clap(short = 'x', long, action)]
	hex: bool,
	///Print raw bytes, not hex
	#[clap(short = 'r', long, action)]
	raw: bool,

	#[clap(long, action)]
	hell: bool,
	#[clap(long, action)]
	heaven: bool,

	#[clap(long, action)]
	hello: bool,
	#[clap(long = "olé!", action)]
	olé: bool,

	#[clap(long, action)]
	rick: bool,

	///Files to hash
	#[clap(value_parser)]
	file: Vec<path::PathBuf>,
}

///easter-egg handler, lmao.
///
///if it detects any egg, returns `true`, otherwise `false`
///
///# Panics
///if `println!` panics, and/or if 2 or more eggs are selected
fn egg_cooker(c: &Cli) -> bool {
	{
		let mut b = false;
		for egg in [c.hell, c.heaven, c.hello, c.olé, c.rick] {
			if egg && b {
				unreachable!()
			}
			if egg {
				b = true;
			};
		}
	}

	if c.hell {
		println!(
			"{}",
			rand_pick(&[
				"I can't go to hell. I'm all out of vacation days.",
				"Highway to Hell!",
				"RIP N' TEAR",
				"Son't eorry evrryone makez nistakes while typong",
			])
		);
		return true;
	}
	if c.heaven {
		println!(
			"{}",
			rand_pick(&[
				"Locked Out of Heaven!",
				"Stairway to Heaven",
				"[Heaven], are you WATCHING?",
				"The Holy C",
			])
		);
		return true;
	}
	if c.hello {
		println!("world!");
		return true;
	}
	if c.olé {
		println!("¡Ostia tío! ¿Cómo has logrado escribir eso?");
		return true;
	}
	if c.rick {
		println!("We're no strangers to love...");
		return true;
	}
	false
}

fn main() -> std::io::Result<()> {
	let cli = Cli::parse();

	if (cli.full && cli.brief) || (cli.lower && cli.upper) || (cli.hex && cli.raw) {
		unreachable!()
	}

	//if an egg is activated, no work should be done
	if egg_cooker(&cli) {
		return Ok(());
	}

	//allocate once, reuse everywhere (remember to reset!)
	let mut sbox = vec![0; cli.length]; //state box, IV = 0

	if cli.file.is_empty() {
		stream_processor(stdin().lock(), &mut sbox)?;
		if cli.raw {
			stdout().lock().write_all(&sbox)?;
		} else {
			println!(
				"{}{}",
				u8vec_to_hex(&sbox, cli.upper),
				if cli.brief { "" } else { " -" }
			);
		}
	} else {
		let mut stdout_v = stdout();
		let mut lock = stdout_v.lock();

		for path in cli.file {
			if path == path::Path::new("-") {
				//avoid creating multiple BRs on the same stdin (just in case)
				stream_processor(stdin().lock(), &mut sbox)?;
			} else {
				match std::fs::File::open(&path) {
					Ok(f) => stream_processor(f, &mut sbox)?,
					Err(e) => {
						std::io::stderr().lock().write_all(
							{ format!("{}: {}: {}\n", NAME, path.display(), e) }.as_bytes(),
						)?;
						continue;
					}
				};
			}

			if cli.raw {
				stdout_v.write_all(&sbox)?;
				writeln!(lock, "{}", if cli.brief { "" } else { " -" })?;
			} else {
				let hex = u8vec_to_hex(&sbox, cli.upper);
				if cli.brief {
					writeln!(lock, "{}", hex)?;
				} else {
					writeln!(lock, "{} {}", hex, path.display())?;
				}
			}
			sbox.fill(0); //reset (clear)
		}
	}
	Ok(())
}
