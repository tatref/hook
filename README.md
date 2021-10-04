# What is this?
A Rust library / macro to help with the [`LD_PRELOAD` trick](http://www.goldsborough.me/c/low-level/kernel/2016/08/29/16-48-53-the_-ld_preload-_trick/)

# Setup
```
$ cargo new --lib foo/
$ cd foo/
$ git clone https://github.com/tatref/rust-hook
```

Add to `Cargo.toml`:
```
[dependencies]
"hook" = { path = "hook" }
libc = "*"

[lib]
```

Code your own lib, or copy examples
```
$ cp hook/examples/simple.rs src/lib.rs
```

# Usage
```
$ cargo build --release
$ ls -l target/release/libfoo.so 
-rwxrwxr-x 2 yann yann 3398952 Oct  5 00:24 target/release/libfoo.so
$ LD_PRELOAD=./target/release/libfoo.so ls
[...]
We got: "Cargo.toml"
We got: "src"
We got: "target"
```