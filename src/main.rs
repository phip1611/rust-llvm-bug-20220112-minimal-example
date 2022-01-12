#![no_std]
#![no_main]
// see https://docs.rust-embedded.org/embedonomicon/smallest-no-std.html
#![feature(lang_items)]
// to use custom allocator
// #![feature(default_alloc_error_handler)]
// default_alloc_error_handler makes links errors ("rust_oom not found")
// We just use our own/custom error handler.
#![feature(alloc_error_handler)]
// required to access ".message()" on PanicInfo
#![feature(panic_info_message)]
#![deny(missing_debug_implementations)]

core::arch::global_asm!(include_str!("start.S"));
core::arch::global_asm!(include_str!("multiboot2_header.S"));

// ONLY USE ALLOCATIONS WHEN AN ALLOCATOR WAS SET UP!
#[allow(unused)]
#[macro_use]
extern crate alloc;

use core::alloc::{GlobalAlloc, Layout};


/// This symbol is referenced in "start.S". It doesn't need the "pub"-keyword,
/// because visibility is a Rust feature and not important for the object file.
#[no_mangle]
fn entry_rust(_: u32, _: u32) -> ! {
    // actually compiles
    let _ = hashbrown::HashMap::<f32, f32>::new();
    // breaks build with: LLVM ERROR: Do not know how to split the result of this operator!
    let _ = fontdue::Font::from_bytes([0], fontdue::FontSettings::default());
    loop {}
}

struct GlobalAllocator;
#[global_allocator]
static ALLOC: GlobalAllocator = GlobalAllocator;

unsafe impl GlobalAlloc for GlobalAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        todo!()
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        todo!()
    }
}

#[panic_handler]
fn panic_handler(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[alloc_error_handler]
fn alloc_error_handler(_: Layout) -> ! {
    panic!()
}

// see https://docs.rust-embedded.org/embedonomicon/smallest-no-std.html
#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

