# gobuild
A library to compile go code into a Rust library/application.
This library takes inspiration from the `cc` crate.

# Using gobuild
First, you'll want to both add a build script for your crate (build.rs) and also add this crate to your Cargo.toml via:

```toml
[build-dependencies]
gobuild = {git = "https://github.com/zilong-dai/gobuild.git", rev = "main" }
```

Next, update the `build.rs` to something like:

```rust
// build.rs

fn main() {
    gobuild::Build::new()
        .file("hello.go")
        .compile("hello");
}
```

This will produce a `libhello.h` and `libhello.a` in `OUT_DIR`.
