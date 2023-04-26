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

        let num_1 = match num_1.trim().parse::<i32>() {
    Ok(num) => num,
    Err(_) => {
        println!("Invalid input: {}", num_1);
        continue;
    }
};

let num_2 = match num_2.trim().parse::<i32>() {
    Ok(num) => num,
    Err(_) => {
        println!("Invalid input: {}", num_2);
        continue;
    }
};

match operand.trim() {
    "+" => println!("{} + {} = {}", num_1, num_2, num_1 + num_2),
    "-" => println!("{} - {} = {}", num_1, num_2, num_1 - num_2),
    "*" => println!("{} * {} = {}", num_1, num_2, num_1 * num_2),
    "/" => {
        if num_2 == 0 {
            println!("Error: Division by zero");
        } else {
            println!("{} / {} = {}", num_1, num_2, num_1 / num_2);
        }
    },
    _ => println!("Invalid operand"),
}


        println!("Result: {}", result);
    }
}
