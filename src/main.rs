use std::io;

fn main() {
    let mut input_sign = String::new();
    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("Calculator");
    println!("Input multi, divide, plus or minus");

    while input_sign.trim() != "multi" && input_sign.trim() != "divide" && input_sign.trim() != "plus" && input_sign.trim() != "minus" {
        input_sign.clear();
        io::stdin().read_line(&mut input_sign).expect("Failed to read line");

        if input_sign.trim() != "multi" && input_sign.trim() != "divide" && input_sign.trim() != "plus" && input_sign.trim() != "minus" {
            println!("Invalid sign, try again:");
        }
    }

    println!("Enter first number: ");
    io::stdin().read_line(&mut input1).expect("Failed to read line");

    println!("Enter second number: ");
    io::stdin().read_line(&mut input2).expect("Failed to read line");

    // Convert inputs from strings to numbers
    let input1: f64  = input1.trim().parse().expect("Not a number");
    let input2: f64 = input2.trim().parse().expect("Not a number");

    let result = if input_sign.trim() == "multi" {
        input1 * input2
    } else if input_sign.trim() == "divide" {
        input1 / input2
    } else if input_sign.trim() == "plus" {
        input1 + input2
    } else {
        input1 - input2
    };

    println!("The result is: {}", result);
}
