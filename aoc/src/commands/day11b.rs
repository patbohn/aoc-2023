use super::{CommandImpl, DynError};
use clap::Parser;
use core::fmt;
use std::path::PathBuf;
use std::{collections::BTreeSet, fs};

#[derive(Parser, Debug)]
pub struct Day11b {
    #[clap(long, short)]
    input: PathBuf,
}

impl CommandImpl for Day11b {
    fn main(&self) -> Result<(), DynError> {
        let input = fs::read_to_string(&self.input).unwrap();
        let mut universe = Universe::create(&input);
        //dbg!(&universe);
        println!("{}", &universe);
        universe.expand(1000000);
        //dbg
        //println!("{}", &universe);
        let result = universe.get_total_distances();
        println!("Day11a: {result}");
        Ok(())
    }
}
#[derive(Debug)]
struct Galaxy {
    id: usize,
    location: (usize, usize),
}

#[derive(Debug)]
struct Universe {
    galaxies: Vec<Galaxy>,
    border: (usize, usize),
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut row = vec!["."; self.border.1];
        row.push("\n");
        let mut array = vec![row; self.border.0];
        for galaxy in &self.galaxies {
            let location = galaxy.location;
            *array.get_mut(location.0).unwrap().get_mut(location.1).unwrap() = "#";
        }
        let string: String = array.into_iter().flatten().collect();
        write!(f, "{}", string)
    }
}

impl Universe {
    fn create(input: &str) -> Self {
        let mut galaxies = Vec::new();
        let mut id = 0;
        let mut border: (usize, usize) = (0, 0);
        for (row_id, row) in input.lines().enumerate() {
            border.0 += 1;
            border.1 = row.len();
            let cols: Vec<usize> = row
                .chars()
                .enumerate()
                .filter(|(_, c)| c == &'#')
                .map(|(index, _)| index)
                .collect();
            for col in cols {
                galaxies.push(Galaxy { id: id, location: (row_id, col) });
                id += 1;
            }
        }
        dbg!(&border);
        Universe { galaxies: galaxies, border: border }
    }
    fn expand(&mut self, expansion_distance: usize) {
        let mut empty_columns: BTreeSet<usize> = (0..self.border.1 - 1).collect();
        let mut empty_rows: BTreeSet<usize> = (0..self.border.0 - 1).collect();
        for galaxy in &self.galaxies {
            let (row, col) = galaxy.location;
            empty_columns.remove(&col);
            empty_rows.remove(&row);
        }
        dbg!(&empty_columns);
        dbg!(&empty_rows);
        for mut galaxy in self.galaxies.iter_mut() {
            for column in empty_columns.iter().rev() {
                if &galaxy.location.1 > &column {
                    galaxy.location.1 += expansion_distance - 1;
                }
            }
        }
        for mut galaxy in self.galaxies.iter_mut() {
            for row in empty_rows.iter().rev() {
                if &galaxy.location.0 > &row {
                    galaxy.location.0 += expansion_distance - 1;
                }
            }
        }
        self.border.0 += empty_rows.len() * (expansion_distance - 1);
        self.border.1 += empty_columns.len() * (expansion_distance - 1);
    }

    fn get_total_distances(&self) -> usize {
        let mut total_distance = 0;
        for galaxy1 in &self.galaxies {
            for galaxy2 in &self.galaxies {
                if galaxy1.id < galaxy2.id {
                    total_distance += galaxy1.location.1.abs_diff(galaxy2.location.1);
                    total_distance += galaxy1.location.0.abs_diff(galaxy2.location.0);
                }
            }
        }
        total_distance
    }
}
