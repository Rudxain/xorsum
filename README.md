# About this branch

This is meant for the development of [lib.rs](src/lib.rs), because any change to it would be a breaking change (the API is **very unstable**, currently). This branch allows us to edit the lib without the need to increment the Major-semver-number.

When the API stabilises, we'll merge this branch into `main`, increase the Minor-semver-number, and publish a new release to crates.io and a new GH-tag.

## CONTRIBUTING

The owner (me) has no idea what he's doing, LMAO. So please, if you have any suggestion about how to shape the API, its layout, accepted types, single-generic vs multiple-fns, etc... please let us know. We're taking some inspiration from BLAKE3, but most of its API serves an entirely different purpose (keys, HMAC, and some more crypto stuff), so doing a naive copy-paste is very unhelpful
