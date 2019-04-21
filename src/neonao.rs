#![allow(dead_code)]
#![allow(unused_imports)]
use combine::parser::byte::digit;
use combine::parser::char::{char, spaces, string, Str};
use combine::parser::choice::or;
use combine::parser::repeat::take_until;
use combine::stream::easy;
use combine::stream::state::State;
use combine::{
    any, attempt, between, choice, look_ahead, many, many1, not_followed_by, satisfy, sep_by,
    ParseError, Parser, Stream,
};

fn bold_text<I>() -> impl Parser<Input = &'static str, Output = String> {
    let char_in_bold = not_followed_by(string("**")).with(any());
    string("**").with(many(char_in_bold)).skip(string("**"))
}

#[test]
fn parser_test() {
    let (result, rest) = bold_text::<&str>().parse("**hello * world**").unwrap();
    assert_eq!(result, "hello * world");
    assert_eq!(rest, "");
}
