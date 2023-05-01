use std::io::Write;
fn main() {
    prnt!("Input your number:");
    std::io::stdout().flush().expect("An error occurred flushing");
    let mut input: String = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read the line");
    println!("You guessed {}", input);
}
