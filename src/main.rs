//I don't want to pollute the global scope, so I'll use `use` sparingly
use std::io::{stdin, stdout, Read, Write};
use xorsum::*; //temporary pollution, lol

fn main() -> std::io::Result<()> {
	let mut path_args: Vec<String> = Vec::new();
	let mut brief = false;
	let mut upper = false;
	let mut raw = false;

	let mut digest_len = DEFAULT_SIZE;

	{//ensure both vars are temp
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
				path_args.push(arg) //interpret as filename
			}
		}
	}

	if path_args.len() == 0 {
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
	} else {//lol, I noticed this says "pain"
		for p_a in path_args {
			let path = std::path::Path::new(&p_a);
			if path.is_file() || p_a == "-" {
				let hash = if p_a == "-" {
					xor_hasher(stdin().bytes(), digest_len)
				}
				//I hope this uses a buffer to prevent RAM from exploding
				else {
					xor_hasher(std::fs::File::open(&p_a)?.bytes(), digest_len)
				};

				if raw {
					stdout().write_all(&hash).unwrap();
					println!("{}", if brief { "" } else { " -" })
				} else {
					let hex = bytevec_tohex(&hash, upper);
					if brief {
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
		}
	}
	Ok(())
}
