use clap::Parser;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use super::{CommandImpl, DynError};

#[derive(Parser, Debug)]
pub struct Day2a {
    #[clap(long, short)]
    input: PathBuf,
}

impl CommandImpl for Day2a {
    fn main(&self) -> Result<(), DynError> {
        let mut total_sum = 0;

        for line in fs::read_to_string(&self.input).unwrap().lines() {
            total_sum += get_game_id_if_possible(line);
        }
        println!("Day2a: {total_sum}");
        Ok(())
    }
}

fn get_game_id_if_possible(line: &str) -> i32 {
    let max_limits: HashMap<&str, i32> = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);
    let mut game_possible = true;

    let game_id: i32 =
        line.split(":").collect::<Vec<&str>>()[0].split("Game ").collect::<Vec<&str>>()[1]
            .parse()
            .unwrap();

    for draw in line.split(": ").collect::<Vec<&str>>()[1].split("; ").collect::<Vec<&str>>() {
        for num_color in draw.split(", ").collect::<Vec<&str>>() {
            //println!("{num_color}");
            let split_num_color = num_color.split(" ").collect::<Vec<&str>>();

            let color: &str = split_num_color[1];
            let num: i32 = split_num_color[0].parse().unwrap();

            match max_limits.get(color) {
                Some(limit) => {
                    if limit < &num {
                        game_possible = false;
                    }
                }
                None => {
                    println!("Could not find color {color} in max_limit dict")
                }
            }
            //println!("{color}: {num}")
        }
    }
    if game_possible {
        println! {"Game {game_id} is possible!"}
        return game_id;
    } else {
        println! {"Game {game_id} is not possible!"}
        return 0;
    }
}
