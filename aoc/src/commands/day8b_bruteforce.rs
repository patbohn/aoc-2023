use super::{CommandImpl, DynError};
use clap::Parser;
use regex::Regex;
use std::path::PathBuf;
use std::{collections::HashMap, fs};

#[derive(Parser, Debug)]
pub struct Day8bBruteforce {
    #[clap(long, short)]
    input: PathBuf,
}

impl CommandImpl for Day8bBruteforce {
    fn main(&self) -> Result<(), DynError> {
        let input = fs::read_to_string(&self.input).unwrap();
        let (instructions, nodes) = parse_input(&input);

        let result = find_path_length(instructions, nodes).unwrap();
        println!("Day8b: {result}");
        Ok(())
    }
}

fn parse_input(input: &str) -> (Vec<Instruction>, HashMap<&str, Vec<&str>>) {
    let mut nodes: HashMap<&str, Vec<&str>> = HashMap::new();
    let parse_regex = Regex::new(r"(.*) = \((.*), (.*)\)").unwrap();
    let mut lines = input.lines();
    let instructions: Vec<Instruction> =
        lines.next().unwrap().chars().map(|c| Instruction::from_char(c)).collect();
    lines.next();
    for line in lines {
        let values = line.split_ascii_whitespace();
        let matches: Vec<&str> = parse_regex
            .captures(line)
            .unwrap()
            .iter()
            .map(|x| x.unwrap().as_str().trim())
            .collect();
        nodes.insert(matches[1], vec![matches[2], matches[3]]);
    }
    (instructions, nodes)
}
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Instruction {
    L,
    R,
}

impl Instruction {
    fn from_char(c: char) -> Instruction {
        match c {
            'L' => Instruction::L,
            'R' => Instruction::R,
            _ => panic!("Only L and R are allowed as instruction"),
        }
    }
}

fn find_path_length(
    instructions: Vec<Instruction>,
    nodes: HashMap<&str, Vec<&str>>,
) -> Result<usize, String> {
    let starts: Vec<&str> = nodes
        .keys()
        .cloned()
        .into_iter()
        .filter(|key| key.chars().last().unwrap() == 'A')
        .collect();
    println!("Total number of starts: {}", starts.len());
    let mut steps: usize = 0;
    let mut instruction_index: usize = 0;
    let mut currents: Vec<&str> = starts.clone();
    let first_instruction = instructions[0];

    let mut instruction = first_instruction;
    while !check_all_ends(&currents) {
        //dbg!(&currents);
        currents = currents
            .iter()
            .map(|current| *nodes.get(current).unwrap().get(instruction as usize).unwrap())
            .collect();
        //currents = nodes.get(&current).expect("step undefined").get(instruction as usize);

        if instruction_index == instructions.len() - 1 {
            instruction_index = 0;
        } else {
            instruction_index += 1;
        }
        steps = steps.checked_add(1).expect("Step count overflowing");
        instruction = instructions[instruction_index];
        if steps % 100000000 == 0 {
            println!("{} M steps reached", steps / 1000000);
        }
        if (instruction_index == 0) & (currents == starts) {
            panic!("Infinite loop!");
        }
    }
    return Ok(steps);
}

fn check_all_ends(currents: &Vec<&str>) -> bool {
    let num_end = currents.iter().filter(|current| current.chars().last().unwrap() == 'Z').count();
    return currents.len() == num_end;
}
//
//
// idea: find
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
        assert_eq!(instructions.get(0).unwrap(), &Instruction::L);
        assert_eq!(instructions.get(2).unwrap(), &Instruction::R);
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

    fn test_input3() {
        let input = "LR

    11A = (11B, XXX)
    11B = (XXX, 11Z)
    11Z = (11B, XXX)
    22A = (22B, XXX)
    22B = (22C, 22C)
    22C = (22Z, 22Z)
    22Z = (22B, 22B)
    XXX = (XXX, XXX)"
            .to_string();
        let (instructions, nodes) = parse_input(&input);
        let num_steps = find_path_length(instructions, nodes);
        assert_eq!(num_steps, Ok(6));
    }
}
