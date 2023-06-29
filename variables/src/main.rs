fn main() {
    //mut keyword allows variables to be reassigned
    let mut var = 5;
    println!("Var is {}", var);
    var = 6;
    println!("Var is {}", var);

    // f64 by default
    let test = 5.0;
    let test: f32 = 4.0;
    
    // you can shadow variables

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    //tuple with different types
    let (x, y, z) = tup;
    //destructuring

    //accessing tuples
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;
    
}
