use clap::Parser;
use std::fs;
use std::path::PathBuf;

use super::{CommandImpl, DynError};

use std::collections::HashMap;

#[derive(Parser, Debug)]
pub struct Day1a {
    #[clap(long, short)]
    input: PathBuf,
}

impl CommandImpl for Day1a {
    fn main(&self) -> Result<(), DynError> {
        let mut total_sum = 0;

        for line in fs::read_to_string(&self.input).unwrap().lines() {
            let (first, last) = find_first_and_last_int(line);
            println!("{first} {last}");
            total_sum += 10 * first + last;
        }
        println!("Day1a: {total_sum}");
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
        }
    }
    (first, last)
}
