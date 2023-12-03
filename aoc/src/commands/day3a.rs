use clap::Parser;
use regex::Regex;
use std::fs;
use std::path::PathBuf;

use super::{CommandImpl, DynError};

#[derive(Parser, Debug)]
pub struct Day3a {
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

impl CommandImpl for Day3a {
    fn main(&self) -> Result<(), DynError> {
        let mut numbers_per_row: Vec<Vec<Number>> = Vec::new();
        let mut symbols_per_row: Vec<Vec<Symbol>> = Vec::new();

        for line in fs::read_to_string(&self.input).unwrap().lines() {
            numbers_per_row.push(find_numbers(line));
            symbols_per_row.push(find_symbols(line));
        }

        let result = check_number_for_symbol(numbers_per_row, symbols_per_row);
        println!("Day3a: {result}");
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

fn check_number_for_symbol(
    numbers_per_row: Vec<Vec<Number>>,
    symbols_per_row: Vec<Vec<Symbol>>,
) -> usize {
    let mut total_sum: usize = 0;
    let num_rows = numbers_per_row.len();
    println!("Total number of rows: {num_rows}");
    for (row, numbers_in_row) in numbers_per_row.iter().enumerate() {
        println!("Checking row {row}");
        for number in numbers_in_row {
            println!("Checking number {} {}:{}", number.value, number.start_col, number.end_col);
            let mut contacts_symbol = false;
            if row != 0 {
                //println!("Checking row above for symbols");
                for symbol in symbols_per_row[row - 1].iter() {
                    if (symbol.col >= number.start_col.saturating_sub(1))
                        & (symbol.col <= number.end_col + 1)
                    {
                        contacts_symbol = true;
                    }
                }
            }
            //println!("Checking this row for symbols");
            for symbol in symbols_per_row[row].iter() {
                if (symbol.col == number.start_col.saturating_sub(1))
                    | (symbol.col == number.end_col + 1)
                {
                    contacts_symbol = true;
                }
            }
            if row < numbers_per_row.len() - 1 {
                //println!("Checking row below for symbols");
                for symbol in symbols_per_row[row + 1].iter() {
                    if (symbol.col >= number.start_col.saturating_sub(1))
                        & (symbol.col <= number.end_col + 1)
                    {
                        contacts_symbol = true;
                    }
                }
            }
            if contacts_symbol {
                println!(
                    "Number {} in row {} col {} is next to a symbol",
                    number.value, row, number.start_col
                );
                total_sum += number.value as usize;
            }
        }
    }
    return total_sum;
}
