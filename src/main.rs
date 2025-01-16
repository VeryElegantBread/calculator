use core::panic;
use std::{collections::HashMap, env::args, io::{self, Write}};



fn main() {
    let mut equation = String::new();
    for argument in args().skip(1) {
        equation = [equation, argument].join(" ");
    }

    if equation.is_empty() {
        math_loop()
    } else if let Some(value) = math(equation[1..].to_string(), &HashMap::new()) {
        println!("{:?}", value);
    }
}



fn math(base_equation: String, variables: &HashMap<String, String>) -> Option<f64> {
    let equation = format_equation(base_equation);

    let mut current_result = None;
    let chars: Vec<char> = equation.chars().collect();
    for char_num in 0..chars.len() {
        let char = chars[char_num];
        if ['+', '-', '*', '/', '%', '^'].iter().any(|&i| i == char) && chars[char_num + 1] == '\\' {
            let num1;
            let num2;
            if let Some(value) = current_result {
                num1 = value;
            } else if let Some(value) = get_value(equation[0..char_num - 1].to_string(), variables) {
                num1 = value;
            } else {
                return None;
            }
            if let Some(value) = get_value(equation[char_num + 2..].split('\\').next().expect("no second number found").to_string(), variables) {
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
        get_value(equation[..equation.len() - 1].to_string(), variables)
    }
}



fn math_loop() {
    let mut variables: HashMap<String, String> = HashMap::new();
    loop {
        print!("--> ");
        io::stdout().flush().expect("Failed to flush stdout");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input = input.trim();

        let split_input: Vec<_> = input.split(' ').collect();
        if split_input.len() > 2 && split_input[1] == "=" {
            let equation = input[split_input[0].len() + 3..].to_string();
            if let Some(value) = math(equation.to_string(), &variables) {
                let variable_name = split_input[0];
                if variable_name.chars().any(|i| i == '\\') {
                    println!("No backslashes in variable names.");
                } else {
                    variables.insert(variable_name.to_string(), equation);
                    println!("{} = {:?}", variable_name, value);
                }
            }
        } else if let Some(value) = math(input.to_string(), &variables) {
            println!("{:?}", value);
        }
    }
}



fn get_value(string: String, variables: &HashMap<String, String>) -> Option<f64> {
    if string.starts_with('(') {
        math(string[1..string.len() - 1].to_string(), variables)
    } else if let Ok(float) = string.parse::<f64>() {
        Some(float)
    } else if let Some(value) = variables.get(&string) {
        math(value.to_string(), variables)
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
        '^' => num1.powf(num2),
        _ => panic!("HOW??"),
    }
}




fn format_equation(base_equation: String) -> String {
    let mut equation = String::new();
    let mut parenthises: u8 = 0;
    for char in base_equation.chars() {
        if char == ' ' && parenthises == 0 {
            equation.push('\\');
        } else {
            equation.push(char);
            if char == '(' {
                parenthises += 1;
            } else if char == ')' {
                parenthises -= 1;
            }
        }
    }
    equation.push('\\');
    equation
}




#[cfg(test)]
mod tests;
