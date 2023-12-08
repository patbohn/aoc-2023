use super::{CommandImpl, DynError};
use clap::Parser;
use regex::Regex;
use std::path::PathBuf;
use std::{collections::HashMap, fs};

#[derive(Parser, Debug)]
pub struct Day8a {
    #[clap(long, short)]
    input: PathBuf,
}

impl CommandImpl for Day8a {
    fn main(&self) -> Result<(), DynError> {
        let mut result = 0;
        let input = fs::read_to_string(&self.input).unwrap();
        let (instructions, nodes) = parse_input(&input);
        let result = find_path_length(instructions, nodes).unwrap();
        println!("Day8a: {result}");
        Ok(())
    }
}

fn parse_input(input: &str) -> (Vec<char>, HashMap<&str, (&str, &str)>) {
    let mut nodes: HashMap<&str, (&str, &str)> = HashMap::new();
    let parse_regex = Regex::new(r"(.*) = \((.*), (.*)\)").unwrap();
    let mut lines = input.lines();
    let instructions: Vec<char> = lines.next().unwrap().chars().collect();
    lines.next();
    for line in lines {
        let values = line.split_ascii_whitespace();
        let matches: Vec<&str> = parse_regex
            .captures(line)
            .unwrap()
            .iter()
            .map(|x| x.unwrap().as_str().trim())
            .collect();
        nodes.insert(matches[1], (matches[2], matches[3]));
    }
    (instructions, nodes)
}

fn find_path_length(
    instructions: Vec<char>,
    nodes: HashMap<&str, (&str, &str)>,
) -> Result<usize, String> {
    let start = "AAA";
    let end = "ZZZ";
    let mut steps: usize = 0;
    let mut instruction_index: usize = 0;
    let mut current: &str = start;
    let first_instruction = instructions[0];
    //dbg!(&nodes);
    //dbg!(&instructions);
    let mut instruction = first_instruction;
    while current != end {
        //dbg!(&current);
        match instruction {
            'L' => {
                current = nodes.get(&current).expect("step undefined").0;
            }
            'R' => {
                current = nodes.get(&current).expect("step undefined").1;
            }
            _ => return Err("undefined instruction".to_string()),
        }
        if instruction_index == instructions.len() - 1 {
            instruction_index = 0;
        } else {
            instruction_index += 1;
        }
        steps += 1;
        instruction = instructions[instruction_index];
        if (current == start) & (instruction == first_instruction) {
            return Err("Infinite loop".to_string());
        }
    }
    return Ok(steps);
}
//
//
//
//
//
//
// Tests follow here
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing() {
        let input = "LLR

        AAA = (BBB, BBB)
        BBB = (AAA, ZZZ)
        ZZZ = (ZZZ, ZZZ)"
            .to_string();
        let (instructions, nodes) = parse_input(&input);
        assert_eq!(instructions, vec!['L', 'L', 'R']);
    }
    #[test]
    fn test_input1() {
        let input = "RL

        AAA = (BBB, CCC)
        BBB = (DDD, EEE)
        CCC = (ZZZ, GGG)
        DDD = (DDD, DDD)
        EEE = (EEE, EEE)
        GGG = (GGG, GGG)
        ZZZ = (ZZZ, ZZZ)"
            .to_string();
        let (instructions, nodes) = parse_input(&input);
        let num_steps = find_path_length(instructions, nodes);
        assert_eq!(num_steps, Ok(2));
    }
    #[test]
    fn test_input2() {
        let input = "LLR

        AAA = (BBB, BBB)
        BBB = (AAA, ZZZ)
        ZZZ = (ZZZ, ZZZ)"
            .to_string();
        let (instructions, nodes) = parse_input(&input);
        let num_steps = find_path_length(instructions, nodes);
        assert_eq!(num_steps, Ok(6));
    }
}
