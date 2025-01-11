use core::panic;
use std::{env::args, io::{self, Write}};



fn main() {
    let mut equation = String::new();
    for argument in args().skip(1) {
        equation = [equation, argument].join(" ");
    }

    if equation.is_empty() {
        math_loop()
    } else if let Some(value) = math(equation[1..].to_string()) {
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
    equation.push('_');


    let mut current_result = None;
    let chars: Vec<char> = equation.chars().collect();
    for char_num in 0..chars.len() {
        let char = chars[char_num];
        if ['+', '-', '*', '/', '%', '^'].iter().any(|&i| i == char) && chars[char_num + 1] == '_' {
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

        let input = input.trim();
        if let Some(value) = math(input.to_string()) {
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
        '^' => num1.powf(num2),
        _ => panic!("HOW??"),
    }
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn misc_tests() {
        assert_eq!(math("5".to_string()), Some(5.0));
        assert_eq!(math("2 + (3 * 4)".to_string()), Some(14.0));
        assert_eq!(math("".to_string()), None);
    }

    #[test]
    fn test_add() {
        assert_eq!(math("1 + 3".to_string()), Some(4.0));
        assert_eq!(math("2 + 7 + 6".to_string()), Some(15.0));
        assert_eq!(math("7 + -3".to_string()), Some(4.0));
        assert_eq!(math("-9 + -10".to_string()), Some(-19.0));
        assert_eq!(math("-9 + 10".to_string()), Some(1.0));
        assert_eq!(math("9.8 + 11.1".to_string()), Some(20.9));
    }

    #[test]
    fn test_subtract() {
        assert_eq!(math("4 - 2".to_string()), Some(2.0));
        assert_eq!(math("6 - 2 - 4".to_string()), Some(0.0));
        assert_eq!(math("7 - -3".to_string()), Some(10.0));
        assert_eq!(math("-4 - -5".to_string()), Some(1.0));
        assert_eq!(math("-2 - 3".to_string()), Some(-5.0));
        assert_eq!(math("8.3 - 2.2".to_string()), Some(8.3 - 2.2));
    }

    #[test]
    fn test_multiply() {
        assert_eq!(math("2 * 4".to_string()), Some(8.0));
        assert_eq!(math("8 * 2 * 3".to_string()), Some(48.0));
        assert_eq!(math("5 * -21".to_string()), Some(-105.0));
        assert_eq!(math("-2 * -3".to_string()), Some(6.0));
        assert_eq!(math("-3 * 4".to_string()), Some(-12.0));
        assert_eq!(math("4.7 * 4.3".to_string()), Some(20.21));
    }

    #[test]
    fn test_divide() {
        assert_eq!(math("5 / 2".to_string()), Some(2.5));
        assert_eq!(math("5 / 4 / 8".to_string()), Some(0.15625));
        assert_eq!(math("9 / -3".to_string()), Some(-3.0));
        assert_eq!(math("-5 / -8".to_string()), Some(0.625));
        assert_eq!(math("-4 / 10".to_string()), Some(-0.4));
        assert_eq!(math("43.1 / 7.9".to_string()), Some(43.1 / 7.9));
    }

    #[test]
    fn test_modulo() {
        assert_eq!(math("3 % 2".to_string()), Some(1.0));
        assert_eq!(math("6 % 8 % 9".to_string()), Some(6.0));
        assert_eq!(math("5 % -3".to_string()), Some(5.0 % -3.0));
        assert_eq!(math("-197 % -5".to_string()), Some(-2.0));
        assert_eq!(math("-9 % 4".to_string()), Some(-9.0 % 4.0));
        assert_eq!(math("547.43 % 4.2".to_string()), Some(547.43 % 4.2));
    }

    #[test]
    fn test_exponents() {
        assert_eq!(math("2 ^ 3".to_string()), Some(8.0));
        assert_eq!(math("3 ^ 2 ^ 2".to_string()), Some(81.0));
        assert_eq!(math("5 ^ -4".to_string()), Some(0.0016));
        assert_eq!(math("-4 ^ -3".to_string()), Some(-0.015625));
        assert_eq!(math("-8 ^ 3".to_string()), Some(-512.0));
        assert_eq!(math("7.9 ^ 4.8".to_string()), Some(7.9_f64.powf(4.8)));
    }
}
