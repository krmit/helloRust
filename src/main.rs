extern crate dsfmt;

#[cfg(not(test))]
use NumberSize::{ Litle, Normal, Big};
#[cfg(not(test))]
use std::rand::{Rng, SeedableRng};
#[cfg(not(test))]
use dsfmt::DSFMTRng;

mod hello;


#[cfg(not(test))]
struct Ampere(f32);

#[cfg(not(test))]
struct Volt(f32);

#[cfg(not(test))]
struct Ohm(f32);

#[deriving(Show)]
#[cfg(not(test))]
enum NumberSize {
    Litle(i32),
    Normal(i32),
    Big(i32),
}

/// `helloRust` is a program that prints a greeting that is personalized based on
/// the arguments given and also print some more simple tests.
#[cfg(not(test))]
fn main() {
    let i = -5i;
    let u = 5u;
    let t = (1f32,"hello");
    let (x, y) = (1f32,2.5f32 );
    let y = if y == 5f32 { 10f32 } else { 15f32 };
    let l = Litle(32i32);
    let n = Normal(320i32);
    let b = Big(3200i32);
    
    let mut rng: DSFMTRng = SeedableRng::from_seed(5);
    let r = rng.next_u32();

    println!("{} {} {} {} {} {}", i,u,x,y, t.0,t.1);
    
    let batery=Volt(5f32);
	let resitor=Ohm(560f32);
	let test_amp=Ampere(0.01f32);
	
	println!("{}", batery.0);
	println!("{}", resitor.0);
	println!("{}", test_amp.0);
	
	println!("{}", l);
	println!("{}", n);
	println!("{}", b);
	
	println!("{}", r);
	
    println!("{}",hello::hello_rust(5i).ok().expect("Error when printing from hello_rust"));
}
