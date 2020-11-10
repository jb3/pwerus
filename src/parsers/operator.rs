#[derive(Debug, PartialEq)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl Operator {
    pub fn new(input: &str) -> (&str, Self) {
        let op_char = &input[0..1];
        let op = match op_char {
            "+" => Operator::Add,
            "-" => Operator::Subtract,
            "*" => Operator::Multiply,
            "/" => Operator::Divide,
            _ => unimplemented!("Operator not implemented"),
        };

        (&input[1..], op)
    }
}

#[cfg(test)]
mod operator_tests {
    use super::*;

    #[test]
    fn can_parse_add() {
        assert_eq!(Operator::new("+"), ("", Operator::Add));
    }

    #[test]
    fn can_parse_subtract() {
        assert_eq!(Operator::new("-"), ("", Operator::Subtract));
    }

    #[test]
    fn can_parse_multiply() {
        assert_eq!(Operator::new("*"), ("", Operator::Multiply));
    }

    #[test]
    fn can_parse_divide() {
        assert_eq!(Operator::new("/"), ("", Operator::Divide));
    }

    #[test]
    #[should_panic(expected = "Operator not implemented")]
    fn handles_invalid_operator() {
        Operator::new("not an operator");
    }
}
