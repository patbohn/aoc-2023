use clap::Parser;
use regex::Regex;
use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;

use super::{CommandImpl, DynError};

#[derive(Parser, Debug)]
pub struct Day4a {
    #[clap(long, short)]
    input: PathBuf,
}

impl CommandImpl for Day4a {
    fn main(&self) -> Result<(), DynError> {
        let mut result = 0;

        for line in fs::read_to_string(&self.input).unwrap().lines() {
            //do something
            let (card_id, num_matches) = get_num_matching(line);
            let score = calc_score(num_matches);
            result += score;
        }
        println!("Day4a: {result}");
        Ok(())
    }
}

fn get_card_id(line: &str) -> usize {
    /*
    let card_id: u32 = line.split(":").collect::<Vec<&str>>()[0]
    .split(" ")
    .collect::<Vec<&str>>()
    .iter()
    .filter(|x| x.len() > 0)[0]
    .parse()
    .expect("Getting card id failed with line {line}");
    */
    let re = Regex::new(r"Card\s*([0-9]+):").unwrap();
    let number_match: usize =
        re.captures(line).unwrap().get(1).expect("No Card ID found").as_str().parse().unwrap();
    println!("Match: {number_match}");

    return number_match;
}

fn get_num_matching(line: &str) -> (usize, usize) {
    let card_id = get_card_id(line);
    println!("Card id: {card_id}");
    let number_sets: Vec<&str> =
        line.split(": ").collect::<Vec<&str>>()[1].split(" | ").collect::<Vec<&str>>();
    println!("Length of number_sets: {}", number_sets.len());
    println!("Number set 1: {}", number_sets[0]);
    println!("Number set 2: {}", number_sets[1]);
    let picked_numbers: HashSet<u8> = number_sets[0]
        .split(" ")
        .collect::<Vec<&str>>()
        .iter()
        .filter(|x| x.len() > 0)
        .map(|x| x.parse().unwrap())
        .collect::<HashSet<u8>>();
    let winning_numbers: HashSet<u8> = number_sets[1]
        .split(" ")
        .collect::<Vec<&str>>()
        .iter()
        .filter(|x| x.len() > 0)
        .map(|x| x.parse().unwrap())
        .collect::<HashSet<u8>>();
    let mut matching_numbers: usize = 0;
    for number in picked_numbers {
        if winning_numbers.contains(&number) {
            println!("Number {number} is in winning set.");
            matching_numbers += 1
        }
    }

    return (card_id, matching_numbers);
}

fn calc_score(num_matching: usize) -> usize {
    if num_matching > 0 {
        return 2_i32.pow(num_matching as u32 - 1_u32) as usize;
    } else {
        return 0;
    }
}
