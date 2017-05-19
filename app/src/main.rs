#![feature(core_str_ext)]
#![feature(start)]

extern crate core;
extern crate libc;
extern crate std_dep;

use core::mem;
use core::str::StrExt;

#[start]
fn start(_argc: isize, _argv: *const *const u8) -> isize {
    unsafe { libc::puts(mem::transmute("hello world\0".as_ptr())) };
    std_dep::fn_in_std_dep();
    std::fn_in_std();
    0
}
