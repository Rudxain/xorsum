//why isn't this in `std`?
pub fn bytevec_tohex(vector: &Vec<u8>, upper: bool) -> String {
	let mut hex = String::new();
	for byte in vector {
		hex += &(if upper {
			format!("{byte:02X}")
		} else {
			format!("{byte:02x}")
		})
	}
	hex
}

pub fn xor_hasher<T: std::iter::Iterator<Item = Result<u8, std::io::Error>>>(
	bytes: T,
	len: usize,
) -> Vec<u8> {
	let mut sbox = vec![0; len]; //state box, IV = 0
	if len > 0 {
		for (i, b) in bytes.enumerate() {
			sbox[i % len] ^= b.unwrap();
		}
	}
	sbox
}

pub const NAME: &str = "xorsum";
pub const VERSION: &str = "3.0.1"; //should be the same as in Cargo.toml
pub const DEFAULT_SIZE: usize = 8;

pub const NO_FILE_MSG: &str = "No such file or directory";
pub const DIR_MSG: &str = "Is a directory";

pub const HELP_ARG: [&str; 2] = ["-h", "--help"];
pub const VER_ARG: [&str; 2] = ["-v", "--version"];
pub const LEN_ARG: [&str; 2] = ["-l", "--length"];
pub const BRIEF_ARG: [&str; 2] = ["-b", "--brief"];
pub const RAW_ARG: [&str; 2] = ["-r", "--raw"];
pub const LOWER_ARG: [&str; 2] = ["-a", "--lower"];
pub const UPPER_ARG: [&str; 2] = ["-A", "--UPPER"];

pub fn print_help() {
	println!(
		"\
		Usage: {NAME} [OPTION]... [FILE]...\n\
		If no FILES are given, or if FILE is \"-\", reads Standard Input\n\
		Options:\
	"
	);
	println!("{}, {}	Print this help", HELP_ARG[0], HELP_ARG[1]);
	println!("{}, {}	Print version number", VER_ARG[0], VER_ARG[1]);
	println!(
		"{}, {} <LEN>	Hash size in bytes (prior to hex-encoding). Default {}",
		LEN_ARG[0], LEN_ARG[1], DEFAULT_SIZE
	);
	println!(
		"{}, {}	Only print hash, no filenames",
		BRIEF_ARG[0], BRIEF_ARG[1]
	);
	println!("{}, {}	Print raw bytes, not hex", RAW_ARG[0], RAW_ARG[1]);
	println!("{}, {}	lowercase hex (default)", LOWER_ARG[0], LOWER_ARG[1]);
	println!("{}, {}	UPPERCASE hex", UPPER_ARG[0], UPPER_ARG[1]);
}
