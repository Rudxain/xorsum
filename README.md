# Algorithm
Computes a hash by using an algorithm based on the [XOR-cipher](https://en.wikipedia.org/wiki/XOR_cipher). Basically, it splits the file in chunks whose byte-length is the same as the digest size (padding with 0s), and XORs all those chunks together into a new chunk of the same size, the resulting chunk is printed as an array of hexadecimal bytes.

This isn't a good hash function, it's only good for checksums, because it lacks the [Avalanche Effect](https://en.wikipedia.org/wiki/Avalanche_effect), flipping 1 input bit flips 1 output bit. It is intended to be a simple/basic, educational, and fast checksum algorithm.

# Program
The digest size is 128bit by default, but can be set to any valid `usize` value with the `--len` option. The IV is hardcoded to be 0x0, it'll allow custom values in the future.

The naming is based on Unix and GNU-coreutils naming conventions, like [`cksum`](https://en.wikipedia.org/wiki/Cksum) and [`md5sum`](https://en.wikipedia.org/wiki/Md5sum). The behavior of the program is also intended to be similar (but not identical) to those checksum programs, with some inspiration from [`b3sum`](https://github.com/BLAKE3-team/BLAKE3/tree/master/b3sum).

I'm still trying to fix the formatting of the output to be a single sequence of hex nibbles without delimiter.

# Usage
```sh
cargo install xorsum
xorsum [OPTION]... [FILE]...
```

If you want to build from source:
```sh
cd [REPO] #path to cloned/downloaded repo
cargo build --release
```

# Examples
Let's create an empty text file named `a`. The output of `xorsum --len 4 a` should be:
```
[00, 00, 00, 00] a
```

If we write "aaaa" to this file and rehash it with `xorsum a -l 4`, the output will be:
```
[61, 61, 61, 61] a
```
Because "61" is the hex value of UTF-8 char "a"

Rehashing the file with `xorsum a` yields:
```
[61, 61, 61, 61, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00] a
```
This is because the IV is all 0s, and the padding is 0 too.

For more info, run:
```sh
xorsum --help
```

# ⚠DISCLAIMER
**DO NOT USE FOR 🔐CRYPTOGRAPHIC PURPOSES.** The algorithm is **🔓not** crypto-secure
