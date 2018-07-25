
extern crate x86_64;
use x86_64::structures::idt::*;


pub fn initIDT()
{
	println!("Init IRQs");
	let mut irqtbl : &'static mut x86_64::structures::idt::InterruptDescriptorTable = 
	unsafe{
		let myTbl = 0x4000 as *mut x86_64::structures::idt::InterruptDescriptorTable;
		&mut *myTbl
	} ;
	irqtbl.reset();
	
	let mut entryOptions = irqtbl.divide_by_zero.set_handler_fn(handler);
	entryOptions.set_present(true);
	unsafe { entryOptions.set_stack_index(0); }
	entryOptions.set_privilege_level(x86_64::PrivilegeLevel::Ring0);
	
	entryOptions = irqtbl.debug.set_handler_fn(handler);
	entryOptions.set_present(true);
	unsafe { entryOptions.set_stack_index(0); }
	entryOptions.set_privilege_level(x86_64::PrivilegeLevel::Ring0);
	
	entryOptions = irqtbl.non_maskable_interrupt.set_handler_fn(handler);
	entryOptions.set_present(true);
	unsafe { entryOptions.set_stack_index(0); }
	entryOptions.set_privilege_level(x86_64::PrivilegeLevel::Ring0);
	
	entryOptions = irqtbl.breakpoint.set_handler_fn(handler);
	entryOptions.set_present(true);
	unsafe { entryOptions.set_stack_index(0); }
	entryOptions.set_privilege_level(x86_64::PrivilegeLevel::Ring0);
	
	entryOptions = irqtbl.overflow.set_handler_fn(handler);
	entryOptions.set_present(true);
	unsafe { entryOptions.set_stack_index(0); }
	entryOptions.set_privilege_level(x86_64::PrivilegeLevel::Ring0);
	
	entryOptions = irqtbl.bound_range_exceeded.set_handler_fn(handler);
	entryOptions.set_present(true);
	unsafe { entryOptions.set_stack_index(0); }
	entryOptions.set_privilege_level(x86_64::PrivilegeLevel::Ring0);
	
	entryOptions = irqtbl.invalid_opcode.set_handler_fn(handler);
	entryOptions.set_present(true);
	unsafe { entryOptions.set_stack_index(0); }
	entryOptions.set_privilege_level(x86_64::PrivilegeLevel::Ring0);
	
	entryOptions = irqtbl.device_not_available.set_handler_fn(handler);
	entryOptions.set_present(true);
	unsafe { entryOptions.set_stack_index(0); }
	entryOptions.set_privilege_level(x86_64::PrivilegeLevel::Ring0);
	
	entryOptions = irqtbl.double_fault.set_handler_fn(errorHandler);
	entryOptions.set_present(true);
	unsafe { entryOptions.set_stack_index(0); }
	entryOptions.set_privilege_level(x86_64::PrivilegeLevel::Ring0);
	
	entryOptions = irqtbl.invalid_tss.set_handler_fn(errorHandler);
	entryOptions.set_present(true);
	unsafe { entryOptions.set_stack_index(0); }
	entryOptions.set_privilege_level(x86_64::PrivilegeLevel::Ring0);
	
	entryOptions = irqtbl.segment_not_present.set_handler_fn(errorHandler);
	entryOptions.set_present(true);
	unsafe { entryOptions.set_stack_index(0); }
	entryOptions.set_privilege_level(x86_64::PrivilegeLevel::Ring0);
	
	entryOptions = irqtbl.stack_segment_fault.set_handler_fn(errorHandler);
	entryOptions.set_present(true);
	unsafe { entryOptions.set_stack_index(0); }
	entryOptions.set_privilege_level(x86_64::PrivilegeLevel::Ring0);
	
	entryOptions = irqtbl.general_protection_fault.set_handler_fn(errorHandler);
	entryOptions.set_present(true);
	unsafe { entryOptions.set_stack_index(0); }
	entryOptions.set_privilege_level(x86_64::PrivilegeLevel::Ring0);
	
	entryOptions = irqtbl.page_fault.set_handler_fn(pagefaultHandler);
	entryOptions.set_present(true);
	unsafe { entryOptions.set_stack_index(0); }
	entryOptions.set_privilege_level(x86_64::PrivilegeLevel::Ring0);
	
	entryOptions = irqtbl.x87_floating_point.set_handler_fn(handler);
	entryOptions.set_present(true);
	unsafe { entryOptions.set_stack_index(0); }
	entryOptions.set_privilege_level(x86_64::PrivilegeLevel::Ring0);
	
	entryOptions = irqtbl.alignment_check.set_handler_fn(errorHandler);
	entryOptions.set_present(true);
	unsafe { entryOptions.set_stack_index(0); }
	entryOptions.set_privilege_level(x86_64::PrivilegeLevel::Ring0);
	
	entryOptions = irqtbl.machine_check.set_handler_fn(handler);
	entryOptions.set_present(true);
	unsafe { entryOptions.set_stack_index(0); }
	entryOptions.set_privilege_level(x86_64::PrivilegeLevel::Ring0);
	
	entryOptions = irqtbl.simd_floating_point.set_handler_fn(handler);
	entryOptions.set_present(true);
	unsafe { entryOptions.set_stack_index(0); }
	entryOptions.set_privilege_level(x86_64::PrivilegeLevel::Ring0);
	
	entryOptions = irqtbl.virtualization.set_handler_fn(handler);
	entryOptions.set_present(true);
	unsafe { entryOptions.set_stack_index(0); }
	entryOptions.set_privilege_level(x86_64::PrivilegeLevel::Ring0);
	
	entryOptions = irqtbl.security_exception.set_handler_fn(errorHandler);
	entryOptions.set_present(true);
	unsafe { entryOptions.set_stack_index(0); }
	entryOptions.set_privilege_level(x86_64::PrivilegeLevel::Ring0);
	
	//init IDTPtr. soll für Andere CPUs im Bootprozess verwendet werden. Deshalb nicht load Funktion.
	let mut baseAdress: &mut x86_64::instructions::tables::DescriptorTablePointer =  (unsafe { &mut (*(0xFA0 as *mut x86_64::instructions::tables::DescriptorTablePointer)) });
	baseAdress.limit = 4096;
	baseAdress.base = 0x4000;
	unsafe {x86_64::instructions::tables::lidt(baseAdress); }
	
	
	
}

extern "x86-interrupt" fn handler(_: &mut ExceptionStackFrame)
{
		println!("Handler");
		loop{}
}

extern "x86-interrupt" fn errorHandler(_: &mut ExceptionStackFrame, error_code: u64)
{
		println!("Handler Wit error");
		loop{}
}

extern "x86-interrupt" fn pagefaultHandler(_: &mut ExceptionStackFrame, error_code: PageFaultErrorCode)
{
	println!("pagefault");
	loop{}
}
