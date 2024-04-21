# xorsum

<div align=center>
  <img
    alt="XOR symbol at upper-left corner, plus-sign at bottom-right corner"
    src=icon.svg
    width=50% height=50%
  >
</div>

## Algorithm

It uses the [XOR-cipher](https://en.wikipedia.org/wiki/XOR_cipher) to compute a [checksum](https://en.wikipedia.org/wiki/Checksum) digest. Basically, it splits the data in chunks whose length is the same as digest size (padding with 0), and XORs all chunks between each other into a new chunk that's returned as output.

This isn't a good [hash function](https://en.wikipedia.org/wiki/Hash_function). It lacks the [Avalanche Effect](https://en.wikipedia.org/wiki/Avalanche_effect), because flipping 1 input bit flips 1 output bit.

## Program

The raw digest size is 8Bytes by default, but can be set to any valid `usize` value with the `--length` option. The printed size is 16B, because of hexadecimal expansion.

> Why 8B?

That was a _somewhat_ arbitrary decision. I've choosen 8 because it's the geometric-mean of 4 and 16, CRC32's and MD5's digest-sizes, respectively. 8B is easier to implement (in many langs) than 16B, when a constant fixed size is desired, because it fits in `u64`.

The [initialization-vector](https://en.wikipedia.org/wiki/Initialization_vector) is hardcoded to be 0.

Name and behavior heavily influenced by
- [uu-`hashsum`](https://github.com/uutils/coreutils/tree/main/src/uu/hashsum)
- [`cksum`](https://en.wikipedia.org/wiki/Cksum)
- [`md5sum`](https://en.wikipedia.org/wiki/Md5sum)
- [`b3sum`](https://github.com/BLAKE3-team/BLAKE3/tree/master/b3sum).

## Usage
To install latest release from [crates.io](https://crates.io) registry:
```sh
cargo install xorsum
```
This isn't guaranteed to be the latest version, but it'll always compile.

To install latest dev crate from GH:
```sh
cargo install --git https://github.com/Rudxain/xorsum.git
```
This is the **most recent** ("cutting-edge") version. Compilation isn't guaranteed. Semver may be broken. And `--help` may not reflect actual program behavior.

To get already-compiled non-dev executables, go to [GH releases](https://github.com/Rudxain/xorsum/releases). `*.elf`s will only be compatible with GNU-Linux x64. `*.exe`s will only be compatible with Windows x64. These **aren't setup/installer** programs, these are the same executables `cargo` would install, so you should run them from a terminal CLI, not click them.

For a Llamalab Automate implementation, visit [XOR hasher](https://llamalab.com/automate/community/flows/42903).

Argument "syntax":
```sh
xorsum [OPTIONS] [FILE]...
```

For ℹinfo about options, run:
```sh
xorsum --help
```

## Examples

### Regular use

```sh
# let's create an empty file named "a"
echo -n > a
xorsum --length 4 a
# output will be "00000000 a" (without quotes)

# write "aaaa" to this file and rehash it
echo -n aaaa > a
xorsum a -l 4
#out: "61616161 a"
# because "61" is the hex value of the UTF-8 char "a"

# same result when using stdin
echo -n aaaa | xorsum -l4
#61616161 -

xorsum a --brief #`-l 8` is implicit
#6161616100000000
```

> [!note]
> `echo -n` has [different behavior depending on OS and binary version](https://unix.stackexchange.com/a/65819), it might include line endings like `\n` (LF) or `\r\n` (CR-LF). The outputs shown in the example are the (usually desired) result of **NOT** including an EOL.
>
> PowerShell will ignore `-n` because `echo` is an alias of [`Write-Output`](https://docs.microsoft.com/en-us/powershell/module/microsoft.powershell.utility/write-output) and therefore can't recognize `-n`. [`Write-Host -NoNewline`](https://docs.microsoft.com/en-us/powershell/module/microsoft.powershell.utility/write-host?view=powershell-7.2#example-1-write-to-the-console-without-adding-a-new-line) can't be piped nor redirected, so it's not a good alternative.

### Emulating 🏔AE

`--length` **doesn't truncate** the output:

```sh
xorsum some_big_file -bl 3 #"00ff55"
xorsum some_big_file -bl 2 #"69aa" NOT "00ff"
```

As you can see, `-l` can return very different hashes from the same input. This property can be exploited to emulate the Avalanche Effect (to some extent).

### Finding corrupted bytes

If you have 2 copies of a file and 1 is corrupted, you can attempt to ["🔺️triangulate"](https://en.wikipedia.org/wiki/Triangulation) the index of a corrupted byte, without manually searching the entire file. This is useful when dealing with big raw-binary files

```sh
xorsum a b
#6c741b7863326b2c a
#6c74187863326b2c b
# the 0-based index is 2 when using `-l 8`
# mathematically, i mod 8 = 2

xorsum a b -l 3
#3d5a0a a
#3d590a b
# i mod 3 = 1

xorsum a b -l 2
#7f12 a
#7c12 b
# i mod 2 = 0

# you can repeat this process with different `-l` values, to solve it easier.
# IIRC, using primes gives you more info about the index
```

There are programs (like [`diff`](https://en.wikipedia.org/wiki/Diff)) that compare bytes for you, and are more efficient and user-friendly. But if you are into math puzzles, this is a good way to pass the time by solving [systems of linear modular equations](https://youtu.be/LInNgWMtFEs) 🤓.

## Personal thoughts

I was surprised I couldn't find any implementation of a checksum algorithm completely based on `XOR`, so I posted this for the sake of completeness, and because I'm learning Rust. I also made this for low-power devices, despite using the `std` lib, and only compiling to x64 (this will _probably_ change in the future, so don't worry).

## ⚠DISCLAIMER

0. **DO NOT SHARE HASHES OF PRIVATE DATA.** You might be leaking sensitive information. Small hashes and bigger files tend to be safer, because the `sbox` will (probably) have enough bytes to _"mix well"_.
1. This program is **not production-ready**. The version should be `0.x.y` to reflect the incompleteness of the code. I'm sorry for the inconvenience and potential confusion.
