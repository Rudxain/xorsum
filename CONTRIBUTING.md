`main` should behave like [`hashsum`](https://github.com/uutils/coreutils/tree/main/src/uu/hashsum), so it must pass the GNU Test Suite.

I'm considering to implement `xorsl` (`xorsum` "lite"), which is targeted at embedded systems:
- Its API must be similar to BusyBox `cksum` and `md5sum`, so it'll have less features.
- It'll only support 2 `length`s (to avoid dynamic allocation): 4B & 8B.
- All the buffers must be as small as possible.

`lib` should have a similar API to [`BLAKE3`](https://github.com/BLAKE3-team/BLAKE3/blob/master/src/lib.rs), without all of the crypto-specific stuff.
