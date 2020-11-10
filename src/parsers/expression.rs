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
        assert_eq!(
            Expression::new("1+2"),
            ("", Expression {
                lhs: Integer(1),
                rhs: Integer(2),
                op: Operator::Add
            })
        );
    }

    #[test]
    fn can_parse_multidigit_expression() {
        assert_eq!(
            Expression::new("100-20"),
            ("", Expression {
                lhs: Integer(100),
                rhs: Integer(20),
                op: Operator::Subtract
            })
        );
    }
}
