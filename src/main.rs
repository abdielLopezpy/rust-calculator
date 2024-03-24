use std::io::{self, Write};

fn main() {
    println!("---------------------Welcome---------------------\n");

    loop {
        print!("Enter the operation to perform (operator operand1 operand2): ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let parts: Vec<&str> = input.trim().split_whitespace().collect();

        if parts.len() != 3 {
            println!("Error: You must enter a valid operation.");
            continue;
        }

        let operator = parts[0];
        let operand1 = parts[1];
        let operand2 = parts[2];

        if operator != "+" && operator != "-" && operator != "*" && operator != "/" {
            println!("Error: Invalid operator.");
            continue;
        }

        if !is_valid_operand(operand1) || !is_valid_operand(operand2) {
            println!("Error: One or both operands are not valid.");
            continue;
        }

        // Perform the operation based on the operator
        match operator {
            "+" => println!(
                "Addition of {} + {} = {}",
                operand1,
                operand2,
                add(operand1, operand2)
            ),
            "-" => println!(
                "Subtraction of {} - {} = {}",
                operand1,
                operand2,
                sub(operand1, operand2)
            ),
            "*" => println!(
                "Multiplication of {} * {} = {}",
                operand1,
                operand2,
                mul(operand1, operand2)
            ),
            "/" => {
                if operand2 == "0" {
                    println!("Error: Division by zero is not allowed.");
                } else {
                    println!(
                        "Division of {} / {} = {}",
                        operand1,
                        operand2,
                        div(operand1, operand2)
                    );
                }
            }
            _ => unreachable!(),
        }
    }
}

// Function to check if an operand is valid
fn is_valid_operand(operand: &str) -> bool {
    // Check if the operand is a positive integer
    operand.parse::<i32>().is_ok()
}

// Function to perform addition
fn add(operand1: &str, operand2: &str) -> i32 {
    operand1.parse::<i32>().unwrap() + operand2.parse::<i32>().unwrap()
}

// Function to perform subtraction
fn sub(operand1: &str, operand2: &str) -> i32 {
    operand1.parse::<i32>().unwrap() - operand2.parse::<i32>().unwrap()
}

// Function to perform multiplication
fn mul(operand1: &str, operand2: &str) -> i32 {
    operand1.parse::<i32>().unwrap() * operand2.parse::<i32>().unwrap()
}

// Function to perform division
fn div(operand1: &str, operand2: &str) -> i32 {
    operand1.parse::<i32>().unwrap() / operand2.parse::<i32>().unwrap()
}
