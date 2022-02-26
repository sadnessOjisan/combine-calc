use combine::{
    chainl1, choice,
    error::ParseError,
    parser::{
        char::{char, digit, letter, spaces},
    },
    Parser, Stream, many1, between,
};

#[derive(Debug)]
enum Expr {
    Scalar(f64),
    Var(char),
    Prod(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Sum(Box<Expr>, Box<Expr>),
    Diff(Box<Expr>, Box<Expr>),
}

fn parse_expr<Input>() -> impl Parser<Input, Output = Expr>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    let tok = choice([char('-'), char('+')]).map(|op| {
        move |a, b| {
            if op == '+' {
                Expr::Sum(Box::new(a), Box::new(b))
            } else if op == '-' {
                Expr::Diff(Box::new(a), Box::new(b))
            } else {
                unimplemented!()
            }
        }
    });
    let mut sum = chainl1(parse_term(), tok);
    sum
}

fn parse_term<Input>() -> impl Parser<Input, Output = Expr>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    let tok = choice([char('*'), char('/')]).map(|op| {
        move |a, b| {
            if op == '*' {
                Expr::Prod(Box::new(a), Box::new(b))
            } else if op == '/' {
                Expr::Div(Box::new(a), Box::new(b))
            } else {
                unimplemented!()
            }
        }
    });
    let mut prod = chainl1(parse_factor(), tok);
    prod
}

fn parse_factor<Input>() -> impl Parser<Input, Output = Expr>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    let scalar = many1(digit()).map(|t: String| Expr::Scalar(t.parse().unwrap()));
    scalar
}

fn main() {
    let res = parse_expr().parse("1+2*3");
    println!("{:?}", res);
}