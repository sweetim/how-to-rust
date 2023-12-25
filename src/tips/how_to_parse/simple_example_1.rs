use nom::{bytes::complete::tag, combinator::map, sequence::tuple, IResult};

use super::common::parse_field;

#[derive(Debug, PartialEq)]
pub struct VirtualMemory {
    pub total: f32,
    pub free: f32,
    pub used: f32,
    pub available: f32,
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

pub fn parse_virtual_memory_using_parser_combinator_nom(
    input: &str,
) -> IResult<&str, VirtualMemory> {
    map(
        tuple((
            tag("MiB Swap:"),
            parse_field("total"),
            parse_field("free"),
            parse_field("used."),
            parse_field("avail Mem"),
        )),
        |virtual_memory| VirtualMemory {
            total: virtual_memory.1,
            free: virtual_memory.2,
            used: virtual_memory.3,
            available: virtual_memory.4,
        },
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_can_parse_virtual_memory_simple() {
        let input = "MiB Swap:   3048.0 total,   2048.0 free,      0.0 used.   3392.8 avail Mem";

        let actual_delimiter = parse_virtual_memory_using_delimiter(input);
        let actual_regex = parse_virtual_memory_simple_using_regex(input);
        let actual_parser_combinator_nom = parse_virtual_memory_using_parser_combinator_nom(input)
            .unwrap()
            .1;

        let expected = VirtualMemory {
            total: 3048.0,
            free: 2048.0,
            used: 0.0,
            available: 3392.8,
        };

        assert_eq!(actual_delimiter, expected);
        assert_eq!(actual_regex, expected);
        assert_eq!(actual_parser_combinator_nom, expected);
    }
}
