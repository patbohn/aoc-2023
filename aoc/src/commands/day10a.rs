use clap::Parser;
use core::fmt;
use std::fs;
use std::path::PathBuf;

use super::{CommandImpl, DynError};

#[derive(Parser, Debug)]
pub struct Day10a {
    #[clap(long, short)]
    input: PathBuf,
}

impl CommandImpl for Day10a {
    fn main(&self) -> Result<(), DynError> {
        let mut result = 0;
        let field = Field::from_input(fs::read_to_string(&self.input).unwrap());
        //dbg!(&field);
        let circle = &field.find_circle();
        for (direction, path) in circle {
            println!("When walking from {direction}");
            match path {
                Some(p) => {
                    println!(
                        "Length of circle is {}, max distance is thus {}",
                        p.len(),
                        p.len() / 2
                    );
                    let max_dist_position = p.get(p.len() / 2).unwrap();
                    println!(
                        "Max distance is at X: {}, Y: {}",
                        max_dist_position.1, max_dist_position.0
                    );
                }
                None => {
                    println!("No circle starting from {direction}")
                }
            }
        }

        //dbg!(&circle);
        println!("Day10a: {result}");
        Ok(())
    }
}

#[derive(Debug)]
struct Field {
    pipes: Vec<Vec<Pipe>>, //note that this is y,x
    start: Option<(usize, usize)>,
}

impl Field {
    fn from_input(input: String) -> Self {
        let field: Vec<Vec<Pipe>> =
            input.lines().map(|row| row.chars().map(|c| Pipe::from_char(c)).collect()).collect();

        let mut start_position = None;
        for (i, row) in field.iter().enumerate() {
            for (j, &element) in row.iter().enumerate() {
                if element == Pipe::Start {
                    start_position = Some((i, j))
                }
            }
        }
        Field { pipes: field, start: start_position }
    }

    fn limits(&self) -> (usize, usize) {
        let max_y = self.pipes.len();
        let max_x = self.pipes.get(0).unwrap().len();
        (max_y, max_x)
    }

    fn find_circle(&self) -> Vec<(Direction, Option<Vec<(usize, usize)>>)> {
        let start_position = self.start.expect("Start undefined");
        let mut current_position = start_position;
        let mut results: Vec<(Direction, Option<Vec<(usize, usize)>>)> = Vec::new();
        for start_direction in Pipe::Start.directions().unwrap() {
            current_position = start_position;
            println!("Walking {start_direction} from Start");
            let mut entry_direction = start_direction;
            //dbg!(&entry_direction);
            let mut path: Vec<(usize, usize)> = vec![current_position];
            //check if the pipe we're walking to actually allows entering from this direction

            'walk: loop {
                match entry_direction.walk(current_position, self.limits()) {
                    Some(position) => {
                        //dbg!(&position);
                        if position == start_position {
                            println!("Found circle starting from {}", start_direction);
                            results.push((start_direction, Some(path)));
                            break 'walk;
                        } else {
                            current_position = position.clone();
                        }
                    }
                    None => {
                        results.push((start_direction, None));
                        println!("Tried to walk off the edge of the field {}", start_direction);
                        break 'walk;
                    }
                }

                path.push(current_position);
                let pipe =
                    self.pipes.get(current_position.0).unwrap().get(current_position.1).unwrap();
                match pipe.transverse(&entry_direction) {
                    Some(direction) => entry_direction = direction,
                    None => {
                        results.push((start_direction, None));
                        println!(
                            "Encountered mismatching pipe when starting from {}",
                            start_direction
                        );
                        break 'walk;
                    }
                }
            }
        }
        results
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Pipe {
    None,
    NS,
    EW,
    NE,
    ES,
    SW,
    WN,
    Start,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Direction {
    N,
    E,
    S,
    W,
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Direction::N => write!(f, "N"),
            Direction::E => write!(f, "E"),
            Direction::S => write!(f, "S"),
            Direction::W => write!(f, "W"),
        }
    }
}

impl Direction {
    fn walk(
        &self,
        position: (usize, usize),
        field_limits: (usize, usize),
    ) -> Option<(usize, usize)> {
        use Direction::*;
        let x = position.1;
        let y = position.0;
        let max_x = field_limits.1;
        let max_y = field_limits.0;
        match self {
            N => return if y > 0 { Some((y - 1, x)) } else { None },
            E => return if x <= max_x { Some((y, x + 1)) } else { None },
            S => return if y <= max_y { Some((y + 1, x)) } else { None },
            W => return if x > 0 { Some((y, x - 1)) } else { None },
        }
    }

    fn comp(&self) -> Direction {
        use Direction::*;
        match self {
            N => S,
            E => W,
            S => N,
            W => E,
        }
    }
}

impl Pipe {
    fn directions(&self) -> Option<Vec<Direction>> {
        use Direction::*;
        use Pipe::*;
        match self {
            None => Option::None,
            NS => Some(vec![N, S]),
            EW => Some(vec![E, W]),
            NE => Some(vec![N, E]),
            ES => Some(vec![E, S]),
            SW => Some(vec![S, W]),
            WN => Some(vec![W, N]),
            Start => Some(vec![N, E, S, W]),
        }
    }

    fn from_char(c: char) -> Self {
        match c {
            '.' => Pipe::None,
            'S' => Pipe::Start,
            '|' => Pipe::NS,
            '-' => Pipe::EW,
            'L' => Pipe::NE,
            'J' => Pipe::WN,
            '7' => Pipe::SW,
            'F' => Pipe::ES,
            other => panic!("Unaware of char {other}"),
        }
    }

    fn transverse(&self, entry: &Direction) -> Option<Direction> {
        match self.directions() {
            Some(directions) => {
                if directions.contains(&entry.comp()) {
                    Some(directions.iter().filter(|p| p != &&entry.comp()).last().unwrap().clone())
                } else {
                    None
                }
            }
            None => None,
        }
    }
}
