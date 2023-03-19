#![allow(unused)]
use std::io;
use rand::Rng;
use std::io::{Write, ErrorKind,  BufReader, BufRead};
use std::fs::File;
use std::cmp::Ordering;

fn main() {
    let mut name = String::new();   
    let greeting = "Hello";
    print!("Input your name: "); 
    io::stdout().flush();
    // print typically requires a flush afterwards since it is buffered to memory
    io::stdin().read_line(&mut name).expect("Didn't receive input");
    println!("Hello, {}! {}", name.trim(), greeting);
    variables();
}
// Learning the different types of variables 
fn variables() {
    const ONE_MIL: u32 = 1_000_000;
    const PI: f32 = 3.1415926;
    let age: &str = "47";
    let mut age: u32 = age.trim().parse().expect("Age wasn't assigned a number");
    println!("I'm {} and I want ${}", age, ONE_MIL);
}
