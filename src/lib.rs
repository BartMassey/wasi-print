// c.f.
// https://dev.to/thepuzzlemaker/nostd-with-wasi-is-more-complicated-than-i-thought-it-would-be-14j7

#![feature(macro_metavar_expr)]

#![no_std]
#![no_main]

extern crate wasi;
extern crate alloc;
extern crate dlmalloc;

pub use alloc::format;

#[global_allocator]
static A: dlmalloc::GlobalDlmalloc = dlmalloc::GlobalDlmalloc;

macro_rules! mkprint {
    ($print:ident, $println:ident, $fd:literal) => {
        #[macro_export]
        macro_rules! $print {
            ($$fmt:literal $$(, $$args:tt)* $$(,)?) => {
                let _ = $$crate::print_fd($fd, &format!($$fmt $$(, $$args)*));
            };
        }
        #[macro_export]
        macro_rules! $println {
            ($$fmt:literal $$(, $$args:tt)* $$(,)?) => {
                $print!($$fmt $$(, $$args)*);
                $print!("\n");
            };
        }
    };
}

mkprint!(print, println, 1);
mkprint!(eprint, eprintln, 2);

pub fn abort() -> Result<(), wasi::Errno> {
    unsafe { wasi::proc_raise(wasi::SIGNAL_ABRT) }
}

#[panic_handler]
#[no_mangle]
pub extern "C" fn panic_handler(panic_info: &core::panic::PanicInfo) -> ! {
    eprintln!("{}", panic_info);
    let _ = abort();
    loop {}
}

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
