use core::{
    alloc::{GlobalAlloc, Layout},
    ffi::c_void,
};

/// [`GlobalAlloc`] that calls `malloc`/`free` from libc.
#[derive(Debug)]
pub struct LibcAlloc {}

unsafe impl GlobalAlloc for LibcAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        extern "C" {
            fn malloc(size: u32) -> *mut c_void;
        }

        // FIXME: this should use aligned alloc API from libc.
        //        see `rust/library/std/src/sys/unix/alloc.rs`
        //        for an example.
        let ptr = unsafe { malloc(layout.size() as u32) as *mut u8 };

        // Check that pointer is sufficiently aligned.
        assert_eq!(ptr as usize & (layout.align() - 1), 0);

        ptr
    }
    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        extern "C" {
            fn free(ptr: *mut c_void);
        }

        unsafe { free(ptr as *mut c_void) };
    }
}
