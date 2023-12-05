use super::{CommandImpl, DynError};
use clap::Parser;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::fs;
use std::path::PathBuf;

#[derive(Parser, Debug)]
pub struct Day5b2 {
    #[clap(long, short)]
    input: PathBuf,
}

impl CommandImpl for Day5b2 {
    fn main(&self) -> Result<(), DynError> {
        let mut result = 0;
        let input: String = fs::read_to_string(&self.input).unwrap();
        let mut split_input: VecDeque<&str> = input.split("\n\n").collect::<VecDeque<&str>>();
        let mut seed_line: VecDeque<usize> = split_input
            .pop_front()
            .unwrap()
            .split(":")
            .nth(1)
            .expect("Couldn't split seeds")
            .split_ascii_whitespace()
            .map(|x| x.parse().expect("Couldnt convert seed to usize"))
            .collect();

        let mut offset_maps: HashMap<ConvType, (ConvType, Vec<Offset>)> = HashMap::new();

        for line in split_input {
            //let map = line.pop();
            //println!("New input: {line}");
            //do something
            let (source_category, destination_category, offsets) =
                generate_boundary_and_offset_arrays(line).expect("Could not generate range");

            /*
            for map in ranges.iter() {
                map.clone().print_values();
            }
            */
            offset_maps.insert(source_category, (destination_category, offsets));
        }
        let mut lowest_location = 99999999999;

        let final_destination = ConvType::Location;

        for i in 0..seed_line.len() / 2 {
            let seed_start: usize = seed_line.pop_front().expect("Couldnt get seed start");
            let seed_range: usize = seed_line.pop_front().expect("Couldnt get seed range");
            println!("Calculating range{i} from {seed_start} to {}", seed_start + seed_range - 1);

            for seed_number in seed_start..seed_start + seed_range {
                //println!("Seed: {seed_number}");
                let mut source_category = ConvType::Seed;
                let mut source_number = seed_number;

                while source_category != final_destination {
                    let (destination_target, destination_offsets) =
                        offset_maps.get(&source_category).expect("Could not find conversion map");
                    let destination_number = convert_number(source_number, destination_offsets);
                    //println!("Seed {seed_number}: source: {source_number} destination: {destination_number}");
                    source_number = destination_number;
                    source_category = destination_target.clone();
                }
                if source_number < lowest_location {
                    println!(
                        "Seed {seed_number} has currently the lowest location {source_number}"
                    );
                    lowest_location = source_number;
                }
            }
        }

        //println!("Day5a: {result}");
        Ok(())
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
enum ConvType {
    Seed,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temperature,
    Humidity,
    Location,
}

impl ConvType {
    fn from_str(s: &str) -> Option<ConvType> {
        match s {
            "seed" => Some(ConvType::Seed),
            "soil" => Some(ConvType::Soil),
            "fertilizer" => Some(ConvType::Fertilizer),
            "water" => Some(ConvType::Water),
            "light" => Some(ConvType::Light),
            "temperature" => Some(ConvType::Temperature),
            "humidity" => Some(ConvType::Humidity),
            "location" => Some(ConvType::Location),
            _ => None,
        }
    }
    fn to_string(&self) -> &'static str {
        match self {
            ConvType::Seed => "Seed",
            ConvType::Soil => "Soil",
            ConvType::Fertilizer => "Fertilizer",
            ConvType::Water => "Water",
            ConvType::Light => "Light",
            ConvType::Temperature => "Temperature",
            ConvType::Humidity => "Humidity",
            ConvType::Location => "Location",
        }
    }
}

struct Offset {
    start: usize,
    end: usize,
    offset: i64,
}

fn convert_number(number: usize, offset_data: &Vec<Offset>) -> usize {
    for offset in offset_data {
        if offset.start <= number {
            if number <= offset.end {
                //println!("Calculating destination number from {}: {}", number, offset.offset);
                let offset_number = (number as i64)
                    .checked_add(offset.offset)
                    .expect("Overflow when adding offset to number!");
                return offset_number as usize;
            }
        }
    }
    return number;
}

// takes in multiple lines starting with the description and generates conversion maps
fn generate_boundary_and_offset_arrays(
    multiline: &str,
) -> Result<(ConvType, ConvType, Vec<Offset>), String> {
    let mut split_lines: VecDeque<&str> = multiline.lines().collect::<VecDeque<&str>>();
    let first_line = split_lines.pop_front().expect("Could not take first line");
    let mut info_list = first_line.split_ascii_whitespace().nth(0).unwrap().split("-");
    let source_category =
        ConvType::from_str(info_list.next().expect("Couldnt extract source category"))
            .expect("Unknown parameter");
    info_list.next();
    let destination_category =
        ConvType::from_str(info_list.next().expect("Couldnt extract destination category"))
            .expect("Unknown parameter");
    /*
    println!(
        "Source: {}, Destination: {}",
        source_category.to_string(),
        destination_category.to_string()
    );
    */
    //start range, end_range, offset
    let mut offset_data: Vec<Offset> = Vec::with_capacity(split_lines.len());

    for line in split_lines {
        if line.len() > 0 {
            //println!("line: {line}");
            let (source_start, source_end, offset) = extract_boundaries_and_offset(line);
            offset_data.push(Offset { start: source_start, end: source_end, offset: offset });
        }
    }
    Ok((source_category, destination_category, offset_data))
}

// source start, source end (inclusive), offset
fn extract_boundaries_and_offset(line: &str) -> (usize, usize, i64) {
    let mut values: VecDeque<usize> = line
        .split_ascii_whitespace()
        .map(|x| x.parse().expect("could not convert value to usize"))
        .collect();
    let destination_start = values.pop_front().expect("destination start not found");
    let source_start = values.pop_front().expect("source start not found");
    let range = values.pop_front().expect("range not found");
    let range_end =
        (source_start + range).checked_sub(1).expect("Underflow when calculating range end");
    let offset: i64 = (destination_start as i64)
        .checked_sub(source_start as i64)
        .expect("Underflow when calculating offset");
    /*   println!(
        "Destination start: {}, Source start: {}, range: {}, calculated range end: {}, calculated offset: {}",
        destination_start, source_start, range,range_end,  offset
    );
    */
    (source_start, range_end, offset)
}
