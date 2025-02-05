use nom::{
    branch::alt,
    character::complete::line_ending,
    combinator::{eof, map},
    multi::many1,
    sequence::terminated,
    Parser,
};

use super::{
    simple_example_1::{
        parse_virtual_memory_simple_using_regex, parse_virtual_memory_using_delimiter,
        parse_virtual_memory_using_parser_combinator_nom, VirtualMemory,
    },
    simple_example_2::{
        parse_cpu_states_using_delimiter, parse_cpu_states_using_parser_combinator_nom,
        parse_cpu_states_using_regex, CpuStates,
    },
};

#[derive(Debug, PartialEq)]
pub struct SummaryDisplay {
    cpu_states: Vec<CpuStates>,
    virtual_memory: VirtualMemory,
}

pub fn parse_sumamry_display_using_delimiter(input: &str) -> SummaryDisplay {
    let lines = input.trim_end().split("\n").collect::<Vec<_>>();

    let virtual_memory_line_number: usize = match lines.last().unwrap().is_empty() {
        true => lines.len() - 2,
        false => lines.len() - 1,
    };

    let cpu_states = lines[0..virtual_memory_line_number]
        .iter()
        .copied()
        .map(parse_cpu_states_using_delimiter)
        .collect::<Vec<_>>();

    SummaryDisplay {
        cpu_states,
        virtual_memory: parse_virtual_memory_using_delimiter(lines[virtual_memory_line_number]),
    }
}

pub fn parse_summary_display_using_regex(input: &str) -> SummaryDisplay {
    SummaryDisplay {
        cpu_states: parse_cpu_states_using_regex(input),
        virtual_memory: parse_virtual_memory_simple_using_regex(input),
    }
}

pub fn parse_summary_display_using_parser_combinator_nom(input: &str) -> SummaryDisplay {
    map(
        (
            many1(terminated(
                parse_cpu_states_using_parser_combinator_nom,
                line_ending,
            )),
            terminated(
                parse_virtual_memory_using_parser_combinator_nom,
                alt((line_ending, eof)),
            ),
        ),
        |output| SummaryDisplay {
            cpu_states: output.0,
            virtual_memory: output.1,
        },
    )
    .parse(input)
    .unwrap()
    .1
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    use super::*;
    use crate::tips::how_to_parse::{simple_example_1::VirtualMemory, simple_example_2::CpuStates};

    #[test]
    fn it_can_parse_top_header_overall_cpus() {
        let input = r"%Cpu(s):  0.4 us,  0.8 sy,  0.0 ni, 98.4 id,  0.0 wa,  0.0 hi,  0.4 si,  0.0 st
MiB Swap:   3048.0 total,   2048.0 free,      0.0 used.   3392.8 avail Mem";

        let expected = SummaryDisplay {
            cpu_states: vec![CpuStates {
                id: -1,
                user: 0.4,
                system: 0.8,
                nice: 0.0,
                idle: 98.4,
                io_wait: 0.0,
                hw_irq: 0.0,
                soft_irq: 0.4,
                steal: 0.0,
            }],
            virtual_memory: VirtualMemory {
                total: 3048.0,
                free: 2048.0,
                used: 0.0,
                available: 3392.8,
            },
        };

        let actual_delimiter = parse_sumamry_display_using_delimiter(input);
        let actual_regex = parse_summary_display_using_regex(input);
        let actual_parser_combinator_nom = parse_summary_display_using_parser_combinator_nom(input);

        assert_eq!(actual_delimiter, expected);
        assert_eq!(actual_regex, expected);
        assert_eq!(actual_parser_combinator_nom, expected);
    }

    #[rstest]
    #[case(
        r"%Cpu0  :  0.0 us, 22.7 sy,  0.0 ni, 77.3 id,  0.0 wa,  0.0 hi,  0.0 si,  0.0 st
%Cpu1  :  4.8 us,  0.0 sy,  0.0 ni, 95.2 id,  0.0 wa,  0.0 hi,  0.0 si,  0.0 st
%Cpu2  :  0.0 us,  0.0 sy,  0.0 ni,100.0 id,  0.0 wa,  0.0 hi,  0.0 si,  0.0 st
%Cpu3  :  5.3 us, 10.5 sy,  0.0 ni, 84.2 id,  0.0 wa,  0.0 hi,  0.0 si,  0.0 st
MiB Swap:   3048.0 total,   2048.0 free,      0.0 used.   3392.8 avail Mem"
    )]
    #[case::with_new_line(
        r"%Cpu0  :  0.0 us, 22.7 sy,  0.0 ni, 77.3 id,  0.0 wa,  0.0 hi,  0.0 si,  0.0 st
%Cpu1  :  4.8 us,  0.0 sy,  0.0 ni, 95.2 id,  0.0 wa,  0.0 hi,  0.0 si,  0.0 st
%Cpu2  :  0.0 us,  0.0 sy,  0.0 ni,100.0 id,  0.0 wa,  0.0 hi,  0.0 si,  0.0 st
%Cpu3  :  5.3 us, 10.5 sy,  0.0 ni, 84.2 id,  0.0 wa,  0.0 hi,  0.0 si,  0.0 st
MiB Swap:   3048.0 total,   2048.0 free,      0.0 used.   3392.8 avail Mem
"
    )]
    fn it_can_parse_top_header_multi_cpus(#[case] input: &str) {
        let expected = SummaryDisplay {
            cpu_states: vec![
                CpuStates {
                    id: 0,
                    user: 0.0,
                    system: 22.7,
                    nice: 0.0,
                    idle: 77.3,
                    io_wait: 0.0,
                    hw_irq: 0.0,
                    soft_irq: 0.0,
                    steal: 0.0,
                },
                CpuStates {
                    id: 1,
                    user: 4.8,
                    system: 0.0,
                    nice: 0.0,
                    idle: 95.2,
                    io_wait: 0.0,
                    hw_irq: 0.0,
                    soft_irq: 0.0,
                    steal: 0.0,
                },
                CpuStates {
                    id: 2,
                    user: 0.0,
                    system: 0.0,
                    nice: 0.0,
                    idle: 100.0,
                    io_wait: 0.0,
                    hw_irq: 0.0,
                    soft_irq: 0.0,
                    steal: 0.0,
                },
                CpuStates {
                    id: 3,
                    user: 5.3,
                    system: 10.5,
                    nice: 0.0,
                    idle: 84.2,
                    io_wait: 0.0,
                    hw_irq: 0.0,
                    soft_irq: 0.0,
                    steal: 0.0,
                },
            ],
            virtual_memory: VirtualMemory {
                total: 3048.0,
                free: 2048.0,
                used: 0.0,
                available: 3392.8,
            },
        };

        let actual_delimiter = parse_sumamry_display_using_delimiter(input);
        let actual_regex = parse_summary_display_using_regex(input);
        let actual_parser_combinator_nom = parse_summary_display_using_parser_combinator_nom(input);

        assert_eq!(actual_delimiter, expected);
        assert_eq!(actual_regex, expected);
        assert_eq!(actual_parser_combinator_nom, expected);
    }
}
