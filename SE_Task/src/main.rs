use std::f64;
use std::io::{self, Write};


enum Operation {

    Add(f64, f64),
    Substract(f64, f64),
    Multiply(f64, f64),
    Divide(f64, f64)
}

fn main() {

    // Prompt user for input
    let first_number = prompt_number("Enter the first number: ");
    let operator: String = prompt_operator("Enter the operation (+, -, *, /): ");
    let second_number = prompt_number("Enter the second number: ");

    // Create an Operation enum instance based on user input
    let operation = match operator.as_str() {
        "+" => Operation::Add(first_number, second_number),
        "-" => Operation::Substract(first_number, second_number),
        "*" => Operation::Multiply(first_number, second_number),
        "/" => Operation::Divide(first_number, second_number),
        _ => {
            println!("Invalid operator.");
            return;
        }
    };

    // Calculate the result and print it
    let result = calculate(operation);
    println!("Result: {}", result);

}

fn calculate(operation: Operation) -> f64 {
    match operation {
        Operation::Add(a, b) => a + b,
        Operation::Substract(a, b) => a - b,
        Operation::Multiply(a, b) => a * b,
        Operation::Divide(a, b) => {
            if b != 0.0 {
              a / b
            }else{
                panic!("Division by zero is not allowed");
            }
        }
    }
}

fn prompt_number(message: &str) -> f64 {
    print!("{}", message);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    input.trim().parse::<f64>().expect("Please enter a valid number")
}

fn prompt_operator(message: &str) -> String {
    print!("{}", message);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    input.trim().to_string()
}
