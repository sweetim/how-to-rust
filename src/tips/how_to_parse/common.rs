use nom::{
    bytes::complete::tag,
    character::complete::space0,
    combinator::opt,
    number::complete::float,
    sequence::{delimited, terminated},
    IResult,
};

pub fn parse_field<'a>(key: &'static str) -> impl FnMut(&'a str) -> IResult<&'a str, f32> {
    terminated(
        delimited(space0, float, space0),
        terminated(tag(key), opt(tag(","))),
    )
}
