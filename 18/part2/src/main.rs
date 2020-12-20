use nom::{
    branch::alt,
    character::complete::{char, digit1, multispace0, one_of},
    combinator::map_res,
    multi::fold_many0,
    sequence::{delimited, tuple},
    IResult,
};
use std::fs::read_to_string;

fn parse_number(input: &str) -> IResult<&str, u64> {
    map_res(digit1, str::parse)(input)
}

fn parse_operand(input: &str) -> IResult<&str, u64> {
    alt((
        parse_number,
        delimited(char('('), parse_expression, char(')')),
    ))(input)
}

fn parse_operation(input: &str) -> IResult<&str, (char, u64)> {
    tuple((
        delimited(multispace0, one_of("+*"), multispace0),
        parse_operand,
    ))(input)
}

fn parse_expression(input: &str) -> IResult<&str, u64> {
    let (input, a) = parse_operand(input)?;
    let (input, products) =
        fold_many0(parse_operation, vec![a], |mut products, (op, b)| match op {
            '+' => {
                let a = products.pop().unwrap();
                products.push(a + b);
                products
            }
            '*' => {
                products.push(b);
                products
            }
            _ => unreachable!(),
        })(input)?;
    Ok((input, products.into_iter().product()))
}

fn main() {
    let expressions_sum: u64 = read_to_string("../input")
        .expect("Failed to open input file")
        .lines()
        .map(|expression| {
            parse_expression(expression)
                .expect("Failed to parse expression")
                .1
        })
        .sum();

    println!(
        "The sum of all the expression results is {}!",
        expressions_sum
    );
}
