use clap::Parser;
use core::fmt;
use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;

use super::{CommandImpl, DynError};

#[derive(Parser, Debug)]
pub struct Day10b {
    #[clap(long, short)]
    input: PathBuf,
}

//idea to solve B:
// 1. Blow up the field 2x (adding to bottom and right), and add a first row top and left
//    this should ensure that all fields that are outside are connected
// 2. Find a field that is definitely outside (e.g. position < min_position pipes)
// 3. Walk to all fields possible
//    -> mark all positions that touch the first, then for each new marked to the same
// 4. To count only fields present at the start, look at only every 2nd (+1)
/*

-> change all tiles not part of the loop to .

.F7..
.||..
.|L-7
.L--J

-> blow up in x direction

..F-7.....
..|.|.....
..|.L---7.
..L-----J.

-> blow up in y direction

..F-7.....
..|.|.....
..|.|.....
..|.|.....
..|.L---7.
..|.....|.
..L-----J.
..........

-> add top and left border
...........
...F-7.....
...|.|.....
...|.|.....
...|.|.....
...|.L---7.
...|.....|.
...L-----J.
...........

-> mark all outside

O..........
...F-7.....
...|.|.....
...|.|.....
...|.|.....
...|.L---7.
...|.....|.
...L-----J.
...........

-> count all non-marked ().) that are at (x%2 == 1 & y%2 == 1)
*/

impl CommandImpl for Day10b {
    fn main(&self) -> Result<(), DynError> {
        let mut result = 0;
        let field = Field::from_input(fs::read_to_string(&self.input).unwrap());
        //dbg!(&field);
        let circle = &field.find_circle();
        let mut correct_path = Vec::new();

        for (direction, path) in circle {
            println!("When walking from {direction}");
            match path {
                Some(p) => {
                    correct_path = p.clone();
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
        println!("{}", &field);
        let exploded_field = field.expand(&correct_path);
        //dbg!(&exploded_field);
        //dbg!(&circle);
        println!("{}", &exploded_field);
        let exploded_field = mark_outside_pipes(exploded_field).unwrap();
        println!("Marked field:");
        println!("{}", &exploded_field);
        let count = count_pre_expansion_dots(&exploded_field);
        println!("Day10a: {count}");
        Ok(())
    }
}

fn mark_outside_pipes(mut field: Field) -> Result<Field, String> {
    // start at 0,0 (we know this one cannot be inside the pipes)
    let mut current_position = (0, 0);
    let mut unchecked_marked_positions: Vec<(usize, usize)> = vec![current_position];
    let field_limits = field.limits();
    use Direction::*;
    while unchecked_marked_positions.len() > 0 {
        current_position = unchecked_marked_positions.pop().unwrap();
        for direction in vec![N, E, S, W] {
            match direction.walk(&current_position, &field_limits) {
                Some(test_position) => {
                    //check whether it is free
                    //dbg!(&test_position);
                    match &field.get(&test_position) {
                        &Pipe::None => {
                            field.set(&test_position, &Pipe::Outside);
                            unchecked_marked_positions.push(test_position);
                        }
                        _ => {}
                    }
                }
                None => {} //cannot walk in this direction
            }
        }
    }

    Ok(field)
}

fn count_pre_expansion_dots(field: &Field) -> usize {
    let mut count = 0;
    let field_limits = field.limits();

    for y in 0..(field_limits.0 / 2) {
        for x in 0..(field_limits.1 / 2) {
            match field.get(&(x * 2 + 1, y * 2 + 1)) {
                Pipe::None => count += 1,
                _ => {}
            }
        }
    }
    count
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
        position: &(usize, usize),
        field_limits: &(usize, usize),
    ) -> Option<(usize, usize)> {
        use Direction::*;
        let x = position.1;
        let y = position.0;
        let max_x = field_limits.1 - 1;
        let max_y = field_limits.0 - 1;
        match self {
            N => return if y > 0 { Some((y - 1, x)) } else { None },
            E => return if x < max_x { Some((y, x + 1)) } else { None },
            S => return if y < max_y { Some((y + 1, x)) } else { None },
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
    Outside,
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
            Outside => Option::None,
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
            'O' => Pipe::Outside,
            other => panic!("Unaware of char {other}"),
        }
    }

    fn as_char(&self) -> char {
        use Pipe::*;
        match self {
            None => '.',
            Start => 'S',
            NS => '|',
            EW => '-',
            NE => 'L',
            WN => 'J',
            SW => '7',
            ES => 'F',
            Outside => 'O',
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

#[derive(Debug)]
struct Field {
    pipes: Vec<Vec<Pipe>>, //note that this is y,x
    start: Option<(usize, usize)>,
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut string = "".to_string();
        for row in &self.pipes {
            let row_str: String = row.iter().map(|pipe| pipe.as_char().to_string()).collect();
            string.push_str(&row_str);
            string.push_str("\n");
        }
        write!(f, "{}", string)
    }
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
                match entry_direction.walk(&current_position, &self.limits()) {
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

    fn get(&self, position: &(usize, usize)) -> &Pipe {
        self.pipes.get(position.0).unwrap().get(position.1).unwrap()
    }

    fn set(&mut self, position: &(usize, usize), pipe: &Pipe) {
        *self.pipes.get_mut(position.0).unwrap().get_mut(position.1).unwrap() = pipe.clone();
    }

    fn expand(&self, path: &Vec<(usize, usize)>) -> Self {
        //first step: set all fields that aren't part of the path to None
        let (y_length, x_length) = self.limits();
        let mut field = vec![vec![Pipe::None; x_length * 2 + 1]; y_length * 2 + 1];
        let start = path.first().unwrap().clone();
        let mut this_path = path.clone();
        this_path.push(start);
        //second step: add in circular path
        for steps in this_path.array_windows::<2>() {
            let current_step = steps.first().unwrap();
            let next_step = steps.last().unwrap();
            let new_x: usize = current_step.1 * 2 + 1;
            let new_y: usize = current_step.0 * 2 + 1;
            //add existing connection
            *field.get_mut(new_y).unwrap().get_mut(new_x).unwrap() = self.get(current_step).clone();
            let delta_x: isize = next_step.1 as isize - current_step.1 as isize;
            let delta_y: isize = next_step.0 as isize - current_step.0 as isize;
            let mut intermediate_x: usize = new_x;
            let mut intermediate_y: usize = new_y;
            let mut intermediate_pipe = Pipe::None;
            match delta_x {
                -1 | 1 => {
                    intermediate_pipe = Pipe::EW;
                    intermediate_x = intermediate_x
                        .checked_add_signed(delta_x)
                        .expect("Calculating intermediate position x value failed.");
                }
                0 => {}
                _ => panic!("Delta x should never be something other than -1, 0 or 1"),
            }
            match delta_y {
                -1 | 1 => {
                    intermediate_pipe = Pipe::NS;
                    intermediate_y = intermediate_y
                        .checked_add_signed(delta_y)
                        .expect("Calculating intermediate position y value failed.");
                }
                0 => {}
                _ => panic!("Delta x should never be something other than -1, 0 or 1"),
            }
            //add intermediate connection
            *field.get_mut(intermediate_y).unwrap().get_mut(intermediate_x).unwrap() =
                intermediate_pipe;
        }
        Field { pipes: field, start: Some(start) }
    }
}
