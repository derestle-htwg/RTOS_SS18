

pub struct frame
{
	pub next: &'static mut frame,

}

impl frame
{
	pub fn address(&self) -> u64
	{
		unsafe { self as *const frame as u64 }
	}
	pub fn take(&mut self) -> &'static mut frame
	{
		unsafe { 
			let myPtr: *mut frame = self as *mut frame;
			&mut *myPtr
		}
	}
	
	
	pub fn new(PhysicalAddress: u64) -> &'static mut frame
	{
		
		
		let myFrame = unsafe { &mut *(PhysicalAddress as *mut frame) };
		
		if(!myFrame.isNull())
		{
			myFrame.next = unsafe { &mut *(0 as *mut frame) };
		
			
		}
		myFrame
		
	}

	pub fn isNull(&self) -> bool
	{
		unsafe { let addr: usize = self as *const frame as usize; addr <= 1}
	}
	
	
}
use spin::Mutex;
use core::ops::DerefMut;
//use std::mem;


struct llFields
{
	llStart: &'static mut frame,
	llEnd: &'static mut frame
}


//Frames dürfen nicht auf 0 zeigen weil das sonst von dem Option als None Interpretiert wird und damit ein Crash passiert
lazy_static! {
   static ref myLlFields: Mutex<llFields> = { Mutex::new(llFields{ llStart: frame::new(1), llEnd: frame::new(1) })};
}
      

pub fn InsertMemoryRange(Begin: u64, End: u64)
{
	let mut start: u64 = Begin;
	let mut myEnd: u64 = End;
	start = (start + 0xFFF) & 0xFFFFFFFFFFFFF000; //Aufrunden
	myEnd = (myEnd+1) & 0xFFFFFFFFFFFFF000; // +1 und Abrunden, damit z.B. aus 2FFF 3000 als Ende wird
	
	while start < myEnd //kleiner, nicht kleinergleich damit letzte Seite nicht hinzugefügt wird.
	{
		//Check ob im kernel oder andere wichtige Speicherbereiche
		if(start >= 0) & (start <= 0x4000)
		{ 
			start = start + 0x1000;
			continue 
		}
		
		appendFrame(frame::new(start));
		start = start + 0x1000;
	}
}

pub fn appendFrame(frm: &'static mut frame)
{
	let mut myMut = &myLlFields;
	
	let mut unlockedLlFields = myMut.lock();
	
	frm.next = unlockedLlFields.llStart.take();
	unlockedLlFields.llStart = frm.take();
}

pub fn getFrame() -> Option<&'static mut frame>
{
	let mut unlockedLlFields = myLlFields.lock();
	
	let mut newLLStart: &'static mut frame = frame::new(0);
	let mut myRetVal: &'static mut frame = frame::new(0);
	
	
	
	
	//take trickst das borrow checker system aus!
	//leider notwendig für verkettete Listen auf Speicherseiten die sich selber referenzieren.
	myRetVal = unlockedLlFields.llStart.take();
	
	
	if myRetVal.isNull() {
		None
	} else {
		unlockedLlFields.llStart = myRetVal.next.take();
		Some(myRetVal)
	}
	
}
