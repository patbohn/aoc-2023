use clap::Parser;
use std::fs;
use std::path::PathBuf;

use super::{CommandImpl, DynError};

use std::collections::HashMap;

#[derive(Parser, Debug)]
pub struct Day1b {
    #[clap(long, short)]
    input: PathBuf,
}

impl CommandImpl for Day1b {
    fn main(&self) -> Result<(), DynError> {
        let mut total_sum = 0;

        for line in fs::read_to_string(&self.input).unwrap().lines() {
            let (first, last) = find_first_and_last_int(line);
            println!("{first} {last}");
            total_sum += 10 * first + last;
        }
        println!("Day1b: {total_sum}");
        Ok(())
    }
}

fn find_first_and_last_int(line: &str) -> (i32, i32) {
    let mut first: i32 = -1;
    let mut found_first = false;
    let mut last: i32 = -1;

    for (i, char) in line.chars().enumerate() {
        if char.is_ascii_digit() {
            if !found_first {
                first = char.to_digit(10).unwrap() as i32;
                found_first = true;
            }
            last = char.to_digit(10).unwrap() as i32;
        } else {
            match convert_str_to_number(&line[i..]) {
                Some(i) => {
                    if !found_first {
                        first = i;
                        found_first = true;
                    }
                    last = i;
                }
                None => {}
            }
        }
    }
    (first, last)
}

fn convert_str_to_number(line: &str) -> Option<i32> {
    let str_to_int: HashMap<&str, i32> = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
        ("zero", 0),
    ]);

    for (key, value) in str_to_int {
        let length = key.len();
        if length <= line.len() {
            if key == &line[..length] {
                return Some(value);
            }
        }
    }
    None
}
