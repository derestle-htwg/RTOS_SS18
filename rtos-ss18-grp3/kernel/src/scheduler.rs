#![feature(asm)]


pub struct Thread
{
	pub rsp: u64,
	pub cr3: u64
}

pub fn registerThread(newThread: &fn(u64))
{
	
}

pub fn _yield()
{
	
	unsafe{
		asm!( "hlt")
	}
}

fn wait()
{
	unsafe{
		asm!( "hlt")
	}
}

pub fn start(next: &Thread, rip: u64)
{
	unsafe{
		asm!( "
				mov %rdx,%rsp
				mov	%rax,%cr3
				push %rbx
				ret"
			:: "{rdx}"(next.rsp), "{rax}"(next.cr3), "{rbx}"(rip)
			);
	}
}

pub fn dispatch(next: &Thread)
{
	unsafe{
		asm!( "
				mov %rdx,%rsp
				mov	%cr3,%rax"
			:: "{rdx}"(next.rsp), "{rax}"(next.cr3)
			);
	}
	
}
