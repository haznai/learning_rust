use crate::day01::Solution;
use itertools::Itertools;
use pest::Parser;
use pest_derive::Parser;

// struct declarations
#[derive(Parser)]
#[grammar = "day02grammar.pest"]
pub struct OPcodeParser;
pub struct Day02solution;

impl Solution<usize> for Day02solution {
    fn solve_first_part(inputs: &[String]) -> usize {
        // input is only one line
        let input: &str = inputs[0].as_str();
        let mut parsed_thing = OPcodeParser::parse(Rule::opcodes, input)
            .expect("parsing unsuccessful")
            .map(|pair| pair.as_str().parse::<usize>().unwrap())
            .collect_vec();

        // aoc wants us to do this
        parsed_thing[1] = 12;
        parsed_thing[2] = 2;

        for i in (0..parsed_thing.len()).step_by(4) {
            // only consider this when there are stil 4 places to go
            if i <= parsed_thing.len() - 4 {
                match parsed_thing[i] {
                    // opcode for addition
                    1 => {
                        let index = parsed_thing[i + 3];
                        parsed_thing[index] =
                            parsed_thing[parsed_thing[i + 1]] + parsed_thing[parsed_thing[i + 2]];
                    }
                    // opcode for multiplication
                    2 => {
                        let index = parsed_thing[i + 3];
                        parsed_thing[index] =
                            parsed_thing[parsed_thing[i + 1]] * parsed_thing[parsed_thing[i + 2]];
                    }
                    // opcode for halting
                    99 => break,
                    // values that aren't opcodes shouldn't happen
                    _ => panic!(
                        "paniced with i: {} and parsed_thing[i]:{}",
                        i, parsed_thing[i],
                    ),
                }
            } else {
                break;
            }
        }
        // solutions wants the value of the first field
        parsed_thing[0]
    }

    fn solve_second_part(inputs: &[String]) -> usize {
        todo!()
    }
}
