use std::io;
use std::fs::OpenOptions;
use std::io::Write;

fn main() {
    loop {
        println!("Enter first number:");
        let num1 = read_number();
        
        println!("Enter operation (+, -, *, /, ^):");
        let op = read_operator();
        
        println!("Enter second number:");
        let num2 = read_number();
        
        let result = calculate(num1, num2, op);
        
        match result {
            Ok(res) => {
                println!("Result: {}", res);
                log_history(num1, op, num2, res);
            }
            Err(e) => println!("Error: {}", e),
        }

        println!("Do you want to perform another calculation? (yes/no)");
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read input");
        if choice.trim().to_lowercase() != "yes" {
            break;
        }
    }
}

// Function to read a number from the user
fn read_number() -> f64 {
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        match input.trim().parse::<f64>() {
            Ok(num) => return num,
            Err(_) => println!("Invalid number, please enter again:"),
        }
    }
}

// Function to read a valid operator
fn read_operator() -> char {
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let op = input.trim();
        if ["+", "-", "*", "/, ^"].contains(&op) {
            return op.chars().next().unwrap();
        } else {
            println!("Invalid operator. Please enter +, -, *, or /:");
        }
    }
}

// Function to perform calculation
fn calculate(num1: f64, num2: f64, op: char) -> Result<f64, &'static str> {
    match op {
        '+' => Ok(num1 + num2),
        '-' => Ok(num1 - num2),
        '*' => Ok(num1 * num2),
        '^' => Ok(num1.powf(num2)),
        '/' => {
            if num2 == 0.0 {
                Err("Cannot divide by zero")
            } else {
                Ok(num1 / num2)
            }
        }
        _ => Err("Invalid operator"),
    }
}

// Function to log history
fn log_history(num1: f64, op: char, num2: f64, result: f64) {
    let history = format!("{} {} {} = {}\n", num1, op, num2, result);
    let mut file = OpenOptions::new().append(true).create(true).open("history.txt").expect("Cannot open file");
    file.write_all(history.as_bytes()).expect("Failed to write to file");
}