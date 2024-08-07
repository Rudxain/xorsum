#![warn(
	future_incompatible,
	clippy::exit,
	clippy::unwrap_used,
	clippy::print_stdout,
	clippy::print_stderr,
	clippy::cargo,
	clippy::pedantic,
	clippy::shadow_unrelated,
	clippy::string_to_string,
	clippy::decimal_literal_representation,
	clippy::unseparated_literal_suffix,
	clippy::empty_structs_with_brackets,
	clippy::format_push_string
)]
// can't `forbid`, blame `clap::Parser`
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

use std::path::{Path, PathBuf};

mod utils;
#[allow(clippy::wildcard_imports)]
use utils::*;

/// default hash/digest/output length/size in bytes
const DEFAULT_LEN: usize = 8;

/// crate and program name
const NAME: &str = "xorsum";

use clap::Parser;
#[derive(Parser)]
#[clap(
	version,
	about = "Print XOR (64-bit) checksums",
	long_about = "If no FILES are given, or if FILE is \"-\", reads Standard Input"
)]
struct Cli {
	/// Digest size in octets (prior to hex-encoding)
	#[clap(short, long, default_value_t = DEFAULT_LEN, value_parser)]
	length: usize,

	/// Only print digest, no filenames
	#[clap(short, long, action)]
	brief: bool,

	/// Files to digest
	#[clap(value_parser)]
	file: Vec<PathBuf>,
}

#[allow(clippy::type_complexity)]
fn main() -> Result<(), Box<dyn std::error::Error>> {
	use std::{io, io::Write};

	let cli = Cli::parse();

	let stdin_v = io::stdin();
	// print without `lock`
	let mut stdout_v = io::stdout();
	let mut stderr_v = io::stderr();

	// allocate once, reuse everywhere (remember to reset!)
	let mut sbox = vec![0; cli.length]; // state box, IV = 0

	if cli.file.is_empty() {
		stream_digestor(stdin_v, &mut sbox)?;
		writeln!(
			stdout_v,
			"{}{}",
			to_hex_inplace(sbox),
			if cli.brief { "" } else { " -" }
		)?;
	} else {
		for path in cli.file {
			debug_assert_eq!(sbox, vec![0; cli.length]);

			if path == Path::new("-") {
				// it seems multiple BRs are fine if not simultaneous
				stream_digestor(io::stdin(), &mut sbox)?;
			} else {
				match std::fs::File::open(&path) {
					Ok(f) => stream_digestor(f, &mut sbox)?,
					Err(e) => {
						writeln!(stderr_v, "{NAME}: {}: {e}", path.display())?;
						continue;
					}
				};
			}

			let hex = to_hex_clone(&sbox);
			if cli.brief {
				writeln!(stdout_v, "{hex}")
			} else {
				writeln!(stdout_v, "{hex} {}", path.display())
			}?;
			// this is not needed in the last iter
			sbox.fill(0); //reset (clear)
		}
	}
	Ok(())
}
