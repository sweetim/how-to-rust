use nom::{
    bytes::complete::tag,
    character::complete::space0,
    combinator::opt,
    number::complete::float,
    sequence::{delimited, terminated, tuple},
    IResult,
};

#[derive(Debug, PartialEq)]
pub struct VirtualMemory {
    total: f32,
    free: f32,
    used: f32,
    available: f32,
}

pub fn parse_virtual_memory_using_delimiter(input: &str) -> VirtualMemory {
    let mut output = input
        .split_whitespace()
        .filter_map(|text| text.parse::<f32>().ok());

    VirtualMemory {
        total: output.next().unwrap(),
        free: output.next().unwrap(),
        used: output.next().unwrap(),
        available: output.next().unwrap(),
    }
}

pub fn parse_virtual_memory_simple_using_regex(input: &str) -> VirtualMemory {
    let re = regex::Regex::new(r"MiB Swap:\s+(\d+\.\d+)\s+total,\s+(\d+\.\d+)\s+free,\s+(\d+\.\d+)\s+used\.\s+(\d+\.\d+)\s+avail Mem")
        .unwrap();

    let mut values = re
        .captures_iter(input)
        .map(|captures| captures.extract::<4>().1)
        .flat_map(|text| text.into_iter().map(str::parse::<f32>).map(Result::unwrap));

    VirtualMemory {
        total: values.next().unwrap(),
        free: values.next().unwrap(),
        used: values.next().unwrap(),
        available: values.next().unwrap(),
    }
}

fn parse_field<'a>(key: &'static str) -> impl FnMut(&'a str) -> IResult<&'a str, f32> {
    terminated(
        delimited(space0, float, space0),
        terminated(tag(key), opt(tag(","))),
    )
}

pub fn parse_virtual_memory_using_parser_combinator_nom(input: &str) -> VirtualMemory {
    let (_, virtual_memory) = tuple((
        tag("MiB Swap:"),
        parse_field("total"),
        parse_field("free"),
        parse_field("used."),
        parse_field("avail Mem"),
    ))(input)
    .unwrap();

    VirtualMemory {
        total: virtual_memory.1,
        free: virtual_memory.2,
        used: virtual_memory.3,
        available: virtual_memory.4,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_can_parse_virtual_memory_simple() {
        let input = "MiB Swap:   3048.0 total,   2048.0 free,      0.0 used.   3392.8 avail Mem";

        let actual_1 = parse_virtual_memory_using_delimiter(input);
        let actual_2 = parse_virtual_memory_simple_using_regex(input);
        let actual_3 = parse_virtual_memory_using_parser_combinator_nom(input);

        let expected = VirtualMemory {
            total: 3048.0,
            free: 2048.0,
            used: 0.0,
            available: 3392.8,
        };

        assert_eq!(actual_1, expected);
        assert_eq!(actual_2, expected);
        assert_eq!(actual_3, expected);
    }
}
