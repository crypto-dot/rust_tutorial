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

}
