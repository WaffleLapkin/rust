use core::{ffi, panic};

use crate::libc_alloc;

/// The global allocator.
#[global_allocator]
pub static GLOBAL_ALLOCATOR: libc_alloc::LibcAlloc = libc_alloc::LibcAlloc {};

#[lang = "start"]
fn start<T>(main: fn() -> T, _: isize, _: *const *const u8, _: u8) -> isize {
    _ = main();
    0
}

/* #[panic_handler]
fn panic_handler(info: &panic::PanicInfo<'_>) -> ! {
    _ = eprintln!("program {info}");

    extern "C" {
        fn exit(exit_code: ffi::c_int) /*-> !*/;
    }

    // Tell the symulator we want to exit, actually, and not loop forever.
    unsafe { exit(-1) }

    loop {}
}
 */
