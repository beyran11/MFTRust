fn main() {
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;

    // remainder
    let remainder = 43 % 5 ;
    let tup: (i32, f64, u8) = (500, 6.4, 1) ;

    // Array type
    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];

    let months = ["January", "February", "March", "April", "May", "June", "July",
        "August", "September", "October", "November", "December"];

    println!("remiander is :{},{}",remainder,quotient);

    another_function();

    function_with_input(11) ;

    let x = plus_one(5);

    println!("The value of x is: {}", x);
}

fn another_function() {
    println!("Another function.");
}

fn function_with_input(x:i32) {
    println!("Input is :{}",x);
}

fn plus_one(x: i32) -> i32 {
    x + 1
}