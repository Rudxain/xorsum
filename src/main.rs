use std::io::{ Read, Result, stdin };
use std::process::exit;

const NAME: &str = "xorsum";
const VERSION: &str = "1.0.0";

const HELP_CMD: [&str; 2] = ["-h", "--help"];
const VER_CMD: [&str; 2] = ["-v", "--version"];
const BRIEF_CMD: [&str; 2] = ["-b", "--brief"];
const LOWER_CMD: [&str; 2] = ["-a", "--lower"];
const UPPER_CMD: [&str; 2] = ["-A", "--UPPER"];

fn main() -> Result<()> {
	let args: Vec<String> = std::env::args().collect();

	let mut files: Vec<String> = Vec::new();
	let mut brief = false;
	let mut upper = false; //uppercase hex

	let mut is_len = false;
	let mut byte_len: usize = 0x10; //digest/hash size in Bytes

	let mut first_iter = true;
	for arg in &args {
		if first_iter {first_iter = false; continue}
		if is_len {
			byte_len = arg.parse().unwrap();
			is_len = false;
			continue
		}
		if arg == "--len" || arg == "-l" {
			is_len = true;
			continue
		}

		if arg == HELP_CMD[0] || arg == HELP_CMD[1] {
			println!("Usage: {} [OPTION]... [FILE]...", NAME);
			println!("If no FILES are given, or if FILE is \"-\", reads Standard Input");
			println!("OPTIONS:");
			println!("{}, {}		Print help", HELP_CMD[0], HELP_CMD[1]);
			println!("{}, {}		Print version number", VER_CMD[0], VER_CMD[1]);
			println!("{}, {}		Only hash, no filename", BRIEF_CMD[0], BRIEF_CMD[1]);
			println!("{}, {}		lowercase hex (default)", LOWER_CMD[0], LOWER_CMD[1]);
			println!("{}, {}		UPPERcase hex", UPPER_CMD[0], UPPER_CMD[1]);
			exit(0);
		}
		if arg == VER_CMD[0] || arg == VER_CMD[1] {
			println!("{} {}", NAME, VERSION);
			exit(0);
		}

		if arg == BRIEF_CMD[0] || arg == BRIEF_CMD[1] {brief = true; continue}
		if arg == UPPER_CMD[0] || arg == UPPER_CMD[1] {upper = true; continue}
		if arg == LOWER_CMD[0] || arg == LOWER_CMD[1] {upper = false; continue}

		if arg.starts_with("-") {
			println!("Unrecognized option. Run `{} --help` for details", NAME);
			exit(1);
		}
		else {
			files.push(arg.to_string());
		}
	}

	let read_in = files.len() == 0;

	let mut sbox: Vec<u8> = vec![0; byte_len]; //state box, IV = 0

	let mut i: usize = 0;
	if read_in {
		for b in stdin().bytes() {
			sbox[i] ^= b.unwrap();
			i += 1;
			i %= byte_len;
		}

		if upper {println!("{:02X?}", sbox)} else {println!("{:02x?}", sbox)}
	}
	else {
		for path in files {
			let f = std::fs::File::open(&path)?;
			//I hope this uses a buffer to prevent RAM from exploding
			for b in f.bytes() {
				sbox[i] ^= b.unwrap();
				i += 1;
				i %= byte_len;
			}

			if brief {
				if upper {println!("{:02X?}", sbox)} else {println!("{:02x?}", sbox)}
			} else {
				if upper {println!("{:02X?} {}", sbox, path)} else {println!("{:02x?} {}", sbox, path)}
			}

			sbox.fill(0);
		}
	}

	Ok(())
}