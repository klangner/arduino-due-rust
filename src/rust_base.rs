//
// Basic functions required by Rust core library.
//
//#![feature(lang_items)]

#[lang = "panic_fmt"]
#[no_mangle]
pub extern "C" fn panic_fmt() -> ! { loop {} }

// This function is needed by arm exception handling routines.
#[no_mangle] pub extern fn __aeabi_unwind_cpp_pr0() { loop {} }
