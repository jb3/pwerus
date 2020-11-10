mod parsers;

pub fn do_thing() -> parsers::expression::Expression {
    let (_, expr) = parsers::expression::Expression::new("10+200");

    expr
}
