use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit0, space0},
    combinator::{map, map_res},
    sequence::{preceded, terminated, tuple},
    IResult,
};

use super::common::parse_field;

#[derive(Debug, PartialEq)]
pub struct CpuStates {
    pub id: i32,
    pub user: f32,
    pub system: f32,
    pub nice: f32,
    pub idle: f32,
    pub io_wait: f32,
    pub hw_irq: f32,
    pub soft_irq: f32,
    pub steal: f32,
}

pub fn parse_cpu_states_using_delimiter(input: &str) -> CpuStates {
    let mut tokens = input.split_terminator(&[':', ',', ' ']);

    let cpu_id = tokens
        .next()
        .unwrap()
        .split("%Cpu")
        .filter_map(|text| text.parse::<i32>().ok())
        .next()
        .unwrap_or(-1);

    let mut tokens = tokens.filter_map(|text| text.parse::<f32>().ok());

    CpuStates {
        id: cpu_id,
        user: tokens.next().unwrap(),
        system: tokens.next().unwrap(),
        nice: tokens.next().unwrap(),
        idle: tokens.next().unwrap(),
        io_wait: tokens.next().unwrap(),
        hw_irq: tokens.next().unwrap(),
        soft_irq: tokens.next().unwrap(),
        steal: tokens.next().unwrap(),
    }
}

pub fn parse_cpu_states_using_regex(input: &str) -> Vec<CpuStates> {
    let re = regex::RegexBuilder::new(r"^%Cpu([\d]+|\b\(s\))\s*:\s*(\d+.\d+)\sus,\s*(\d+.\d+)\ssy,\s*(\d+.\d+)\sni,\s*(\d+.\d+)\sid,\s*(\d+.\d+)\swa,\s*(\d+.\d+)\shi,\s*(\d+.\d+)\ssi,\s*(\d+.\d+)\sst")
        .multi_line(true)
        .build()
        .unwrap();

    let tokens = re
        .captures_iter(input)
        .map(|captures| captures.extract::<9>().1)
        .flatten()
        .map(|text| text.parse::<f32>().unwrap_or(-1.0))
        .collect::<Vec<_>>();

    tokens
        .chunks(9)
        .map(|chunk| CpuStates {
            id: chunk[0] as i32,
            user: chunk[1],
            system: chunk[2],
            nice: chunk[3],
            idle: chunk[4],
            io_wait: chunk[5],
            hw_irq: chunk[6],
            soft_irq: chunk[7],
            steal: chunk[8],
        })
        .collect::<Vec<_>>()
}

pub fn parse_cpu_states_using_parser_combinator_nom(input: &str) -> IResult<&str, CpuStates> {
    map(
        tuple((
            parse_cpu_id,
            parse_field("us"),
            parse_field("sy"),
            parse_field("ni"),
            parse_field("id"),
            parse_field("wa"),
            parse_field("hi"),
            parse_field("si"),
            parse_field("st"),
        )),
        |cpus| CpuStates {
            id: cpus.0,
            user: cpus.1,
            system: cpus.2,
            nice: cpus.3,
            idle: cpus.4,
            io_wait: cpus.5,
            hw_irq: cpus.6,
            soft_irq: cpus.7,
            steal: cpus.8,
        },
    )(input)
}

fn parse_cpu_id(input: &str) -> IResult<&str, i32> {
    alt((
        map(tag("%Cpu(s):"), |_| -1),
        map_res(
            preceded(tag("%Cpu"), terminated(digit0, tuple((space0, tag(":"))))),
            str::parse::<i32>,
        ),
    ))(input)
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(
        "%Cpu0  :  0.0 us, 22.7 sy,  0.0 ni, 77.3 id,  0.0 wa,  0.0 hi,  0.0 si,  0.0 st",
        (0, 0.0, 22.7, 0.0, 77.3, 0.0, 0.0, 0.0, 0.0)
    )]
    #[case(
        "%Cpu2  :  0.0 us,  0.0 sy,  0.0 ni,100.0 id,  0.0 wa,  0.0 hi,  0.0 si,  0.0 st",
        (2, 0.0, 0.0, 0.0, 100.0, 0.0, 0.0, 0.0, 0.0)
    )]
    #[case(
        "%Cpu(s):  0.4 us,  0.8 sy,  0.0 ni, 98.4 id,  0.0 wa,  0.0 hi,  0.4 si,  0.0 st",
        (-1, 0.4, 0.8, 0.0, 98.4, 0.0, 0.0, 0.4, 0.0)
    )]
    fn it_can_parse_cpu_status(
        #[case] input: &str,
        #[case] expected: (i32, f32, f32, f32, f32, f32, f32, f32, f32),
    ) {
        let expected = CpuStates {
            id: expected.0,
            user: expected.1,
            system: expected.2,
            nice: expected.3,
            idle: expected.4,
            io_wait: expected.5,
            hw_irq: expected.6,
            soft_irq: expected.7,
            steal: expected.8,
        };

        let actual_delimiter = parse_cpu_states_using_delimiter(input);
        let actual_regex = parse_cpu_states_using_regex(input)
            .into_iter()
            .nth(0)
            .unwrap();
        let actual_parser_combinator_nom = parse_cpu_states_using_parser_combinator_nom(input)
            .unwrap()
            .1;

        assert_eq!(actual_delimiter, expected);
        assert_eq!(actual_regex, expected);
        assert_eq!(actual_parser_combinator_nom, expected);
    }

    #[rstest]
    #[case::without_space("%Cpu0:", 0)]
    #[case::with_space("%Cpu12  :", 12)]
    #[case::all_cpus("%Cpu(s):", -1)]
    fn it_parse_cpu_count(#[case] input: &str, #[case] expected: i32) {
        let actual = parse_cpu_id(input).unwrap().1;

        assert_eq!(actual, expected);
    }
}
