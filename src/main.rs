use core::panic;
use std::{env::args, io::{self, Write}};



fn main() {
    let mut equation = String::new();
    for argument in args().skip(1) {
        equation = [equation, argument].join(" ");
    }

    if equation.is_empty() {
        math_loop()
    } else if let Some(value) = math(equation[1..].to_string() + "_") {
        println!("{:?}", value);
    }
}



fn math(base_equation: String) -> Option<f64> {
    let mut equation = String::new();
    let mut parenthises: u8 = 0;
    for char in base_equation.chars() {
        if char == ' ' && parenthises == 0 {
            equation.push('_');
        } else {
            equation.push(char);
            if char == '(' {
                parenthises += 1;
            } else if char == ')' {
                parenthises -= 1;
            }
        }
    }


    let mut current_result = None;
    let mut parenthises: u8 = 0;

    let chars: Vec<char> = equation.chars().collect();
    for char_num in 0..chars.len() {
        let char = chars[char_num];
        if char == '(' {
            parenthises += 1;
        } else if char == ')' {
            parenthises -= 1;
        } else if ['+', '-', '*', '/', '%', '^'].iter().any(|&i| i == char) && parenthises == 0 {
            let num1;
            let num2;
            if let Some(value) = current_result {
                num1 = value;
            } else if let Some(value) = get_value(equation[0..char_num - 1].to_string()) {
                num1 = value;
            } else {
                return None;
            }
            if let Some(value) = get_value(equation[char_num + 2..].split('_').next().expect("no second number found").to_string()) {
                num2 = value;
            } else {
                return None;
            }
            current_result = Some(do_operation(num1, num2, char))
        }
    }

    if let Some(result) = current_result {
        Some(result)
    } else {
        get_value(equation[..equation.len() - 1].to_string())
    }
}



fn math_loop() {
    loop {
        print!("--> ");
        io::stdout().flush().expect("Failed to flush stdout");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        // Remove the trailing newline character
        let input = input.trim();
        if let Some(value) = math(input.to_string() + "_") {
            println!("{:?}", value);
        }
    }
}



fn get_value(string: String) -> Option<f64> {
    if string.starts_with('(') {
        math(string[1..string.len() - 1].to_string() + "_")
    } else if let Ok(float) = string.parse::<f64>() {
        Some(float)
    } else {
        println!("unknown: {string:?}");
        None
    }
}



fn do_operation(num1: f64, num2: f64, operation: char) -> f64 {
    match operation {
        '+' => num1 + num2,
        '-' => num1 - num2,
        '*' => num1 * num2,
        '/' => num1 / num2,
        '%' => num1 % num2,
        '^' => {
            let mut result = num1;
            for _ in 1..num2 as usize {
                result *= num1;
            }
            result
        },
        _ => panic!("HOW??"),
    }
}
