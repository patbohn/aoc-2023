use clap::Parser;
use regex::Regex;
use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;

use super::{CommandImpl, DynError};

#[derive(Parser, Debug)]
pub struct Day4b {
    #[clap(long, short)]
    input: PathBuf,
}

struct CardGame {
    id: usize,
    num_matches: usize,
}

impl CommandImpl for Day4b {
    fn main(&self) -> Result<(), DynError> {
        let mut result = 0;
        let mut game_results: Vec<CardGame> = Vec::new();
        for line in fs::read_to_string(&self.input).unwrap().lines() {
            //do something

            let (card_id, num_matches) = get_num_matching(line);
            game_results.push(CardGame { id: card_id, num_matches: num_matches });
        }
        let score = calc_full_score(game_results);
        println!("Day4b: {score}");
        Ok(())
    }
}

fn get_card_id(line: &str) -> usize {
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

fn calc_full_score(game_results: Vec<CardGame>) -> usize {
    println!("Length of game results is {}", game_results.len());
    let mut num_copies: Vec<usize> = vec![1; game_results.len()];
    //println!("Length of num_copies vector is {}", num_copies.len());
    for game_id in 0..game_results.len() - 1 {
        let current_game = &game_results[game_id];
        println!("Game {} has {} copies", game_id, num_copies[game_id]);
        for _ in 0..num_copies[game_id] {
            for i in game_id..(game_id + current_game.num_matches) {
                let increasing_id = i + 1;
                //println!("Increasing number of copies for {}", increasing_id);
                if increasing_id < game_results.len() {
                    num_copies[i + 1] += 1;
                }
            }
        }
    }
    let mut total_num_games: usize = 0;
    for copies in num_copies.iter() {
        total_num_games += copies;
    }
    return total_num_games;
}