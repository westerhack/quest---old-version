#![allow(unused)]
use quest::object;
use quest::*;

fn main() {
    simple_logger::init().unwrap();


	let o = quest::object::r#type::Number::from(0);
	let o = o.into_shared();
	println!("{:#?}", o);
}