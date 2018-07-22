#![feature(abi_x86_interrupt)]
#![feature(const_let)]
#![feature(lang_items)] // required for defining the panic handler
#![feature(global_allocator)]
#![feature(const_fn)]
#![feature(alloc)]
#![feature(allocator_api)]
#![feature(panic_implementation)]
#![no_std]
#![no_main] // disable all Rust-level entry points
// src/main.rs
extern crate volatile;
extern crate x86_64;

#[macro_use]

mod vga_buffer;
mod VMM;
use core::fmt::Arguments;
use vga_buffer::*;
extern crate spin;
#[macro_use]
extern crate lazy_static;
extern crate multiboot2;
mod bootInformation;
mod irq;



use core::panic::PanicInfo;
#[no_mangle]
#[panic_implementation]
fn panic(_info: &PanicInfo) -> ! {
	println!("Panic :{:?}", _info);
    loop {}
}


pub unsafe fn exit_qemu() -> ! {
    use x86_64::instructions::port::Port;

    let mut port = Port::<u32>::new(0xf4);
    port.write(0);
    loop{}
}

#[no_mangle]
pub extern "C" fn _start(bootinfo: &u64) -> ! {
	println!("Hello World!");
	bootInformation::parse(bootinfo);
	
	
	
	let a = 0;
	let b = 1;
	
		
	irq::initIRQs();
	
	println!("{}", unsafe{*(0x1234567812345678 as *const u8)});
	
	unsafe {exit_qemu()}
	
	

}


//kopiert und angepasst von 
//https://source.that.world/source/rux/browse/master/selfalloc/src/lib.rs




extern crate alloc;
const PAGE_LENGTH: usize = 4096;

static ALLOCATOR: WatermarkAllocator = WatermarkAllocator {watermark: 0};

struct WatermarkAllocator {
	watermark: usize,
}

pub unsafe fn setup_allocator() {
	
}
	
// http://os.phil-opp.com/kernel-heap.html#alignment

/// Align downwards. Returns the greatest x with alignment `align`
/// so that x <= addr. The alignment must be a power of 2.
pub fn align_down(addr: usize, align: usize) -> usize {
	0
}

/// Align upwards. Returns the smallest x with alignment `align`
/// so that x >= addr. The alignment must be a power of 2.
pub fn align_up(addr: usize, align: usize) -> usize {
	0
}

impl WatermarkAllocator {
	pub fn new() -> Self {
		
		WatermarkAllocator {
			watermark: 0,
		}
	}

	pub fn allocate(&mut self, size: usize, align: usize) -> *mut u8 {
		unsafe{0 as *mut u8}
	}
}

#[global_allocator]
static WATER_ALLOCATOR: WaterAlloc = WaterAlloc;

use core::alloc::Layout;
use core::alloc::{GlobalAlloc};

struct WaterAlloc;

unsafe impl<'a> GlobalAlloc for WaterAlloc {
	
	unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
		unsafe {0 as _}
	}

	unsafe fn dealloc(&self, _pointer: *mut u8, _layout: Layout) { }
}

#[lang="oom"]
#[no_mangle]
pub fn rust_oom(layout: Layout) -> ! {
	panic!("Out of memory");
}
