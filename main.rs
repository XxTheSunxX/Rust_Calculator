// Simple CLI Calculator Written in Rust 9/27/2025
use std::io;
use std::process;

fn main() {
    println!("RUST CLI CALCULATOR, Written by xXTheSunXx.");
    
    println!("Start calculator: (y)es or (n)o");

    loop{
        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read choice.");
        let choice = choice.trim();
        if choice == "y" {
            calc();

        } else if choice == "n" {
            println!("Exiting...");
            process::exit(0);

        } else {
            println!("Enter either 'y' or 'n' ");
            continue;
        }
    }
}

fn calc() {
    loop{
        println!("Enter first number: ");
        let mut number_1 = String::new(); // declare variable as set to new string
        io::stdin() // standard in
            .read_line(&mut number_1) //read line and set number 1 as the entered line
            .expect("Failed to read line"); // expect to fail to read line
        let number_1: f64 = match number_1.trim().parse() { // number 1 is set to float and match/trim/parse
            Ok(num) => num, // if the result is ok, then bind the value to num
            Err(_) => { // if not then error and display the println! message
                println!("Invalid Input, try again.");
                continue;
            }
        };

        println!("Enter second number: ");
        let mut number_2 = String::new();
        io::stdin()
            .read_line(&mut number_2)
            .expect("Failed to read line");
        let number_2: f64 = match number_2.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input, try again.");
                continue;
            }
        };

        println!("Enter operator: (+, -, *, /) or 'q' to quit: ");
        let mut op = String::new();
        io::stdin()
            .read_line(&mut op)
            .expect("Failed to read line");
        let op = op.trim();

        if op == "q" {
            println!("Exiting calculator now...");
            break;
        }

        //perform calculation
        let result = match op { // let the result variable equals chosen op, then which ever op it is, complete the calculation
            "+" => number_1 + number_2,
            "-" => number_1 - number_2,
            "*" => number_1 * number_2,
            "/" => {
                if number_2 == 0.0 { // if number_2 is equals to 0.0, throw an error
                    println!("Error: Division by zero.");
                    continue;
                }
                number_1 / number_2
            }

            _ => { // _ is a catch-all pattern, It matches anything that wasn't matched by the previous arms.
                println!("Invalid operator, use: +, -, *, / or 'q'.");
                continue;
            }
        
        };

        println!("Result: {}", result);
    }

}