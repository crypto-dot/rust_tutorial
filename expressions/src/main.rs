fn main() {
    

    // if expressions syntax
    let number = 3;
    if number < 5 {
        print!("{number}");
    } 
    else {
        println!("the number is greater than 5 ! {number}");
    }

    // multi level syntax
    //    if number % 4 == 0 {
    //     println!("number is divisible by 4");
    // } else if number % 3 == 0 {
    //     println!("number is divisible by 3");
    // } else if number % 2 == 0 {
    //     println!("number is divisible by 2");
    // } else {
    //     println!("number is not divisible by 4, 3, or 2");
    // }


    let someCondition = true;
    // in line boolean expression
    let number2 = if someCondition {5} else {2};
    
    // variables must be the same type when using inline boolean expressions the following is not correct
    // let number = if condition { 5 } else { "six" };


}
