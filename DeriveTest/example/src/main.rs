
pub trait exampleTrait{
    fn hello_world(&self);
}


#[macro_use]
extern crate example_derive; 
use example_derive::*;

#[derive(Debug)]
#[derive(example)]
struct SubCakes
{
	b:i32
}

#[derive(Debug)]
#[derive(example)]
struct Pancakes
{
	a: i32,
	c: SubCakes
} 

fn main() {
    let x = Pancakes{a:1, c:SubCakes{b:0}};

    x.hello_world();
}
