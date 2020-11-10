use super::{integer::Integer, operator::Operator};

#[derive(Debug, PartialEq)]
pub struct Expression {
    pub lhs: Integer,
    pub rhs: Integer,
    pub op: Operator,
}

impl Expression {
    pub fn new(input: &str) -> (&str, Self) {
        let (rest, lhs) = Integer::new(input);
        let (rest, op) = Operator::new(rest);
        let (rest, rhs) = Integer::new(rest);

        (rest, Self { lhs, rhs, op })
    }
}

#[cfg(test)]
mod expression_tests {
    use super::*;

    #[test]
    fn can_parse_basic_expression() {
        let (_, op) = Operator::new("+");
        let (_, a) = Integer::new("1");
        let (_, b) = Integer::new("2");

        assert_eq!(
            Expression::new("1+2"),
            ("", Expression { lhs: a, rhs: b, op })
        );
    }

    #[test]
    fn can_parse_multidigit_expression() {
        let (_, op) = Operator::new("-");
        let (_, a) = Integer::new("100");
        let (_, b) = Integer::new("20");

        assert_eq!(
            Expression::new("100-20"),
            ("", Expression { lhs: a, rhs: b, op })
        );
    }
}
