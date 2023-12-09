use super::{CommandImpl, DynError};
use clap::Parser;
use num::integer::lcm;
use regex::Regex;
use std::cmp::Ordering;
use std::path::PathBuf;
use std::{
    collections::{HashMap, HashSet},
    fs,
};

#[derive(Parser, Debug)]
pub struct Day8bsbf {
    #[clap(long, short)]
    input: PathBuf,
}

impl CommandImpl for Day8bsbf {
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
        //dbg!(&per_start_results);
        let mut combined_zfunc = per_start_results.pop().unwrap();
        for zfunc in per_start_results {
            combined_zfunc = ZFUNC::combine_z_functs(&combined_zfunc, &zfunc);
            //dbg!(&combined_zfunc);
        }
        //dbg!(&combined_zfunc);
        println!("Day8b: {}", combined_zfunc.offset);
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
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct ZFUNC {
    offset: usize,
    cycle_length: usize,
}

impl ZFUNC {
    fn combine_z_functs(func_a: &ZFUNC, func_b: &ZFUNC) -> Self {
        let mut i_1: usize = 0;
        let mut i_2: usize = 0;
        'outer: loop {
            let z_1 = func_a.get_nth_z(&i_1);
            let z_2 = func_b.get_nth_z(&i_2);
            match z_1.cmp(&z_2) {
                Ordering::Less => i_1 += 1,
                Ordering::Greater => i_2 += 1,
                Ordering::Equal => break 'outer,
            }
        }

        ZFUNC {
            offset: func_a.get_nth_z(&i_1).clone(),
            cycle_length: lcm(func_a.cycle_length, func_b.cycle_length),
        }
    }

    fn get_nth_z(&self, n: &usize) -> usize {
        self.offset + self.cycle_length * n
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

fn find_cycle_length_and_z(
    instructions: &Vec<Instruction>,
    nodes: &HashMap<&str, Vec<&str>>,
    start: &str,
) -> Result<ZFUNC, String> {
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

        if instruction_index == 0 {
            if start_positions.contains_key(current) {
                let start_position = start_positions.get(current).unwrap().clone();

                return Ok((ZFUNC {
                    offset: z_locations.get(0).unwrap().clone(),
                    cycle_length: steps - start_position,
                }));
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
