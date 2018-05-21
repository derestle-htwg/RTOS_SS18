#[macro_use]
extern crate derive_Trait; 

use derive_Trait::*;



#[derive(Debug)]
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

fn main() {
    let x = Pancakes{a:1, c:SubCakes{b:0}, e:DumpableCakes{d:3}};
    let dumpy = dumpStream{};
	x.DumpObj(&dumpy);
    
}
