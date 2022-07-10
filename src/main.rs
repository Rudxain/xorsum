use std::io::Read;

fn main() -> std::io::Result<()> {
	let args: Vec<String> = std::env::args().collect();

	if args.len() == 1 || args[1] == "--help" ||  args[1] == "-h" {
		println!("usage: {} PATH", args[0]);
		std::process::exit(0);
	}

	let path = &args[1];
	let f = std::fs::File::open(path)?;

	const BYTE_SIZE: usize = 0x10; //digest/hash length in Bytes
	let mut sbox: [u8; BYTE_SIZE] = [0; BYTE_SIZE]; //state box, IV = 0

	let mut i: usize = 0;
	//I hope this uses a buffer to prevent RAM from exploding
	for b in f.bytes() {
		sbox[i] ^= b.unwrap();
		i += 1;
		i %= BYTE_SIZE;
	}

	//this is good enough, but the format is bad
	println!("{:02X?}", sbox);

	Ok(())
}