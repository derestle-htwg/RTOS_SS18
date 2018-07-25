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
	
		
	irq::initIDT();
	
	//TSS
	let mut myTSS : &mut x86_64::structures::tss::TaskStateSegment = unsafe { &mut *(0xF00 as *mut x86_64::structures::tss::TaskStateSegment)};
	
	myTSS.privilege_stack_table[0] = x86_64::VirtAddr::new(0x200000);
	myTSS.privilege_stack_table[1] = x86_64::VirtAddr::new(0x210000);
	myTSS.privilege_stack_table[2] = x86_64::VirtAddr::new(0x220000);
	myTSS.interrupt_stack_table[0] = x86_64::VirtAddr::new(0x230000);
	myTSS.interrupt_stack_table[1] = x86_64::VirtAddr::new(0x240000);
	myTSS.interrupt_stack_table[2] = x86_64::VirtAddr::new(0x250000);
	myTSS.interrupt_stack_table[3] = x86_64::VirtAddr::new(0x260000);
	myTSS.interrupt_stack_table[4] = x86_64::VirtAddr::new(0x270000);
	myTSS.interrupt_stack_table[5] = x86_64::VirtAddr::new(0x280000);
	myTSS.interrupt_stack_table[6] = x86_64::VirtAddr::new(0x290000);
	
	
	let mut myGDT : &'static mut [u64; 8] = unsafe { &mut  *(0xFC0 as *mut [u64; 8])};
	myGDT[0] = 0;
	myGDT[1] = (1<<53) + (1<<47) + (1<<44) + (1<<43) + (1<<42); //Code 64
	myGDT[2] = (1<<47) + (1<<44);
	myGDT[3] = 0;
	myGDT[4] = 0;
	myGDT[5] = 128 + (0xF00<<16) + (9 << 40) + (1<<47);
	myGDT[6] = 0;
	myGDT[7] = 0;
	
	let mut myGDTPtr : &mut x86_64::instructions::tables::DescriptorTablePointer = unsafe { &mut *(0xFB0 as *mut x86_64::instructions::tables::DescriptorTablePointer)};//x86_64::instructions::tables::DescriptorTablePointer
	myGDTPtr.limit = 0x40;
	myGDTPtr.base = 0xFC0;
	
	let mut myIDTPtr : &mut x86_64::instructions::tables::DescriptorTablePointer = unsafe { &mut *(0xFA0 as *mut x86_64::instructions::tables::DescriptorTablePointer)};//x86_64::instructions::tables::DescriptorTablePointer
	myIDTPtr.limit = 0x1000;
	myIDTPtr.base = 0x4000;
	
	unsafe 
	{
		x86_64::instructions::tables::lgdt(&myGDTPtr);
		
		x86_64::instructions::tables::load_tss(x86_64::structures::gdt::SegmentSelector::new(5,  x86_64::PrivilegeLevel::Ring0));
	}
	println!("{}", unsafe{*(0x1234567812345678 as *const u8)});
	
	unsafe {exit_qemu()}
	
	

}


//kopiert und angepasst von 
//https://source.that.wo





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
