use std::io::Write;
use rand::Rng;
use std::cmp::Ordering;
fn main() {
    let random: u32 = rand::thread_rng().gen_range(1..=100);
    loop {
        print!("Input your guess (between 1 and 100 inclusive):");
        std::io::stdout().flush().expect("An error occurred flushing");
        let mut guess: String = String::new();
        std::io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };
        match guess.cmp(&random) {
            Ordering::Less => println!("{} is less than the secret number!", guess),
            Ordering::Equal => {
                println!("Correct guess!");
                break;
            },
            Ordering::Greater => println!("{} is greater than the secret number!", guess)
        }
    }
}
