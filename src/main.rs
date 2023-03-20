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
    
    matches2();
}
// Learning the different types of variables 
fn variables() {
    // Unsigned integers: u8, u16, u32, u64, u128, usize
    // Signed integers: i8, i16, i32, i64, i128, isize
    const ONE_MIL: u32 = 1_000_000;
    const PI: f32 = 3.1415926;
    let age: &str = "47";
    // This called shadowing it's perfectly legal to declare two variables that have the same name
    // but different data types.
    let mut age: u32 = age.trim().parse().expect("Age wasn't assigned a number");
    println!("I'm {} and I want ${}", age, ONE_MIL);
}

fn variables2() {
    println!("Max u32: {}", u32::MAX);

    println!("Max u64: {}", u64::MAX);

    println!("Max u128: {}", u128::MAX);
    //We can add an underscore next to a variable so the rust compiler does not throw unsused var
    //error
    let _is_true: bool = true;
}

fn random() {
    let random_num: i32 = rand::thread_rng().gen_range(1..101);
    println!("Random: {}", random_num);
}

fn conditionals() {
    let age: i32 = 18;
    if(age >= 14) && (age  < 19) {
        println!("Important Birthday");
    } if(age == 18) || (age > 42) {
        print!("Super Important!");
    }
}

fn tertinary() {
    let my_age: i32 = 47;
    let can_vote: bool = if(my_age >=18) {
    true
    } else {
    false};
    println!("{}",can_vote);
}

fn matches() {
    let age2: i32 = 8;
    match age2 {
        1..=18 => println!("Important Birthday"),
        21 | 50 => println!("Important Birthdays!"),
        65..=i32::MAX => println!("Important Birthday!"),
        _ => println!("Not Important")
        // underscore represents everything else 
        // this example is essentially if, else if, else if, else 
    }
}

fn matches2() {
    let my_age: i32= 18;
    let voting_age: i32 = 18;
    match my_age.cmp(&voting_age) {
    Ordering::Less => println!("Can't Vote"),
    Ordering::Greater => println!("You can Vote!"),
    Ordering::Equal => println!("You just gained the right to vote!!")
    }
}

