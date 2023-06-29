fn main() {
    println!("Hello, world!");
    
    print_labeled_measurement(5, 's');

    print!("{}", five());
    print!("{}", fiveReturnsNothing());
}

// fn syntax
fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

// fn can only return expressions 

fn five() -> i32 {
    5
} 


// not statements

// fn fiveReturnsNothing() -> i32 {
//     5;
// } 
