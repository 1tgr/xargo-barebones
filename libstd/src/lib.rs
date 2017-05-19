#![no_std]

#![feature(core_intrinsics)]
#![feature(lang_items)]

extern crate std_dep;

use core::intrinsics;

pub fn fn_in_std() {
    std_dep::fn_in_std_dep()
}

pub mod prelude {
    pub mod v1 {
    }
}

#[lang = "eh_personality"]
#[no_mangle]
pub extern fn rust_eh_personality() {
}

#[lang = "eh_unwind_resume"]
#[no_mangle]
pub extern fn rust_eh_unwind_resume() {
}

#[lang = "panic_fmt"]
#[no_mangle]
pub extern fn rust_begin_panic(_msg: core::fmt::Arguments,
                               _file: &'static str,
                               _line: u32) -> ! {
    unsafe { intrinsics::abort() }
}
