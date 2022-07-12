use std::io::{Read, Write};
//I don't want to pollute the global scope, so I'll use `use` sparingly

//why isn't this in `std`?
fn bytevec_tohex(vector: &Vec<u8>, upper: bool) -> String {
	let mut hex = String::new();
	for byte in vector {
		hex += &(if upper {format!("{byte:02X}")} else {format!("{byte:02x}")})
	}
	hex
}

fn xor_hasher<T: std::iter::Iterator<Item = Result<u8, std::io::Error>>>(bytes: T, len: usize) -> Vec<u8>
{
	let mut sbox = vec![0; len]; //state box, IV = 0
	if len > 0 {
		let mut i = 0;
		for b in bytes {
			sbox[i] ^= b.unwrap();
			i = (i + 1) % len;
		}
	}
	sbox
}

const NAME: &str = "xorsum";
const VERSION: &str = "2.0.1"; //should be the same as in Cargo.toml
const DEFAULT_SIZE: usize = 8;

const HELP_ARG: [&str; 2] = ["-h", "--help"];
const VER_ARG: [&str; 2] = ["-v", "--version"];
const LEN_ARG: [&str; 2] = ["-l", "--length"];
const BRIEF_ARG: [&str; 2] = ["-b", "--brief"];
const RAW_ARG: [&str; 2] = ["-r", "--raw"];
const LOWER_ARG: [&str; 2] = ["-a", "--lower"];
const UPPER_ARG: [&str; 2] = ["-A", "--UPPER"];

fn print_help(){
	println!("\
	Usage: {NAME} [OPTION]... [FILE]...\n\
	If no FILES are given, or if FILE is \"-\", reads Standard Input\n\
	Options:\
	");
	println!("{}, {}	Print this help", HELP_ARG[0], HELP_ARG[1]);
	println!("{}, {}	Print version number", VER_ARG[0], VER_ARG[1]);
	println!("{}, {} <LEN>	Hash size in bytes (prior to hex-encoding). Default {}", LEN_ARG[0], LEN_ARG[1], DEFAULT_SIZE);
	println!("{}, {}	Only print hash, no filenames", BRIEF_ARG[0], BRIEF_ARG[1]);
	println!("{}, {}	Print raw bytes, not hex. `{}` is implied", RAW_ARG[0], RAW_ARG[1], BRIEF_ARG[0]);
	println!("{}, {}	lowercase hex (default)", LOWER_ARG[0], LOWER_ARG[1]);
	println!("{}, {}	UPPERCASE hex", UPPER_ARG[0], UPPER_ARG[1]);
}

fn main() -> std::io::Result<()> {
	let mut paths: Vec<String> = Vec::new();
	let mut brief = false;
	let mut upper = false;
	let mut raw = false;

	//temporary internal flag to remember if prev arg was a `LEN_CMD`
	let mut is_len = false;
	let mut digest_len = DEFAULT_SIZE;

	let mut first_iter = true;
	for arg in std::env::args() {
		//awkward way to skip `args[0]`
		if first_iter {first_iter = false; continue}

		if is_len {
			digest_len = arg.parse().unwrap();
			is_len = false;
			continue
		}
		if arg == LEN_ARG[0] || arg == LEN_ARG[1] {
			is_len = true;
			continue
		}

		if arg == HELP_ARG[0] || arg == HELP_ARG[1] {
			print_help();
			return Ok(())
		}
		if arg == VER_ARG[0] || arg == VER_ARG[1] {
			println!("{NAME} {VERSION}");
			return Ok(())
		}

		if arg == BRIEF_ARG[0] || arg == BRIEF_ARG[1] {brief = true; continue}
		if arg == RAW_ARG[0] || arg == RAW_ARG[1] {raw = true; continue}
		if arg == UPPER_ARG[0] || arg == UPPER_ARG[1] {upper = true; continue}
		if arg == LOWER_ARG[0] || arg == LOWER_ARG[1] {upper = false; continue}

		if arg.starts_with("-") && arg != "-" {
			println!("Unrecognized option. Run `{NAME} --help` for details");
			return Ok(()) //IDK if this is good practice lol
		}
		else {
			paths.push(arg) //interpret as filename
		}
	}
	if raw {brief = true} //avoid bugs

	if paths.len() == 0 {
		let hash = xor_hasher(std::io::stdin().bytes(), digest_len);
		if raw {
			std::io::stdout().write_all(&hash).unwrap()
		}
		else {
			println!("{}{}", bytevec_tohex(&hash, upper), if brief {""} else {" -"})
		}
	}
	else {
		for p in paths {
			let hash = if p == "-" { xor_hasher(std::io::stdin().bytes(), digest_len) }
			//I hope this uses a buffer to prevent RAM from exploding
			else { xor_hasher(std::fs::File::open(&p)?.bytes(), digest_len) };

			if raw {
				std::io::stdout().write_all(&hash).unwrap()
			}
			else {
				let hex = bytevec_tohex(&hash, upper);
				if brief { println!("{hex}") } else { println!("{hex} {p}") }
			}
		}
	}
	Ok(())
}
