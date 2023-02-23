// c.f.
// https://dev.to/thepuzzlemaker/nostd-with-wasi-is-more-complicated-than-i-thought-it-would-be-14j7

#![no_std]
#![no_main]

extern crate wasi;
extern crate alloc;
extern crate dlmalloc;

#[global_allocator]
static A: dlmalloc::GlobalDlmalloc = dlmalloc::GlobalDlmalloc;

fn abort() -> Result<(), wasi::Errno> {
    unsafe { wasi::proc_raise(wasi::SIGNAL_ABRT) }
}

#[panic_handler]
#[no_mangle]
pub extern "C" fn panic_handler(panic_info: &core::panic::PanicInfo) -> ! {
    let msg = alloc::format!("{}\n", panic_info);
    let _ = print(&msg);
    let _ = abort();
    loop {}
}

pub fn print(s: &str) -> Result<wasi::Size, wasi::Errno> {
    if s.len() > u32::MAX as usize {
        return Err(wasi::ERRNO_NOMEM);
    }
    let ciovecs = [wasi::Ciovec {
        buf: s.as_ptr(),
        buf_len: s.len(),
    }];
    unsafe { wasi::fd_write(1, &ciovecs) }
}
