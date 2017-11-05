# Luajit RS [![Build Status](https://travis-ci.org/Dreae/luajit-rs.svg?branch=master)](https://travis-ci.org/Dreae/luajit-rs)

[Documentation](https://dreae.github.io/luajit-rs/luajit)

Crate for interfacing with LuaJIT from Rust, for running high-performance Lua code that
can integrate with native-code written in rust.

## Getting Started

```rust
extern crate luajit;

use luajit::State;

pub fn main() {
    let mut state = State::new();
    state.open_libs();
    state.do_string(r#"print("Hello world!")"#);
}
```