# `xorsum`
This program computes a hash by using an algorithm based on the [XOR-cipher](https://en.wikipedia.org/wiki/XOR_cipher). Basically, it splits the file in chunks whose byte-length is the same as the digest size (padding with 0s), and XORs all those chunks together into a new chunk of the same size, the resulting chunk is printed as an array of hexadecimal bytes. I'm still trying to fix the formatting of the output to be a single sequence of hex nibbles without delimiter.

This is not a good hash function, because it doesn't have the [Avalanche Effect](https://en.wikipedia.org/wiki/Avalanche_effect). It is intended to be a simple/basic, educational, and fast checksum algorithm.

Currently, there's no support for Standard-Input, but it'll be added in the future. More flags and args will also be available later.

# Usage
Install crate:
```sh
cargo install xorsum
```

Build & run from source:
```sh
cd "$REPO" #path to cloned/downloaded repo
cargo run -- "$FILE" #path to file you want to hash
```

If you have the compiled executable, do:
```sh
cd "$DIR" #directory where exec is contained, usually "target"
./xorsum "$FILE" #hash the same (or other) file
```

# Examples
Output of `xorsum Cargo.toml` (old commit):
```sh
[00, 33, 3D, 24, 40, 6A, 50, 0A, 5C, 4B, 63, 1F, 68, 1D, 09, 45]
```

# ⚠ DISCLAIMER ⚠
**DO NOT USE FOR CRYPTOGRAPHIC PURPOSES.** The algorithm is **not** crypto-secure
