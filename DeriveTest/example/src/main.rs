#[macro_use]
extern crate derive_Trait; 

use derive_Trait::*;
use std::any::Any;


//#[derive(Debug)]
//#[derive(example)]
struct SubCakes
{
	b:i32
}

#[derive(Debug)]
#[derive(example)]
struct DumpableCakes
{
	d:i32
}

#[derive(Debug)]
#[derive(example)]
struct Pancakes
{
	a: i32,
	c: SubCakes,
	e: DumpableCakes
} 

struct dumpStream{}

impl outputStream for dumpStream
{
	fn sendByte(&self, data: u8){
		println!("Dump: {:?}", data);
	}
}


/*
fn dump<T: Any>(inVar: &T, stream: &outputStream) 
{
	let value_any = inVar as &Any;
	match value_any.downcast_ref::<Dumpable>() {
		Some(as_Dumpable) => {
			as_Dumpable.DumpObj(stream);
		}
		None => {
			println!("none");
		}
	}
}*/


fn main() {
    let x = Pancakes{a:1, c:SubCakes{b:0}, e:DumpableCakes{d:3}};
    let dumpy = dumpStream{};
	//x.DumpObj(&dumpy);
    
    let tmp: u8 = 5;
    
    
    x.DumpObj(&dumpy);
    println!("{:?}",x);
    //x.c.DumpObj(&dumpy);
}
