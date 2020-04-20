// use std::io;
// use std::fmt::Display;
use rand::Rng;

fn main() {
    println!("Hello, world!");
    let lucky_number = rand::thread_rng().gen_range(1, 101);
    println!("Lucky Number : {}",lucky_number);
}
