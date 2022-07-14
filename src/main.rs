//I don't want to pollute the global scope, so I'll use `use` sparingly
use std::io::{Read, Write, stdin, stdout};
use xorsum::*; //temporary pollution, lol

fn main() -> std::io::Result<()> {
	let mut paths: Vec<String> = Vec::new();
	let mut brief = false;
	let mut upper = false;
	let mut raw = false;

	let mut digest_len = DEFAULT_SIZE;

	{
		//ensure both vars are temp
		let mut is_len = false;
		let mut i0 = true;
		for arg in std::env::args() {
			//awkward way to skip `args[0]`
			if i0 {
				i0 = false;
				continue;
			}

			if is_len {
				digest_len = arg.parse().unwrap();
				is_len = false;
				continue;
			}
			if arg == LEN_ARG[0] || arg == LEN_ARG[1] {
				is_len = true;
				continue;
			}

			if arg == HELP_ARG[0] || arg == HELP_ARG[1] {
				print_help();
				return Ok(());
			}
			if arg == VER_ARG[0] || arg == VER_ARG[1] {
				println!("{NAME} {VERSION}");
				return Ok(());
			}

			if arg == BRIEF_ARG[0] || arg == BRIEF_ARG[1] {
				brief = true;
				continue;
			}
			if arg == RAW_ARG[0] || arg == RAW_ARG[1] {
				raw = true;
				continue;
			}
			if arg == UPPER_ARG[0] || arg == UPPER_ARG[1] {
				upper = true;
				continue;
			}
			if arg == LOWER_ARG[0] || arg == LOWER_ARG[1] {
				upper = false;
				continue;
			}

			if arg.starts_with("-") && arg != "-" {
				println!("Unrecognized option. Run `{NAME} --help` for details");
				return Ok(()); //IDK if this is good practice lol
			} else {
				paths.push(arg) //interpret as filename
			}
		}
	}

	if paths.len() == 0 {
		let hash = xor_hasher(stdin().bytes(), digest_len);
		if raw {
			stdout().write_all(&hash).unwrap()
		} else {
			println!(
				"{}{}",
				bytevec_tohex(&hash, upper),
				if brief { "" } else { " -" }
			)
		}
	} else {
		for p in paths {
			let mut no_file = false;
			let hash = if p == "-" {
				xor_hasher(stdin().bytes(), digest_len)
			}
			//I hope this uses a buffer to prevent RAM from exploding
			else {
				//TO-DO: check if file exists and print `{NAME}: {p}: {NO_FILE_MSG}`
				xor_hasher(std::fs::File::open(&p)?.bytes(), digest_len)
			};

			if raw {
				stdout().write_all(&hash).unwrap();
				println!("{}", if brief { "" } else { " -" })
			} else {
				let hex = bytevec_tohex(&hash, upper);
				if brief {
					println!("{hex}")
				} else {
					println!("{hex} {p}")
				}
			}
		}
	}
	Ok(())
}
