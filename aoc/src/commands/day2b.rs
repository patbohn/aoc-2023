use clap::Parser;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use super::{CommandImpl, DynError};

#[derive(Parser, Debug)]
pub struct Day2b {
    #[clap(long, short)]
    input: PathBuf,
}

impl CommandImpl for Day2b {
    fn main(&self) -> Result<(), DynError> {
        let mut total_sum = 0;

        for line in fs::read_to_string(&self.input).unwrap().lines() {
            let game_id = find_fewest_possible(line);
            total_sum += game_id;
        }
        println!("Day2b: {total_sum}");
        Ok(())
    }
}

fn find_fewest_possible(line: &str) -> i32 {
    let mut min_limits: HashMap<&str, i32> = HashMap::from([("red", 0), ("green", 0), ("blue", 0)]);

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

            match min_limits.get(color) {
                Some(limit) => {
                    if limit < &num {
                        println!("{color}, {num}");
                        *min_limits.get_mut(color).unwrap() = num;
                    }
                }
                None => {
                    println!("Could not find color {color} in max_limit dict")
                }
            }
            //println!("{color}: {num}")
        }
    }
    let mut power = 1;
    for (color, num) in min_limits {
        power = power * num;
        //println!("{color} {num}")
    }

    return power;
}
