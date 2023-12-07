use super::{CommandImpl, DynError};
use clap::Parser;
use std::fs;
use std::path::PathBuf;

#[derive(Parser, Debug)]
pub struct Day6b {
    #[clap(long, short)]
    input: PathBuf,
}

impl CommandImpl for Day6b {
    fn main(&self) -> Result<(), DynError> {
        let race = parse_input(&self.input);
        let result: usize = race.num_possible_wins();
        println!("Day6a: {result}");
        Ok(())
    }
}

#[derive(Copy, Clone)]
struct Boatrace {
    t_total: usize,
    min_distance: usize,
}

impl Boatrace {
    fn find_charging_limits(&self) -> Result<(f64, f64), String> {
        let t_total = self.t_total as f64;
        let distance = self.min_distance as f64;
        let square_root = (0.25_f64 * t_total.powi(2) as f64 - distance).sqrt();
        let lower_limit = 0.5_f64 * t_total - square_root;
        let upper_limit = 0.5_f64 * t_total + square_root;
        Ok((lower_limit, upper_limit))
    }
    fn num_possible_wins(&self) -> usize {
        match self.find_charging_limits() {
            Ok((lower_bound, upper_bound)) => {
                if lower_bound.fract() != 0.0 {
                    upper_bound.ceil() as usize - lower_bound.ceil() as usize
                } else if upper_bound.ceil() as usize == lower_bound.ceil() as usize {
                    0
                } else {
                    upper_bound.ceil() as usize - lower_bound.ceil() as usize - 1
                }
            }
            Err(_) => 0_usize,
        }
    }
}

fn parse_input(path: &PathBuf) -> Boatrace {
    let lines = fs::read_to_string(path).unwrap();
    let mut split_lines = lines.lines();
    let time: usize = split_lines
        .next()
        .unwrap()
        .split(":")
        .nth(1)
        .unwrap()
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>()
        .parse()
        .expect("Could not convert times to usize");

    let distance: usize = split_lines
        .next()
        .unwrap()
        .split(":")
        .nth(1)
        .unwrap()
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>()
        .parse()
        .expect("Could not convert distances to usize");
    Boatrace { t_total: time.to_owned(), min_distance: distance.to_owned() }
}

/*
fn product_of_ints_in_range(lower_bound: f64, upper_bound: f64) -> usize {
    let range: std::ops::Range<usize> = lower_bound.ceil() as usize..upper_bound.ceil() as usize;
    range.product()
}
*/
