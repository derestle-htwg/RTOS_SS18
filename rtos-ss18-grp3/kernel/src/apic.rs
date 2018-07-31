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
	loop{};
}
