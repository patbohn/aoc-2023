use clap::Parser;
use std::fs;
use std::path::PathBuf;

use super::{CommandImpl, DynError};

#[derive(Parser, Debug)]
pub struct Day0 {
    #[clap(long, short)]
    input: PathBuf,
}

impl CommandImpl for Day0 {
    fn main(&self) -> Result<(), DynError> {
        let mut result = 0;

        for line in fs::read_to_string(&self.input).unwrap().lines() {
            //do something
        }
        println!("Day0: {result}");
        Ok(())
    }
}
