use nom::{
    bytes::complete::tag,
    character::complete::space0,
    combinator::opt,
    error::Error,
    number::complete::float,
    sequence::{delimited, terminated},
    Parser,
};

pub fn parse_field<'a>(
    key: &'static str,
) -> impl Parser<&'a str, Output = f32, Error = Error<&'a str>> {
    terminated(
        delimited(space0, float, space0),
        terminated(tag(key), opt(tag(","))),
    )
}
