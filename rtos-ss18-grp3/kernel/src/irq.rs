
extern crate x86_64;
use x86_64::structures::idt::*;

#[repr(C)]
pub struct InterruptDescriptorTable {
    pub divide_by_zero: Entry<HandlerFunc>,
    pub debug: Entry<HandlerFunc>,
    pub non_maskable_interrupt: Entry<HandlerFunc>,
    pub breakpoint: Entry<HandlerFunc>,
    pub overflow: Entry<HandlerFunc>,
    pub bound_range_exceeded: Entry<HandlerFunc>,
    pub invalid_opcode: Entry<HandlerFunc>,
    pub device_not_available: Entry<HandlerFunc>,
    pub double_fault: Entry<HandlerFuncWithErrCode>,
    pub invalid_tss: Entry<HandlerFuncWithErrCode>,
    pub segment_not_present: Entry<HandlerFuncWithErrCode>,
    pub stack_segment_fault: Entry<HandlerFuncWithErrCode>,
    pub general_protection_fault: Entry<HandlerFuncWithErrCode>,
    pub page_fault: Entry<PageFaultHandlerFunc>,
    pub x87_floating_point: Entry<HandlerFunc>,
    pub alignment_check: Entry<HandlerFuncWithErrCode>,
    pub machine_check: Entry<HandlerFunc>,
    pub simd_floating_point: Entry<HandlerFunc>,
    pub virtualization: Entry<HandlerFunc>,
    pub security_exception: Entry<HandlerFuncWithErrCode>,
    // some fields omitted
}



pub fn initIRQs()
{
	println!("Init IRQs");
	let mut irqtbl : &mut InterruptDescriptorTable = unsafe{
		let myTbl = 0xD00 as *mut InterruptDescriptorTable;
		&mut *myTbl
	};
	
	let mut entryOptions = irqtbl.divide_by_zero.set_handler_fn(handler);
	entryOptions.set_present(true);
	unsafe { entryOptions.set_stack_index(0); }
	entryOptions.set_privilege_level(x86_64::PrivilegeLevel::Ring3);
	
	entryOptions = irqtbl.debug.set_handler_fn(handler);
	entryOptions.set_present(true);
	unsafe { entryOptions.set_stack_index(0); }
	entryOptions.set_privilege_level(x86_64::PrivilegeLevel::Ring3);
	
	entryOptions = irqtbl.non_maskable_interrupt.set_handler_fn(handler);
	entryOptions.set_present(true);
	unsafe { entryOptions.set_stack_index(0); }
	entryOptions.set_privilege_level(x86_64::PrivilegeLevel::Ring3);
	
	entryOptions = irqtbl.breakpoint.set_handler_fn(handler);
	entryOptions.set_present(true);
	unsafe { entryOptions.set_stack_index(0); }
	entryOptions.set_privilege_level(x86_64::PrivilegeLevel::Ring3);
	
	entryOptions = irqtbl.overflow.set_handler_fn(handler);
	entryOptions.set_present(true);
	unsafe { entryOptions.set_stack_index(0); }
	entryOptions.set_privilege_level(x86_64::PrivilegeLevel::Ring3);
	
	entryOptions = irqtbl.bound_range_exceeded.set_handler_fn(handler);
	entryOptions.set_present(true);
	unsafe { entryOptions.set_stack_index(0); }
	entryOptions.set_privilege_level(x86_64::PrivilegeLevel::Ring3);
	
	entryOptions = irqtbl.invalid_opcode.set_handler_fn(handler);
	entryOptions.set_present(true);
	unsafe { entryOptions.set_stack_index(0); }
	entryOptions.set_privilege_level(x86_64::PrivilegeLevel::Ring3);
	
	entryOptions = irqtbl.device_not_available.set_handler_fn(handler);
	entryOptions.set_present(true);
	unsafe { entryOptions.set_stack_index(0); }
	entryOptions.set_privilege_level(x86_64::PrivilegeLevel::Ring3);
	
	entryOptions = irqtbl.double_fault.set_handler_fn(errorHandler);
	entryOptions.set_present(true);
	unsafe { entryOptions.set_stack_index(0); }
	entryOptions.set_privilege_level(x86_64::PrivilegeLevel::Ring3);
	
	entryOptions = irqtbl.invalid_tss.set_handler_fn(errorHandler);
	entryOptions.set_present(true);
	unsafe { entryOptions.set_stack_index(0); }
	entryOptions.set_privilege_level(x86_64::PrivilegeLevel::Ring3);
	
	entryOptions = irqtbl.segment_not_present.set_handler_fn(errorHandler);
	entryOptions.set_present(true);
	unsafe { entryOptions.set_stack_index(0); }
	entryOptions.set_privilege_level(x86_64::PrivilegeLevel::Ring3);
	
	entryOptions = irqtbl.stack_segment_fault.set_handler_fn(errorHandler);
	entryOptions.set_present(true);
	unsafe { entryOptions.set_stack_index(0); }
	entryOptions.set_privilege_level(x86_64::PrivilegeLevel::Ring3);
	
	entryOptions = irqtbl.general_protection_fault.set_handler_fn(errorHandler);
	entryOptions.set_present(true);
	unsafe { entryOptions.set_stack_index(0); }
	entryOptions.set_privilege_level(x86_64::PrivilegeLevel::Ring3);
	
	entryOptions = irqtbl.page_fault.set_handler_fn(pagefaultHandler);
	entryOptions.set_present(true);
	unsafe { entryOptions.set_stack_index(0); }
	entryOptions.set_privilege_level(x86_64::PrivilegeLevel::Ring3);
	
	entryOptions = irqtbl.x87_floating_point.set_handler_fn(handler);
	entryOptions.set_present(true);
	unsafe { entryOptions.set_stack_index(0); }
	entryOptions.set_privilege_level(x86_64::PrivilegeLevel::Ring3);
	
	entryOptions = irqtbl.alignment_check.set_handler_fn(errorHandler);
	entryOptions.set_present(true);
	unsafe { entryOptions.set_stack_index(0); }
	entryOptions.set_privilege_level(x86_64::PrivilegeLevel::Ring3);
	
	entryOptions = irqtbl.machine_check.set_handler_fn(handler);
	entryOptions.set_present(true);
	unsafe { entryOptions.set_stack_index(0); }
	entryOptions.set_privilege_level(x86_64::PrivilegeLevel::Ring3);
	
	entryOptions = irqtbl.simd_floating_point.set_handler_fn(handler);
	entryOptions.set_present(true);
	unsafe { entryOptions.set_stack_index(0); }
	entryOptions.set_privilege_level(x86_64::PrivilegeLevel::Ring3);
	
	entryOptions = irqtbl.virtualization.set_handler_fn(handler);
	entryOptions.set_present(true);
	unsafe { entryOptions.set_stack_index(0); }
	entryOptions.set_privilege_level(x86_64::PrivilegeLevel::Ring3);
	
	entryOptions = irqtbl.security_exception.set_handler_fn(errorHandler);
	entryOptions.set_present(true);
	unsafe { entryOptions.set_stack_index(0); }
	entryOptions.set_privilege_level(x86_64::PrivilegeLevel::Ring3);
	
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
