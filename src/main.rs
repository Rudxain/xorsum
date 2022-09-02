use clap::{ArgGroup, Parser};
use std::io::{stdin, stdout, Write};

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
	file: Vec<std::path::PathBuf>,
}

fn main() -> std::io::Result<()> {
	let cli = Cli::parse();

	if (cli.full && cli.brief) || (cli.lower && cli.upper) || (cli.hex && cli.raw) {
		unreachable!()
	}

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
		return Ok(());
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

	if cli.rick {
		println!("We're no strangers to love...");
		return Ok(());
	}

	//allocate once, reuse everywhere
	let mut sbox = vec![0; cli.length]; //state box, IV = 0

	if cli.file.is_empty() {
		stream_processor(stdin().lock(), &mut sbox)?;
		if cli.raw {
			stdout().lock().write_all(&sbox).unwrap()
		} else {
			println!(
				"{}{}",
				bytevec_tohex(&sbox, cli.upper),
				if cli.brief { "" } else { " -" }
			)
		}
	} else {
		for path in cli.file {
			let h = std::path::Path::new("-");
			if path.is_file() || path == h {
				if path == h {
					//JIC, avoid creating multiple BRs on the same stdin
					stream_processor(stdin().lock(), &mut sbox)?;
				} else {
					stream_processor(std::fs::File::open(&path)?, &mut sbox)?;
				}

				if cli.raw {
					stdout().lock().write_all(&sbox).unwrap();
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
