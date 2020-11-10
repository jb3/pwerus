fn parse_integer_component(input: &str) -> (&str, &str) {
    let last_index = input
        .char_indices()
        .find_map(|(index, char)| {
            if char.is_ascii_digit() {
                None
            } else {
                Some(index)
            }
        })
        .unwrap_or_else(|| input.len());

    let integer = &input[..last_index];
    let rest = &input[last_index..];
    (rest, integer)
}

#[derive(Debug, PartialEq)]
pub struct Integer(pub i32);

impl Integer {
    pub fn new(input: &str) -> (&str, Self) {
        let (rest, integer) = parse_integer_component(input);
        (rest, Self(integer.parse().unwrap()))
    }
}

#[cfg(test)]
mod integer_tests {
    use super::*;

    #[test]
    fn can_parse_basic_integer() {
        assert_eq!(Integer::new("42"), ("", Integer(42)));
    }

    #[test]
    fn can_parse_integer_component() {
        assert_eq!(parse_integer_component("10hello"), ("hello", "10"));
    }
}
