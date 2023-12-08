use super::{CommandImpl, DynError};
use clap::Parser;
use regex::Regex;
use std::path::PathBuf;
use std::{
    collections::{HashMap, HashSet},
    fs,
};

#[derive(Parser, Debug)]
pub struct Day8b {
    #[clap(long, short)]
    input: PathBuf,
}

impl CommandImpl for Day8b {
    fn main(&self) -> Result<(), DynError> {
        let mut result = 0;
        let input = fs::read_to_string(&self.input).unwrap();
        let (instructions, nodes) = parse_input(&input);
        let starts: Vec<&str> =
            nodes.keys().cloned().filter(|key| key.chars().last().unwrap() == 'A').collect();
        //dbg!(&starts);
        let mut per_start_results = Vec::new();
        for start in &starts {
            per_start_results.push(find_cycle_length_and_z(&instructions, &nodes, start).unwrap())
        }
        //dbg!(per_start_results);
        let result = find_steps_required(per_start_results);
        println!("Day8a: {result}");
        Ok(())
    }
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

// check whether each node is there once in L and once in R
fn confirm_cyclic_graph(nodes: HashMap<&str, (&str, &str)>) -> bool {
    todo!();
}

fn find_cycle_length_and_z(
    instructions: &Vec<Instruction>,
    nodes: &HashMap<&str, Vec<&str>>,
    start: &str,
) -> Result<(usize, usize, usize), String> {
    let mut z_locations: Vec<usize> = Vec::new();
    let mut steps: usize = 0;
    let mut instruction_index: usize = 0;
    let mut current: &str = start;
    let mut start_positions: HashMap<&str, usize> = HashMap::new();
    let mut last_start = start;
    let first_instruction = instructions[0];
    //dbg!(&nodes);
    //dbg!(&instructions);
    let mut instruction = first_instruction;
    'outer: while steps < 10_usize.pow(9) {
        //dbg!(&current);
        if current.chars().last().unwrap() == 'Z' {
            z_locations.push(steps.clone());
        }
        /*
        dbg!(&steps);
        dbg!(&current);
        dbg!(&instruction);
        dbg!(&instruction_index);
        dbg!(&start_positions);
        */

        if instruction_index == 0 {
            if start_positions.contains_key(current) {
                let start_position = start_positions.get(current).unwrap().clone();
                /* println!(
                    "For start {} found a repeating sequence starting from step {} to step {} (length {}).",
                    start, start_position, steps, steps-start_position
                ); */
                //println!("Detected __Z positions at {:?}", z_locations);
                //dbg!(&z_locations);
                return Ok((
                    start_position,
                    steps - start_position,
                    z_locations.get(0).unwrap() - start_position,
                ));
            } else {
                start_positions.insert(current, steps);
            }
        }
        current = nodes.get(current).unwrap().get(instruction as usize).unwrap();
        steps += 1;

        if instruction_index == instructions.len() - 1 {
            instruction_index = 0
        } else {
            instruction_index += 1
        }
        instruction = instructions[instruction_index];
    }
    Err("Could not find circular transversion".to_string())
}

// This requires a lot of assumptions, mainly that all starts only encounter
// only one __Z, and that the length of the cycle that they enter is equal
// to the number of steps until they first encountered Z. In other words,
// Steps_encountering_Z(n) = Z_pos + Cycle_length x n
// can simplify to Cycle_length x (n+1)
// this in turn enables detection by the lowest common denominator
fn find_steps_required(data: Vec<(usize, usize, usize)>) -> usize {
    let factors: Vec<usize> = data.iter().map(|x| x.1).collect();
    //dbg!(&factors);
    let lowest = lcm(&factors);
    return lowest;
}

// lcm of n numbers
// from: https://github.com/TheAlgorithms/Rust/blob/master/src/math/lcm_of_n_numbers.rs
pub fn lcm(nums: &[usize]) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}
//
//
//
//
//
//
