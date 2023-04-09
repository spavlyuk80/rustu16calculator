#[cfg(test)]
mod tests {
    use crate::calculator::Calculator;

    #[test]
    fn test_addition() {
        let calc = Calculator::new("2.5", "3.8", "+").unwrap();
        assert_eq!(calc.get_result().unwrap(), 6.3);
    }

    #[test]
    fn test_subtraction() {
        let calc = Calculator::new("10", "2.5", "-").unwrap();
        assert_eq!(calc.get_result().unwrap(), 7.5);
    }

    #[test]
    fn test_multiplication() {
        let calc = Calculator::new("4", "5", "*").unwrap();
        assert_eq!(calc.get_result().unwrap(), 20.0);
    }

    #[test]
    fn test_division() {
        let calc = Calculator::new("6", "2", "/").unwrap();
        assert_eq!(calc.get_result().unwrap(), 3.0);
    }

    #[test]
    fn test_division_by_zero() {
        let calc = Calculator::new("6", "0", "/").unwrap();
        assert!(calc.get_result().is_err());
    }

    #[test]
    fn test_invalid_sign() {
        let calc = Calculator::new("6", "2", "invalid").unwrap_err();
        assert_eq!(calc.to_string(), "None of the given signs matched");
    }

    #[test]
    fn test_invalid_input_a() {
        let calc = Calculator::new("invalid", "2", "+").unwrap_err();
        assert_eq!(calc.to_string(), "Could not parse value a into float32");
    }

    #[test]
    fn test_invalid_input_b() {
        let calc = Calculator::new("6", "invalid", "+").unwrap_err();
        assert_eq!(calc.to_string(), "Could not parse value a into float32");
    }
}