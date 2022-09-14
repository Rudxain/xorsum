use clap::{ArgGroup, Parser};
use std::{
	io::{stdin, stdout, Write},
	path,
};

mod module;
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
	group(ArgGroup::new("code").args(&["hex", "raw"]))
)]
struct Cli {
	/// Digest size in bytes (prior to hex-encoding)
	#[clap(short, long, default_value_t = DEFAULT_LEN, value_parser)]
	length: usize,

	/// Print hash + filename (default)
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

	/// Print hexadecimal digest (default)
	#[clap(short = 'x', long, action)]
	hex: bool,
	/// Print raw bytes, not hex
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

	/// Files to hash
	#[clap(value_parser)]
	file: Vec<path::PathBuf>,
}

fn main() -> std::io::Result<()> {
	let cli = Cli::parse();

	if (cli.full && cli.brief) || (cli.lower && cli.upper) || (cli.hex && cli.raw) {
		unreachable!()
	}

	let mut egg = false;

	//is there some way to convert all these `ifs` into a single `match`?
	if cli.hell {
		println!(
			"{}",
			rand_pick(&[
				"I can't go to hell. I'm all out of vacation days.",
				"Highway to Hell!",
				"RIP N' TEAR",
				"Son't eorry evrryone makez nistakes while typong",
			])
		);
		egg = true
	}
	if cli.heaven {
		println!(
			"{}",
			rand_pick(&[
				"Locked Out of Heaven!",
				"Stairway to Heaven",
				"[Heaven], are you WATCHING?",
				"The Holy C",
			])
		);
		egg = true
	}
	if cli.hello {
		println!("world!");
		egg = true
	}
	if cli.olé {
		println!("¡Ostia tío! ¿Cómo has logrado escribir eso?");
		egg = true
	}
	if cli.rick {
		println!("We're no strangers to love...");
		egg = true
	}

	if egg {return Ok(())}

	//allocate once, reuse everywhere
	let mut sbox = vec![0; cli.length]; //state box, IV = 0

	if cli.file.is_empty() {
		stream_processor(stdin().lock(), &mut sbox)?;
		if cli.raw {
			stdout().lock().write_all(&sbox)?
		} else {
			println!(
				"{}{}",
				u8vec_to_hex(&sbox, cli.upper),
				if cli.brief { "" } else { " -" }
			)
		}
	} else {
		let mut stdout_v = stdout();
		let mut lock = stdout_v.lock();

		for path in cli.file {
			let hyphen = path::Path::new("-");
			if path.is_file() || path == hyphen {
				if path == hyphen {
					//JIC, avoid creating multiple BRs on the same stdin
					stream_processor(stdin().lock(), &mut sbox)?;
				} else {
					stream_processor(std::fs::File::open(&path)?, &mut sbox)?;
				}

				if cli.raw {
					stdout_v.write_all(&sbox)?;
					writeln!(lock, "{}", if cli.brief { "" } else { " -" })?
				} else {
					let hex = u8vec_to_hex(&sbox, cli.upper);
					if cli.brief {
						writeln!(lock, "{hex}")?
					} else {
						writeln!(lock, "{hex} {}", path.display())?
					}
				}
			} else {
				std::io::stderr().lock().write_all(
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
				)?;
			}
			sbox.fill(0) //reset
		}
	}
	Ok(())
}
