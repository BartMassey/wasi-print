![Maintenance](https://img.shields.io/badge/maintenance-actively--developed-brightgreen.svg)
[![CI](https://github.com/BartMassey/wasi-print/actions/workflows/rust.yml/badge.svg)](https://github.com/BartMassey/wasi-print/actions)
[![crates-io](https://img.shields.io/crates/v/wasi-print.svg)](https://crates.io/crates/wasi-print)
[![api-docs](https://docs.rs/wasi-print/badge.svg)](https://docs.rs/wasi-print)

# wasi-print: : implementation of Rust print macros and similar for no_std WASI
Bart Massey 2023 (version 0.2.0)

This crate contains basic niceties for writing `no_std`
modules for [WASI](https://wasi.dev/). `wasi-print` provides:

* An `abort()` function that raises a WASI exception.
* A `panic_handler` that aborts after trying to print panic information.
* A `print_fd()` function that prints an `&str` to a WASI `fd`.
* Printing macros `print!()`, `println!()`, `!eprint()` and `!eprintln()`.

## Example

This is a full standalone Rust WASM program using
`wasi_print`.

```rust
#![no_std]

use wasi_print::*;

#[no_mangle]
pub extern "C" fn math_add(x: i32, y: i32) -> i32 {
    eprint!("guest running math_add({}, {}) …", x, y);
    let result = x + y;
    eprintln!(" and returning {}", result);
    result
}
```

## Acknowledgments

Figuring out how to write this was made *much* easier by
this excellent [blog
post](https://dev.to/thepuzzlemaker/nostd-with-wasi-is-more-complicated-than-i-thought-it-would-be-14j7)
by "James \[Undefined\]".

## License

This work is licensed under the "MIT License". Please see the file
`LICENSE.txt` in this distribution for license terms.

## Further Acknowledgments

Thanks to the `cargo-readme` crate for generation of this `README`.
