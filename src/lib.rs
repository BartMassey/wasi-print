#![doc(html_root_url = "https://docs.rs/wasi-print/0.2.1")]
#![feature(macro_metavar_expr)]
#![feature(doc_cfg)]
#![no_std]

/*!
This crate contains basic niceties for writing `no_std`
modules for [WASI](https://wasi.dev/). `wasi-print` provides:

* An `abort()` function that raises a WASI exception.
* A `panic_handler` that aborts after trying to print panic information.
* A `print_fd()` function that prints an `&str` to a WASI `fd`.
* Printing macros `print!()`, `println!()`, `!eprint()` and `!eprintln()`.

# Example

This is a full standalone Rust WASM program using
`wasi_print`.

```ignore
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

# Features

* `print`: Include printing code. This requires nightly for
  a variety of reasons.
* `panic_handler`: Provide a panic handler.

# Notes

When used without the `print` feature, this crate can be
built on stable Rust.

# Acknowledgments

Figuring out how to write this was made *much* easier by
this excellent [blog
post](https://dev.to/thepuzzlemaker/nostd-with-wasi-is-more-complicated-than-i-thought-it-would-be-14j7)
by "James \[Undefined\]".

# License

This work is licensed under the "MIT License". Please see the file
`LICENSE.txt` in this distribution for license terms.
*/

#[cfg(feature = "print")]
extern crate alloc;
#[cfg(feature = "print")]
extern crate dlmalloc;
#[cfg(feature = "print")]
extern crate wasi;

#[cfg(feature = "print")]
#[doc(cfg(feature = "print"))]
pub use alloc::format;

#[cfg(feature = "print")]
#[global_allocator]
static A: dlmalloc::GlobalDlmalloc = dlmalloc::GlobalDlmalloc;

#[cfg(feature = "print")]
macro_rules! mkprint {
    ($print:ident, $println:ident, $fd:literal) => {
        /// Print to stdio without a trailing newline.
        #[doc(cfg(feature = "print"))]
        #[macro_export]
        macro_rules! $print {
            ($$fmt:literal $$(, $$args:tt)* $$(,)?) => {
                let _ = $$crate::print_fd($fd, &format!($$fmt $$(, $$args)*));
            };
        }
        /// Print to stdio with a trailing newline.
        #[doc(cfg(feature = "print"))]
        #[macro_export]
        macro_rules! $println {
            ($$fmt:literal $$(, $$args:tt)* $$(,)?) => {
                $print!($$fmt $$(, $$args)*);
                $print!("\n");
            };
        }
    };
}

#[cfg(feature = "print")]
mkprint!(print, println, 1);

#[cfg(feature = "print")]
mkprint!(eprint, eprintln, 2);

/// Attempt to terminate the current execution by raising a
/// WASI `ABRT` exception. This function should not return:
/// if it does, it will return an error indicating why it
/// failed to terminate.
pub fn abort() -> Result<(), wasi::Errno> {
    unsafe { wasi::proc_raise(wasi::SIGNAL_ABRT) }
}

#[cfg(feature = "panic-handler")]
#[doc(cfg(feature = "panic-handler"))]
/// Handle a `panic()` in a WASI-compatible way.
#[panic_handler]
#[no_mangle]
pub extern "C" fn panic_handler(_panic_info: &core::panic::PanicInfo) -> ! {
    #[cfg(feature = "print")]
    eprintln!("{}", _panic_info);
    let _ = abort();
    loop {}
}

#[cfg(feature = "print")]
#[doc(cfg(feature = "print"))]
/// Print the text of `s` to the WASI file descriptor `fd`.
pub fn print_fd(fd: u32, s: &str) -> Result<wasi::Size, wasi::Errno> {
    if s.len() > u32::MAX as usize {
        return Err(wasi::ERRNO_NOMEM);
    }
    let ciovecs = [wasi::Ciovec {
        buf: s.as_ptr(),
        buf_len: s.len(),
    }];
    unsafe { wasi::fd_write(fd, &ciovecs) }
}
