use VMM;
use VMM::LLFrameAllocator::*;
use scheduler;
pub fn getApicID() -> u64
{
	let mut myID : &mut u64 = unsafe { &mut *(0xFEE00020 as *mut u64)};
	*myID
}

pub fn initSMP(){
	
	let mut myRustVector : &mut u64 = unsafe { &mut *(0xCF8 as *mut u64)};
	unsafe {
		let myFNPtr : *const fn(u64) = &(AP_Entry as fn(u64));
		*myRustVector = *myFNPtr as u64;
	};
	let mut myStackVector : &mut u64 = unsafe { &mut *(0xCF0 as *mut u64)};
	*myStackVector = 0x200000; //noch per alloc holen
	//erst init, danach Startup
	let vector: u8 = 0; //Vector auf welchen 4kb sector die Initroutine beginnt
	let mut myICR : &mut u64 = unsafe { &mut *(0xFEE00300 as *mut u64)}; //Command register im LAPIC
	let mut initCommand: u64 = (3 << 18) + 0 + (1<<14) + (1<<11) + (5<<8) ; /*init: all Excluding self, edge, assert, logical, init*/
	let mut startCommand: u64 = (2<<32) + (3 << 18) + 0 + (1<<14) + (1<<11) + (6<<8) + (vector as u64);//Startup: all Excluding self, edge, assert, logical, startup
	*myICR = initCommand; //Setzt alle anderen CPUs in den RESET Modus und hält sie dort
	*myICR = startCommand; //lässt alle anderen CPUs den Code bei 0 ausführen
}

pub fn AP_Entry(bootparam: u64)
{
	let mut followCPU_Stack : &mut u64 = unsafe { &mut *(0xCF0 as *mut u64)};
	*followCPU_Stack = bootparam + 0x2000;
	
	println!("test! {:x}", getApicID());
	if ((getApicID()>>24) & 0xFF) == 1 
	{ // 2. Thread
		CPU2();
	}
	loop{};
}

fn CPU2()
{
	//Dieser Thread soll einen eigenen Addressraum haben.
	let CPU2_CR3 : &mut VMM::PML4 = unsafe { &mut *(VMM::LLFrameAllocator::getFrame().expect("CPU2 CR3") as *mut VMM::LLFrameAllocator::frame as u64 as *mut VMM::PML4 )};
	CPU2_CR3.zero();
	// VGABuf, funktion, stack
	let stack : &mut frame = VMM::LLFrameAllocator::getFrame().expect("CPU2 Stack");
	let vgaBuf = VMM::LLFrameAllocator::frame::new(0xb8000);
	// eigener Thread im privaten Addressraum
	let mut myfnVec:u64 = 0;
	unsafe {
		let myFNPtr : *const fn() = &(privateAddressFN as fn());
		myfnVec = *myFNPtr as u64;
	}
	let fnMem1 = VMM::LLFrameAllocator::frame::new((myfnVec & 0xFFFFFFFFFFFFF000));
	let fnMem2 = VMM::LLFrameAllocator::frame::new(((myfnVec + 0x1000) & 0xFFFFFFFFFFFFF000));
	//Da da alignment der Funktion nicht gesichert ist wird einfach die Darauffolgende Speicherseite mit gemappt
	
	VMM::allocator::mapAddresspace(stack, 0xFFFF800000000000, CPU2_CR3);
	VMM::allocator::mapAddresspace(vgaBuf, 0xFFFF800000003000, CPU2_CR3);
	VMM::allocator::mapAddresspace(fnMem1, 0xFFFF800000001000, CPU2_CR3);
	VMM::allocator::mapAddresspace(fnMem2, 0xFFFF800000002000, CPU2_CR3);
	
	let myThread = scheduler::Thread {cr3: 0, rsp:  0xFFFF800000000000};
	myfnVec = myfnVec & 0x0FFF;
	myfnVec = myfnVec | 0xFFFF800000001000;
	scheduler::start(&myThread, myfnVec);
}	

fn privateAddressFN()
{
	let mut i = 0;
	loop{
		i = i+1;
		i = i-1;
		};
}
