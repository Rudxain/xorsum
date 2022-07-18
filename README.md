# Algorithm
It uses the [XOR-cipher](https://en.wikipedia.org/wiki/XOR_cipher) to compute a [checksum](https://en.wikipedia.org/wiki/Checksum) digest. Basically, it splits the data in chunks whose length is the same as the digest size (padding with 0), and `XOR`s all chunks between each other into a new chunk that's used as output.

This isn't a good [hash function](https://en.wikipedia.org/wiki/Hash_function). It lacks the [Avalanche Effect](https://en.wikipedia.org/wiki/Avalanche_effect), because flipping 1 input bit flips 1 output bit.

# Program
The raw digest size is 64bit (8Byte) by default, but can be set to any valid `usize` value with the `--length` option. The actual size is bigger because the raw digest is expanded to hexadecimal by default. I choose 8, because CRC32 uses 4 and MD5 uses 16, and to make it easier for downgrade implementations to replicate, because 64b fits within a CPU register and can be emulated using 2 `u32`s.

The [initialization-vector](https://en.wikipedia.org/wiki/Initialization_vector) is hardcoded to be 0.

Both the naming and behavior are influenced by [`cksum`](https://en.wikipedia.org/wiki/Cksum), [`md5sum`](https://en.wikipedia.org/wiki/Md5sum), and [`b3sum`](https://github.com/BLAKE3-team/BLAKE3/tree/master/b3sum).

# Usage
To install latest release from [crates.io](https://crates.io) registry:
```sh
cargo install xorsum
```
This isn't guaranteed to be the latest version, but it will never throw compilation errors.

To install latest dev crate from GH:
```sh
cargo install --git https://github.com/Rudxain/xorsum.git
```
This is the most recent version. Compilation isn't guaranteed. Semver may be broken. And `--help` may not reflect actual program behavior.

To get an already-compiled non-dev executable, go to [GH releases](https://github.com/Rudxain/xorsum/releases).

Argument "syntax" (any order is allowed, but it's good practice to place options near each other):
```sh
xorsum [OPTION]... [FILE]...
```

For ℹinfo about options, run:
```sh
xorsum --help
```

# Examples
## Regular use
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

#same result when using stdin
echo -n aaaa | xorsum -l 4
#out: "61616161 -"

xorsum a --brief #`-l 8` is implicit
#out: "6161616100000000"
```
Note: `echo -n` has [different behavior depending on OS and binary version](https://unix.stackexchange.com/a/65819), it might include line endings like `\n` (LF) or `\r\n` (CR-LF). The outputs shown in the example are the (usually desired) result of **NOT** including a new-line.

PowerShell will ignore `-n` because `echo` is an alias of [`Write-Output`](https://docs.microsoft.com/en-us/powershell/module/microsoft.powershell.utility/write-output) and therefore can't recognize `-n`. [`Write-Host -NoNewline`](https://docs.microsoft.com/en-us/powershell/module/microsoft.powershell.utility/write-host?view=powershell-7.2#example-1-write-to-the-console-without-adding-a-new-line) can't be piped nor redirected, so it's not a good alternative.

## Emulating AE
`--length` **doesn't truncate** the digest:
```sh
xorsum some_big_file -b -l 3 #"00ff55"
xorsum some_big_file -b -l 2 #"69aa" NOT "00ff"
```
As you can see, `-l` can return very different hashes from the same input. This property can be exploited to emulate the Avalanche Effect (to some extent)

## Weird names
What if you have a file named "-"?
```sh
echo bruh > -
#to prevent interpretation as an `OPTION`, use "./" relative path
xorsum ./-
```

## Finding corrupted bytes
If you have 2 copies of a file and 1 is corrupted, you can attempt to ["triangulate"](https://en.wikipedia.org/wiki/Triangulation) the index of a corrupted byte, without manually searching the entire file. This is useful when dealing with big raw-binary files
```sh
xorsum a b
#"6c741b7863326b2c a"
#"6c74187863326b2c b"
#the 0-based index is 2 when using `-l 8`
#mathematically, i mod 8 = 2

xorsum a b -l 3
#"3d5a0a a"
#"3d590a b"
#i mod 3 = 1

xorsum a b -l 2
#"7f12 a"
#"7c12 b"
#i mod 2 = 0

#you can repeat this process with different `-l` values, to solve it easier.
#IIRC, using primes gives you more info about the index
```
There are programs (like [`diff`](https://en.wikipedia.org/wiki/Diff)) that compare bytes for you, and are much more efficient and user-friendly. But if you are into math puzzles, this is a good way to pass the time by solving [systems of linear modular equations](https://youtu.be/LInNgWMtFEs)

# Personal thoughts
I was surprised that I couldn't find any implementation of a checksum algorithm completely based on the `XOR` op. So I posted this for the sake of completeness, and because I'm learning Rust. I also made this for people with low-power devices

# ⚠DISCLAIMER
0. **DO NOT USE FOR 🔐CRYPTOGRAPHIC PURPOSES.** The algorithm is **not** crypto-secure

1. **DO NOT SHARE HASHES OF PRIVATE DATA.** You might be leaking sensitive information. Small hashes and bigger files tend to be safer, because the `sbox` will (probably) have enough bytes to "mix well"
