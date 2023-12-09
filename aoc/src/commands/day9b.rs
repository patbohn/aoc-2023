use super::{CommandImpl, DynError};
use clap::Parser;
use std::fs;
use std::path::PathBuf;

#[derive(Parser, Debug)]
pub struct Day9b {
    #[clap(long, short)]
    input: PathBuf,
}

impl CommandImpl for Day9b {
    fn main(&self) -> Result<(), DynError> {
        let mut result = 0;

        for line in fs::read_to_string(&self.input).unwrap().lines() {
            let data_sequence = Data::new(line);
            result += data_sequence.prediction;
        }
        println!("Day9b: {result}");
        Ok(())
    }
}

struct Data {
    measurement: Vec<i64>,
    differentiations: Vec<Vec<i64>>,
    prediction: i64,
}

impl Data {
    pub fn new(line: &str) -> Self {
        let measurement: Vec<i64> =
            line.split_ascii_whitespace().map(|x| x.parse().unwrap()).collect();
        let differentations: Vec<Vec<i64>> = calc_differentiations(&measurement);
        //dbg!(&differentations);
        let prediction = generate_previous(&differentations);

        Data { measurement: measurement, differentiations: differentations, prediction: prediction }
    }
}

fn calc_differentiations(measurement: &Vec<i64>) -> Vec<Vec<i64>> {
    let mut differentiations: Vec<Vec<i64>> = Vec::new();
    differentiations.push(measurement.iter().map(|x| *x as i64).collect());
    while differentiations.last().unwrap().iter().filter(|x| *x != &0i64).count() > 0 {
        differentiations.push(
            differentiations
                .last()
                .as_deref()
                .unwrap()
                .array_windows()
                .map(|window: &[i64; 2]| window.get(1).unwrap() - window.get(0).unwrap())
                .collect(),
        )
    }
    differentiations
}

//part B
fn generate_previous(differentiations: &Vec<Vec<i64>>) -> i64 {
    let mut result: i64 = 0;
    for differentation in differentiations.iter().rev() {
        result = differentation.first().unwrap() - result;
    }
    return result;
}
