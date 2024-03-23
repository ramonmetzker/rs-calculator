use std::io;

fn main() {
    println!("Enter an expression (e.g., 1 + 6):");

    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();

    // let tokens: Vec<&str>  ---- new variable holding a vector of string slices
    // & is a reference operator, which allows us to refer to a value without taking ownership of it
    let tokens: Vec<&str> = input.trim().split(' ').collect();

    if tokens.len() != 3 {
        println!("Invalid format!");
        return;
    }

    let num1 = tokens[0].parse::<f64>().unwrap_or(0.0);
    let num2 = tokens[2].parse::<f64>().unwrap_or(0.0);
    let operator = tokens[1];

    match operator {
        "+" => println!("Result: {}", num1 + num2),
        "-" => println!("Result: {}", num1 - num2),
        "*" => println!("Result: {}", num1 * num2),
        "/" => {
            if num2 == 0.0 {
                println!("Division by zero is not allowed!");
            } else {
                println!("Result: {}", num1 / num2);
            }
        }
        _ => println!("Invalid operator!"),
    }
}
