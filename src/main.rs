use std::io::Read;
//I don't want to pollute the global scope, so I'll use `use` only when needed

//convert vector of bytes to a contiguous hex string
//supports UPPERCASE and lowercase
fn bytevec_tohex(vector: &Vec<u8>, upper: bool) -> String {
	let mut hex = String::new();
	for byte in vector {
		hex += &(if upper {format!("{byte:02X}")} else {format!("{byte:02x}")})
	}
	hex
}

fn main() -> std::io::Result<()> {
	const NAME: &str = "xorsum";
	const VERSION: &str = "2.0.0"; //should be the same as in Cargo.toml

	const HELP_CMD: [&str; 2] = ["-h", "--help"];
	const VER_CMD: [&str; 2] = ["-v", "--version"];
	const BRIEF_CMD: [&str; 2] = ["-b", "--brief"];
	const LOWER_CMD: [&str; 2] = ["-a", "--lower"];
	const UPPER_CMD: [&str; 2] = ["-A", "--UPPER"];
	const LEN_CMD: [&str; 2] = ["-l", "--len"];
	const RAW_CMD: [&str; 2] = ["-r", "--raw"];
	/*
	`--raw` Writes raw output bytes to stdout, rather than hex.
	--no-names is implied. In this case, only a single input is allowed.
	*/

	let mut paths: Vec<String> = Vec::new();
	let mut brief = false;
	let mut upper = false; //uppercase hex
	let mut raw = false;

	//temporary internal flag to remember if prev arg was a `LEN_CMD`
	let mut is_len = false;
	let mut digest_len = 0x10;

	let mut first_iter = true;
	for arg in std::env::args() {
		//awkward way to skip `args[0]`
		if first_iter {first_iter = false; continue}

		if is_len {
			digest_len = arg.parse().unwrap();
			is_len = false;
			continue
		}
		if arg == LEN_CMD[0] || arg == LEN_CMD[1] {
			is_len = true;
			continue
		}

		if arg == HELP_CMD[0] || arg == HELP_CMD[1] {
			println!("Usage: {NAME} [OPTION]... [FILE]...");
			println!("If no FILES are given, or if FILE is \"-\", reads Standard Input");
			println!("OPTIONS:");
			println!("{}, {}		Print help", HELP_CMD[0], HELP_CMD[1]);
			println!("{}, {}		Print version number", VER_CMD[0], VER_CMD[1]);
			println!("{}, {}		Only hash, no filename", BRIEF_CMD[0], BRIEF_CMD[1]);
			println!("{}, {}		lowercase hex (default)", LOWER_CMD[0], LOWER_CMD[1]);
			println!("{}, {}		UPPERCASE hex", UPPER_CMD[0], UPPER_CMD[1]);
			println!("{}, {}		Digest size in bytes. Default is 16B (128bit)", LEN_CMD[0], LEN_CMD[1]);
			println!("{}, {}		No hex, only raw bytes. `{}` is implied. Ignores all files, but the 1st", RAW_CMD[0], RAW_CMD[1], BRIEF_CMD[0]);
			return Ok(())
		}
		if arg == VER_CMD[0] || arg == VER_CMD[1] {
			println!("{NAME} {VERSION}");
			return Ok(())
		}

		if arg == BRIEF_CMD[0] || arg == BRIEF_CMD[1] {brief = true; continue}
		if arg == RAW_CMD[0] || arg == RAW_CMD[1] {raw = true; continue}
		if arg == UPPER_CMD[0] || arg == UPPER_CMD[1] {upper = true; continue}
		if arg == LOWER_CMD[0] || arg == LOWER_CMD[1] {upper = false; continue}

		if arg.starts_with("-") {
			println!("Unrecognized option. Run `{NAME} --help` for details");
			return Ok(())
		}
		else {
			paths.push(arg);
		}
	}
	if raw {brief = true} //avoid bugs

	let mut sbox = vec![0; digest_len]; //state box, IV = 0
	let mut i = 0;

	if paths.len() == 0 {
		if digest_len > 0 {
			for b in std::io::stdin().bytes() {
				sbox[i] ^= b.unwrap();
				i += 1;
				i %= digest_len;
			}
		}
		println!("{}", bytevec_tohex(&sbox, upper))
	}
	else {
		for p in paths {
			if digest_len > 0 {
				let f = std::fs::File::open(&p)?;
				//I hope this uses a buffer to prevent RAM from exploding
				for b in f.bytes() {
					sbox[i] ^= b.unwrap();
					i += 1;
					i %= digest_len;
				}
			}
			if brief { println!("{}", bytevec_tohex(&sbox, upper)) }
			else { println!("{} {p}", bytevec_tohex(&sbox, upper)) }

			sbox.fill(0) //reset
		}
	}
	Ok(())
}