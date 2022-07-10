# `xorsum`
This program computes a hash by using an algorithm based on the [XOR-cipher](https://en.wikipedia.org/wiki/XOR_cipher). Basically, it splits the file in chunks whose byte-length is the same as the digest size (padding with 0s), and XORs all those chunks together into a new chunk of the same size, the resulting chunk is printed as an array of hexadecimal bytes. I'm still trying to fix the formatting of the output to be a single sequence of hex nibbles without delimiter.

Currently, there's no support for Standard-Input, but it'll be added in the future. More flags and args will also be available later.

# Usage
Clone/download the repo, then build & run from source:
```sh
cd "$REPO" #path to downloaded repo
cargo run -- "$FILE" #path to file you want to hash
```

If you already compiled the executable, do:
```sh
cd "$DIR" #directory where exec is contained
./xorsum "$FILE" #hash the same (or other) file
```

Example output:
```
[ 00, ff, 7f, 80, 69, 42, be, ef, aa, 55, 77, 86, 96, 01, 10, 88 ]
```
