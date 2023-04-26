use std::io;

fn main() {
    loop {
        println!("Enter an expression (e.g. 2 + 3): ");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        let parts: Vec<&str> = input.trim().split(' ').collect();

        if parts.len() != 3 {
            println!("Invalid expression");
            continue;
        }

        let left = match parts[0].parse::<f64>() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid expression");
                continue;
            }
        };

        let right = match parts[2].parse::<f64>() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid expression");
                continue;
            }
        };

        let result = match parts[1] {
            "+" => left + right,
            "-" => left - right,
            "*" => left * right,
            "/" => left / right,
            _ => {
                println!("Invalid operator");
                continue;
            }
        };

        println!("Result: {}", result);
    }
}
