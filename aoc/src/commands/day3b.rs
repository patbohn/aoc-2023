use clap::Parser;
use regex::Regex;
use std::fs;
use std::path::PathBuf;

use super::{CommandImpl, DynError};

#[derive(Parser, Debug)]
pub struct Day3b {
    #[clap(long, short)]
    input: PathBuf,
}

struct Number {
    value: usize,
    start_col: usize,
    end_col: usize,
}

struct Symbol {
    value: char,
    col: usize,
}

impl CommandImpl for Day3b {
    fn main(&self) -> Result<(), DynError> {
        let mut numbers_per_row: Vec<Vec<Number>> = Vec::new();
        let mut symbols_per_row: Vec<Vec<Symbol>> = Vec::new();

        for line in fs::read_to_string(&self.input).unwrap().lines() {
            numbers_per_row.push(find_numbers(line));
            symbols_per_row.push(find_symbols(line));
        }

        let result = find_gear_ratio(numbers_per_row, symbols_per_row);
        println!("Day3b: {result}");
        Ok(())
    }
}

fn find_numbers(line: &str) -> Vec<Number> {
    let mut numbers: Vec<Number> = Vec::new();
    let re = Regex::new(r"([0-9]+)").unwrap();
    for number_match in re.find_iter(line) {
        numbers.push(Number {
            value: number_match.as_str().parse().unwrap(),
            start_col: number_match.start(),
            end_col: number_match.end() - 1,
        })
    }
    return numbers;
}

fn find_symbols(line: &str) -> Vec<Symbol> {
    let mut symbols: Vec<Symbol> = Vec::new();
    let re = Regex::new(r"([^0-9.])").unwrap();
    for symbol_match in re.find_iter(line) {
        symbols.push(Symbol {
            value: symbol_match.as_str().parse().unwrap(),
            col: symbol_match.start(),
        })
    }
    return symbols;
}

fn find_gear_ratio(numbers_per_row: Vec<Vec<Number>>, symbols_per_row: Vec<Vec<Symbol>>) -> usize {
    let mut total_sum: usize = 0;
    let num_rows = numbers_per_row.len();
    for (row, symbols_in_row) in symbols_per_row.iter().enumerate() {
        println!("Checking row {row}");
        for symbol in symbols_in_row {
            println!("Checking symbol {} col: {}", symbol.value, symbol.col);
            if symbol.value == '*' {
                println!("Found gear!");
                let mut contacting_nums: Vec<usize> = Vec::new();
                if row != 0 {
                    //println!("Checking row above for numbers");
                    for number in numbers_per_row[row - 1].iter() {
                        if (symbol.col >= number.start_col.saturating_sub(1))
                            & (symbol.col <= number.end_col + 1)
                        {
                            contacting_nums.push(number.value);
                        }
                    }
                }
                //println!("Checking this row for symbols");
                for number in numbers_per_row[row].iter() {
                    if (symbol.col == number.start_col.saturating_sub(1))
                        | (symbol.col == number.end_col + 1)
                    {
                        contacting_nums.push(number.value);
                    }
                }
                if row < numbers_per_row.len() - 1 {
                    //println!("Checking row below for symbols");
                    for number in numbers_per_row[row + 1].iter() {
                        if (symbol.col >= number.start_col.saturating_sub(1))
                            & (symbol.col <= number.end_col + 1)
                        {
                            contacting_nums.push(number.value);
                        }
                    }
                }
                match contacting_nums.len() {
                    0 | 1 => println!("No two numbers matching"),
                    2 => {
                        println!("Found two numbers!");
                        total_sum += contacting_nums[0] * contacting_nums[1];
                    }
                    _ => println!("More than 2 numbers contacting the gear!"),
                }
            }
        }
    }
    total_sum
}
