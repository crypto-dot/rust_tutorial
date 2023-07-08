fn main() {

    // below is an infinite loop
    // loop {
    //     println!("again!");
    // }

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter > 4 {
            break counter * 2;
        }
    };

    println!("result is {result}");
    // loop labels
    // loop labels allow you to use continue and break on the labeled loop rather than the inner most loop

    let mut count = 0;

    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;
        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");


    let mut num = 5;
    while num < 10 {
        println!("{num}!");
        num += 1;
    }

    let a = [3,2,15,6];
    let mut index = 0;

    while index < 4 {
        println!("the value of a at index {index} is {}", a[index]);
        index += 1;
    }

    for element in a {
        println!("the value is: {element}");
    }

    // rev method reverses range
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!");
 
}
