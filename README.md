# Algorithm
It uses the [XOR-cipher](https://en.wikipedia.org/wiki/XOR_cipher) to compute a hash. Basically, it splits the data in chunks whose length is the same as the digest size (padding with 0), and `XOR`s all those chunks together into a new chunk of the same size, the resulting chunk is used as output.

This isn't a good hash function, it's only good for checksums, because it lacks the [Avalanche Effect](https://en.wikipedia.org/wiki/Avalanche_effect), flipping 1 input bit flips 1 output bit. It is intended to be a simple/basic, educational, and fast checksum algorithm.

# Program
The digest size is 64bit (8Byte) by default, but can be set to any valid `usize` value with the `--length` option. The [initialization-vector](https://en.wikipedia.org/wiki/Initialization_vector) is hardcoded to be 0.

Both the naming and behavior are based on  [`cksum`](https://en.wikipedia.org/wiki/Cksum), [`md5sum`](https://en.wikipedia.org/wiki/Md5sum), and [`b3sum`](https://github.com/BLAKE3-team/BLAKE3/tree/master/b3sum).

# Usage
```sh
cargo install xorsum #install from crates.io registry
xorsum [OPTION]... [FILE]... #argument "syntax"
```

For ℹinfo about options, run:
```sh
xorsum --help
```

# Example
```sh
#let's create an empty file named "a"
echo -n "" > a
xorsum --length 4 a
#output will be "00000000 a" (without quotes)

#write "aaaa" to this file and rehash it
echo -n aaaa > a
xorsum a -l 4
#out: "61616161 a"
#because "61" is the hex value of the UTF-8 char "a"

xorsum a --brief
#out: "6161616100000000"
#this is because both the IV and padding are all zeros

#what if you have a file named "-"?
echo bruh > "-"
#to prevent interpretation as an `OPTION`, use "./" relative path
xorsum "./-"
```
Note: `echo -n` has [different behavior depending on OS and binary version](https://unix.stackexchange.com/a/65819), it might include line endings like `\n` (LF) or `\r\n` (CR-LF). The outputs shown in the example are the (usually desired) result of **NOT** including a new-line.

PowerShell will ignore `-n` because `echo` is an alias of [`Write-Output`](https://docs.microsoft.com/en-us/powershell/module/microsoft.powershell.utility/write-output) and therefore can't recognize `-n`. Use `Write-Host -NoNewline` instead.

# ⚠DISCLAIMER
0. **DO NOT USE FOR 🔐CRYPTOGRAPHIC PURPOSES.** The algorithm is **not** crypto-secure.

1. **DO NOT SHARE HASHES OF PRIVATE DATA.** You might be leaking sensitive information. The smaller the file, the more data you leak. The bigger the hash, the more data you leak. Small hashes and bigger files are safer, because the `sbox` will (probably) have enough bytes to "mix well".

I am **not** responsible for any misuse of this software
