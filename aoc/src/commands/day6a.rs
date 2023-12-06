use super::{CommandImpl, DynError};
use clap::Parser;
use std::fs;
use std::path::PathBuf;

#[derive(Parser, Debug)]
pub struct Day6a {
    #[clap(long, short)]
    input: PathBuf,
}

impl CommandImpl for Day6a {
    fn main(&self) -> Result<(), DynError> {
        let races = parse_input(&self.input);
        for race in &races {
            race.print_data();
            race.print_charging_limits();
            let num_wins = race.num_possible_wins();
            println!("Num possible wins: {num_wins}");
        }
        let num_wins: Vec<usize> = races.iter().map(|x| x.num_possible_wins().to_owned()).collect();
        for win in &num_wins {
            println!("{win}");
        }
        let result: usize = num_wins.into_iter().product();
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
        let lower_limit = 0.5_f64 * t_total - (0.25_f64 * t_total.powi(2) as f64 - distance).sqrt();
        let upper_limit = 0.5_f64 * t_total + (0.25_f64 * t_total.powi(2) as f64 - distance).sqrt();
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
            Err(_) => 0 as usize,
        }
    }
    fn print_data(&self) {
        let t_total = self.t_total;
        let distance = self.min_distance;
        println!("t_total: {t_total}, distance: {distance}");
    }

    fn print_charging_limits(&self) {
        let t_total = self.t_total as f64;
        let distance = self.min_distance as f64;
        let lower_limit = 0.5_f64 * t_total - (0.25_f64 * t_total.powi(2) as f64 - distance).sqrt();
        let upper_limit = 0.5_f64 * t_total + (0.25_f64 * t_total.powi(2) as f64 - distance).sqrt();
        println!("lower limit:{lower_limit}, upper limit: {upper_limit}");
    }
}

fn parse_input(path: &PathBuf) -> Vec<Boatrace> {
    let lines = fs::read_to_string(path).unwrap();
    let mut split_lines = lines.lines();
    let times: Vec<usize> = split_lines
        .next()
        .unwrap()
        .split(":")
        .nth(1)
        .unwrap()
        .split_ascii_whitespace()
        .map(|x| x.parse().expect("Could not convert times to usize"))
        .collect();
    let mut distances: Vec<usize> = split_lines
        .next()
        .unwrap()
        .split(":")
        .nth(1)
        .unwrap()
        .split_ascii_whitespace()
        .map(|x| x.parse().expect("Could not convert distances to usize"))
        .collect();
    let mut races: Vec<Boatrace> = Vec::new();
    for (time, distance) in times.iter().zip(distances.iter()) {
        races.push(Boatrace { t_total: time.to_owned(), min_distance: distance.to_owned() })
    }
    races
}

/*
fn product_of_ints_in_range(lower_bound: f64, upper_bound: f64) -> usize {
    let range: std::ops::Range<usize> = lower_bound.ceil() as usize..upper_bound.ceil() as usize;
    range.product()
}
*/
