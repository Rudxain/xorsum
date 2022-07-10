# Algorithm
Computes a hash by using an algorithm based on the [XOR-cipher](https://en.wikipedia.org/wiki/XOR_cipher). Basically, it splits the file in chunks whose byte-length is the same as the digest size (padding with 0s), and XORs all those chunks together into a new chunk of the same size, the resulting chunk is printed as an array of hexadecimal bytes.

This isn't a good hash function, it's only good for checksums, because it lacks the [Avalanche Effect](https://en.wikipedia.org/wiki/Avalanche_effect), flipping 1 input bit flips 1 output bit. It is intended to be a simple/basic, educational, and fast checksum algorithm.

# Program
The naming is based on Unix and GNU-coreutils naming conventions, like `cksum` and `md5sum`. The behavior of the program is also intended to be similar (but not identical) to those checksum programs.

I'm still trying to fix the formatting of the output to be a single sequence of hex nibbles without delimiter.

Currently, there's no support for Standard-Input, but it'll be added in the future. More flags and args will also be available later.

# Usage
```sh
cargo install xorsum
xorsum [FILE] ##path to file you want to hash
```

If you want to build & run from source:
```sh
cd [REPO] #path to cloned/downloaded repo
cargo run -- [FILE]
```

# Examples
Output of `xorsum Cargo.toml` (initial commit):
```sh
[00, 33, 3D, 24, 40, 6A, 50, 0A, 5C, 4B, 63, 1F, 68, 1D, 09, 45]
```

# ⚠DISCLAIMER
**DO NOT USE FOR 🔐CRYPTOGRAPHIC PURPOSES.** The algorithm is **🔓not** crypto-secure
