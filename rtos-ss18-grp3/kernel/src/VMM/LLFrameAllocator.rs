
pub struct frame
{
	pub next: Option<&'static mut frame>,
	pub me: u64,
}
use spin::Mutex;
use core::ops::DerefMut;
//use std::mem;


struct llFields
{
	llStart: Option<&'static mut frame>,
	llEnd: Option<&'static mut frame>
}

static myLlFields: Mutex<llFields> =  Mutex::new( llFields {llStart: None, llEnd: None});


impl frame
{
	pub fn new(PhysicalAddress: u64) -> &'static mut frame
	{
		let myFrame = unsafe { &mut *(PhysicalAddress as *mut frame) };
		myFrame.next = None;
		myFrame.me = PhysicalAddress;
		myFrame
	}
}

pub fn InsertMemoryRange(Begin: u64, End: u64)
{
		
}

pub fn appendFrame(frm: &mut frame)
{
	
}

pub fn getFrame() -> Option<&'static mut frame>
{
	let mut unlockedLlFields = myLlFields.lock();
	
	
	let mut newLLStart: Option<&'static mut frame> = None;
	let mut myRetVal: Option<&'static mut frame> = None;
	
	match unlockedLlFields.llStart {
		Some(ref mut x) => x.me = 0, 
		None => (),
	};
	
	//mem::replace(&mut unlockedLlFields.llStart, &mut newLLStart);

		
	/*unsafe{
		let &mut src = &(unlockedLlFields.llStart);
		src = newLLStart;
		//unlockedLlFields.llStart;
	}*/
	
	
	
	/*
	match unlockedLlFields.llStart {
		Some(ref mut frm) => {
			myRetVal = Some(frm);
			unlockedLlFields.llStart = frm.next
			
		},
		None => panic!("Kein freier Block mehr vorhanden"),
	}*/
	
	myRetVal
}
