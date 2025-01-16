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
        if ['+', '-', '*', '/', '%', '^'].iter().any(|&i| i == char) && chars[char_num + 1] == '_' {
            let num1;
            let num2;
            if let Some(value) = current_result {
                num1 = value;
            } else if let Some(value) = get_value(equation[0..char_num - 1].to_string(), variables) {
                num1 = value;
            } else {
                return None;
            }
            if let Some(value) = get_value(equation[char_num + 2..].split('_').next().expect("no second number found").to_string(), variables) {
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
                variables.insert(variable_name.to_string(), equation);
                println!("{} = {:?}", variable_name, value);
            }
        } else if let Some(value) = math(input.to_string(), &variables) {
            println!("{:?}", value);
        }
    }
}



fn get_value(string: String, variables: &HashMap<String, String>) -> Option<f64> {
    if string.starts_with('(') {
        math(string[1..string.len() - 1].to_string() + "_", variables)
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
    equation.push('_');
    equation
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn misc_tests() {
        assert_eq!(math("5".to_string(), &HashMap::new()), Some(5.0));
        assert_eq!(math("2 + (3 * 4)".to_string(), &HashMap::new()), Some(14.0));
        assert_eq!(math("".to_string(), &HashMap::new()), None);
    }

    #[test]
    fn test_add() {
        assert_eq!(math("1 + 3".to_string(), &HashMap::new()), Some(4.0));
        assert_eq!(math("2 + 7 + 6".to_string(), &HashMap::new()), Some(15.0));
        assert_eq!(math("7 + -3".to_string(), &HashMap::new()), Some(4.0));
        assert_eq!(math("-9 + -10".to_string(), &HashMap::new()), Some(-19.0));
        assert_eq!(math("-9 + 10".to_string(), &HashMap::new()), Some(1.0));
        assert_eq!(math("9.8 + 11.1".to_string(), &HashMap::new()), Some(20.9));
    }

    #[test]
    fn test_subtract() {
        assert_eq!(math("4 - 2".to_string(), &HashMap::new()), Some(2.0));
        assert_eq!(math("6 - 2 - 4".to_string(), &HashMap::new()), Some(0.0));
        assert_eq!(math("7 - -3".to_string(), &HashMap::new()), Some(10.0));
        assert_eq!(math("-4 - -5".to_string(), &HashMap::new()), Some(1.0));
        assert_eq!(math("-2 - 3".to_string(), &HashMap::new()), Some(-5.0));
        assert_eq!(math("8.3 - 2.2".to_string(), &HashMap::new()), Some(8.3 - 2.2));
    }

    #[test]
    fn test_multiply() {
        assert_eq!(math("2 * 4".to_string(), &HashMap::new()), Some(8.0));
        assert_eq!(math("8 * 2 * 3".to_string(), &HashMap::new()), Some(48.0));
        assert_eq!(math("5 * -21".to_string(), &HashMap::new()), Some(-105.0));
        assert_eq!(math("-2 * -3".to_string(), &HashMap::new()), Some(6.0));
        assert_eq!(math("-3 * 4".to_string(), &HashMap::new()), Some(-12.0));
        assert_eq!(math("4.7 * 4.3".to_string(), &HashMap::new()), Some(20.21));
    }

    #[test]
    fn test_divide() {
        assert_eq!(math("5 / 2".to_string(), &HashMap::new()), Some(2.5));
        assert_eq!(math("5 / 4 / 8".to_string(), &HashMap::new()), Some(0.15625));
        assert_eq!(math("9 / -3".to_string(), &HashMap::new()), Some(-3.0));
        assert_eq!(math("-5 / -8".to_string(), &HashMap::new()), Some(0.625));
        assert_eq!(math("-4 / 10".to_string(), &HashMap::new()), Some(-0.4));
        assert_eq!(math("43.1 / 7.9".to_string(), &HashMap::new()), Some(43.1 / 7.9));
    }

    #[test]
    fn test_modulo() {
        assert_eq!(math("3 % 2".to_string(), &HashMap::new()), Some(1.0));
        assert_eq!(math("6 % 8 % 9".to_string(), &HashMap::new()), Some(6.0));
        assert_eq!(math("5 % -3".to_string(), &HashMap::new()), Some(5.0 % -3.0));
        assert_eq!(math("-197 % -5".to_string(), &HashMap::new()), Some(-2.0));
        assert_eq!(math("-9 % 4".to_string(), &HashMap::new()), Some(-9.0 % 4.0));
        assert_eq!(math("547.43 % 4.2".to_string(), &HashMap::new()), Some(547.43 % 4.2));
    }

    #[test]
    fn test_exponents() {
        assert_eq!(math("2 ^ 3".to_string(), &HashMap::new()), Some(8.0));
        assert_eq!(math("3 ^ 2 ^ 2".to_string(), &HashMap::new()), Some(81.0));
        assert_eq!(math("5 ^ -4".to_string(), &HashMap::new()), Some(0.0016));
        assert_eq!(math("-4 ^ -3".to_string(), &HashMap::new()), Some(-0.015625));
        assert_eq!(math("-8 ^ 3".to_string(), &HashMap::new()), Some(-512.0));
        assert_eq!(math("7.9 ^ 4.8".to_string(), &HashMap::new()), Some(7.9_f64.powf(4.8)));
    }
}
