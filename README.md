# About this branch

This is meant for the development of [lib.rs](src/lib.rs), because any change to it would be a breaking change (the API is **very unstable** now). This branch allows us to edit the lib without the need to increment the Major-semver-number.

When the API stabilises, I'll merge this branch into `main`, increase the Minor-semver-number, and publish a new release to crates.io and a new GH-tag
