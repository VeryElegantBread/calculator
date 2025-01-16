use std::collections::HashMap;

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
