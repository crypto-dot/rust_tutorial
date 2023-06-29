use std::io::Write;
use rand::Rng;
use std::cmp::Ordering;


fn main() {
    let random: u64 = rand::thread_rng().gen_range(1..=100);
    loop {
        print!("Input a number!");
        std::io::stdout().flush().expect("Error happened while flushing");
        let mut guess: String = String::new();
        std::io::stdin().read_line(&mut guess).expect("Unable to read line!");
        let guess_num: u64 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };
        match guess_num.cmp(&random) {
            Ordering::Less => println!("{} is less than the random number", guess),
            Ordering::Equal => {
                println!("Correct Guess!");
                break;
            }
            Ordering::Greater => println!("{} is greater than the random number", guess)
        }
    }
}
