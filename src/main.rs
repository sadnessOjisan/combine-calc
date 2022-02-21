extern crate combine;
use combine::parser::char::{spaces, digit, char};
use combine::{many1, sep_by, Parser, EasyParser};
use combine::stream::easy;

fn main() {
    //Parse spaces first and use the with method to only keep the result of the next parser
    let integer = spaces()
        //parse a string of digits into an i32
        .with(many1(digit()).map(|string: String| string.parse::<i32>().unwrap()));

    //Parse integers separated by commas, skipping whitespace
    let mut integer_list = sep_by(integer, spaces().skip(char(',')));

    //Call parse with the input to execute the parser
    let input = "1234, 45,78";
    let result: Result<(Vec<i32>, &str), easy::ParseError<&str>> =
        integer_list.easy_parse(input);
    match result {
        Ok((value, _remaining_input)) => println!("{:?}", value),
        Err(err) => println!("{}", err)
    }
}