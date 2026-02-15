# Directionlib

Direction types I grew tired of maintaining in a single `direction.rs` file in my Bevy game

There are a lot of features in this crate, mostly for slavish consistency, and my own convenience.
If there is a trait that this doesn't already implement, please tell me about it!

There should be sufficient docs in the doc comments but I don't know what else to say that isn't already there.

# Note to self

This has some panicking/unsafe code in it, to enable some const-eval stuff. Always infallible though.
Before pushing to main, run

```sh
cargo +nightly miri test --all-features
cargo clippy
```
