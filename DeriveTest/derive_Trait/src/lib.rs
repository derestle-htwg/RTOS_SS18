#[macro_use]
extern crate example_derive; 
use std::any::{Any, TypeId};


pub use example_derive::*;

pub trait outputStream
{
	fn sendByte(&self, data: u8);
}

pub trait Dumpable
{
	
	fn DumpObj(&self, stream: &outputStream)
	{
		println!("Default implementation");
	}
}

impl Dumpable for u8
{
	fn DumpObj(&self, stream: &outputStream){
		//println!("Dump u8: {:?}", &self);
	}
}
