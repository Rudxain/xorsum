# Algorithm
It uses the [XOR-cipher](https://en.wikipedia.org/wiki/XOR_cipher) to compute a [checksum](https://en.wikipedia.org/wiki/Checksum) digest. Basically, it splits the data in chunks whose length is the same as the digest size (padding with 0), and `XOR`s all chunks between each other into a new chunk that's used as output.

This isn't a [hash function](https://en.wikipedia.org/wiki/Hash_function). It lacks the [Avalanche Effect](https://en.wikipedia.org/wiki/Avalanche_effect), because flipping 1 input bit flips 1 output bit.

# Program
The raw digest size is 64bit (8Byte) by default, but can be set to any valid `usize` value with the `--length` option. The actual size is bigger because the raw digest is expanded to hexadecimal by default. The [initialization-vector](https://en.wikipedia.org/wiki/Initialization_vector) is hardcoded to be 0.

Both the naming and behavior are based on  [`cksum`](https://en.wikipedia.org/wiki/Cksum), [`md5sum`](https://en.wikipedia.org/wiki/Md5sum), and [`b3sum`](https://github.com/BLAKE3-team/BLAKE3/tree/master/b3sum).

# Usage
To install:
```sh
cargo install xorsum #(from crates.io registry)
```

Argument "syntax" (any order is allowed, but it's good practice to place options near each other):
```sh
xorsum [OPTION]... [FILE]...
```

For ℹinfo about options, run:
```sh
xorsum --help
```

# Example
```sh
#let's create an empty file named "a"
echo -n > a
xorsum --length 4 a
#output will be "00000000 a" (without quotes)

#write "aaaa" to this file and rehash it
echo -n aaaa > a
xorsum a -l 4
#out: "61616161 a"
#because "61" is the hex value of the UTF-8 char "a"

xorsum a --brief #`-l 8` is implicit
#out: "6161616100000000"

#`--length` DOESN'T TRUNCATE the output digest
xorsum some-big-file -b -l 3 #"00ff55"
xorsum some-big-file -b -l 2 #"69aa" NOT "00ff"
#as you can see, `-l` can return very different hashes from the same file
#this property can be exploited to emulate the Avalanche Effect (to some extent)

#what if you have a file named "-"?
echo bruh > -
#to prevent interpretation as an `OPTION`, use "./" relative path
xorsum ./-
```
Note: `echo -n` has [different behavior depending on OS and binary version](https://unix.stackexchange.com/a/65819), it might include line endings like `\n` (LF) or `\r\n` (CR-LF). The outputs shown in the example are the (usually desired) result of **NOT** including a new-line.

PowerShell will ignore `-n` because `echo` is an alias of [`Write-Output`](https://docs.microsoft.com/en-us/powershell/module/microsoft.powershell.utility/write-output) and therefore can't recognize `-n`. [`Write-Host -NoNewline`](https://docs.microsoft.com/en-us/powershell/module/microsoft.powershell.utility/write-host?view=powershell-7.2#example-1-write-to-the-console-without-adding-a-new-line) can't be piped nor redirected, so it's not a good alternative.

# ⚠DISCLAIMER
0. **DO NOT USE FOR 🔐CRYPTOGRAPHIC PURPOSES.** The algorithm is **not** crypto-secure.

1. **DO NOT SHARE HASHES OF PRIVATE DATA.** You might be leaking sensitive information. Small hashes and bigger files tend to be safer, because the `sbox` will (probably) have enough bytes to "mix well".

I am **not** responsible for any misuse of this software
