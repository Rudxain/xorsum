#![warn(
	unused,
	future_incompatible,
	clippy::exit,
	clippy::unwrap_used,
	clippy::cargo,
	clippy::pedantic,
	clippy::nursery,
	clippy::shadow_unrelated,
	clippy::string_to_string,
	clippy::decimal_literal_representation,
	clippy::unseparated_literal_suffix,
	clippy::empty_structs_with_brackets,
	clippy::format_push_string,
)]
//can't `forbid`, blame `clap::Parser`
#![deny(
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
#![forbid(unsafe_code)]

use clap::Parser;

mod module;
#[allow(clippy::wildcard_imports)]
use crate::module::*;

///crate and program name
const NAME: &str = "xorsum";

#[derive(Parser)]
#[clap(
	version,
	about = "Print XOR (64-bit) checksums",
	long_about = "If no FILES are given, or if FILE is \"-\", reads Standard Input"
)]
struct Cli {
	///Digest size in bytes (prior to hex-encoding)
	#[clap(short, long, default_value_t = DEFAULT_LEN, value_parser)]
	length: usize,

	///Only print hash, no filenames
	#[clap(short, long, action)]
	brief: bool,

	///Files to hash
	#[clap(value_parser)]
	file: Vec<std::path::PathBuf>,
}

fn main() {
	use std::{
		io::{stderr, stdin, stdout, Write},
		path::Path,
	};

	let cli = Cli::parse();

	let stdin_v = stdin();
	//to print without `lock`
	let mut stdout_v = stdout();
	let mut stderr_v = stderr();

	//allocate once, reuse everywhere (remember to reset!)
	let mut sbox = vec![0; cli.length]; //state box, IV = 0

	if cli.file.is_empty() {
		sbox = stream_processor(stdin_v, sbox).unwrap();
		writeln!(
			stdout_v,
			"{}{}",
			u8vec_to_hex_inplace(sbox),
			if cli.brief { "" } else { " -" }
		)
		.unwrap();
	} else {
		for path in cli.file {
			if path == Path::new("-") {
				//avoid creating multiple BRs on the same stdin (just in case)
				sbox = stream_processor(stdin(), sbox).unwrap();
			} else {
				match std::fs::File::open(&path) {
					Ok(f) => sbox = stream_processor(f, sbox).unwrap(),
					Err(e) => {
						writeln!(stderr_v, "{}: {}: {}\n", NAME, path.display(), e).unwrap();
						continue;
					}
				};
			}

			let hex = u8vec_to_hex_outplace(&sbox);
			if cli.brief {
				writeln!(stdout_v, "{}", hex).unwrap();
			} else {
				writeln!(stdout_v, "{} {}", hex, path.display()).unwrap();
			}

			sbox.fill(0); //reset (clear)
		}
	}
}
