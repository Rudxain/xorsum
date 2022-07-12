# Algorithm
It uses the [XOR-cipher](https://en.wikipedia.org/wiki/XOR_cipher) to compute a hash. Basically, it splits the data in chunks whose length is the same as the digest size (padding with 0), and `XOR`s all those chunks together into a new chunk of the same size, the resulting chunk is used as output.

This isn't a good hash function, it's only good for checksums, because it lacks the [Avalanche Effect](https://en.wikipedia.org/wiki/Avalanche_effect), flipping 1 input bit flips 1 output bit. It is intended to be a simple/basic, educational, and fast checksum algorithm.

# Program
The digest size is 64bit (8Byte) by default, but can be set to any valid `usize` value with the `--length` option. The [initialization-vector](https://en.wikipedia.org/wiki/Initialization_vector) is hardcoded to be 0.

Both the naming and behavior are based on  [`cksum`](https://en.wikipedia.org/wiki/Cksum), [`md5sum`](https://en.wikipedia.org/wiki/Md5sum), and [`b3sum`](https://github.com/BLAKE3-team/BLAKE3/tree/master/b3sum).

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

For info about options, run:
```sh
xorsum --help
```

# Examples
Let's create an empty text file named `a`. The output of `xorsum --len 4 a` should be:
```
00000000 a
```

If we write "aaaa" to this file and rehash it with `xorsum a -l 4`, the output will be:
```
61616161 a
```
Because "61" is the hex value of UTF-8 char "a"

Rehashing the file with `xorsum a -b` yields:
```
6161616100000000
```
This is because both the IV and padding are all zeros.

# ⚠DISCLAIMERS
0. **DO NOT USE FOR 🔐CRYPTOGRAPHIC PURPOSES.** The algorithm is **not** crypto-secure.

1. **DO NOT SHARE HASHES OF PRIVATE DATA.** You might be leaking sensitive information. The smaller the file, the more data you leak. The bigger the hash, the more data you leak. Small hashes and bigger files are safer, because the `sbox` will (probably) have enough bytes to "mix well".

I am **not** responsible for any misuse of this software
