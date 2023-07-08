use std::io::Write;

fn main() {
    loop {
    print!("Please input nth digit of the fibonnacci sequence (hint 0th is the first digit)");
    std::io::stdout().flush().expect("Error flushing console");
    let mut input: String = String::new();
    std::io::stdin().read_line(&mut input).expect("Unable to read line!");
    let numInput : i64 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => continue
    };
    println!("The {} number of the fibonnacci sequence is {}", input.trim(),fibonacci(numInput));
    break;
    }
}


fn fibonacci(num: i64 ) -> i64 {
    let mut i = 0;
    let mut a = 1;
    let mut b = 0;
    let mut c = 0;
    while i < num {
        b = a;
        a = c;
        c = a + b;
        i += 1;
    }
    c
}