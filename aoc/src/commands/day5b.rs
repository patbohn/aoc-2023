use super::{CommandImpl, DynError};
use clap::Parser;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::fs;
use std::path::PathBuf;

#[derive(Parser, Debug)]
pub struct Day5b {
    #[clap(long, short)]
    input: PathBuf,
}

impl CommandImpl for Day5b {
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

        let mut conversion_maps: HashMap<String, Vec<ConversionMap>> = HashMap::new();

        for line in split_input {
            //let map = line.pop();
            //println!("New input: {line}");
            //do something
            let (source_category, ranges) = generate_maps(line).expect("Could not generate range");

            /*
            for map in ranges.iter() {
                map.clone().print_values();
            }
            */
            conversion_maps.insert(source_category, ranges);
        }
        let mut lowest_location = 99999999999;
        for i in 0..seed_line.len() / 2 {
            let seed_start: usize = seed_line.pop_front().expect("Couldnt get seed start");
            let seed_range: usize = seed_line.pop_front().expect("Couldnt get seed range");
            println!("Calculating range{i} from {seed_start} to {}", seed_start + seed_range);
            for seed_number in seed_start..seed_start + seed_range {
                //println!("Seed: {seed_number}");
                let mut source_category: String = "seed".to_owned();
                let mut source_number = seed_number;
                let final_destination = "location";
                while source_category != final_destination {
                    let (destination_target, destination_number) = convert_source_to_dest(
                        &source_category,
                        source_number,
                        conversion_maps
                            .get(&source_category)
                            .expect("Could not find conversion map"),
                    );
                    //println!("Seed {seed_number}: source: {source_category} {source_number} destination: {destination_target} {destination_number}");
                    source_number = destination_number;
                    source_category = destination_target;
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

#[derive(Debug, Clone)]
struct ConversionMap {
    source_category: String,
    destination_category: String,
    source_range_start: usize,
    destination_range_start: usize,
    range_length: usize,
}

impl ConversionMap {
    fn convert_number(&self, source_number: usize) -> Option<usize> {
        if (source_number >= self.source_range_start)
            & (source_number < self.source_range_start + self.range_length)
        {
            return Some(self.destination_range_start + (source_number - self.source_range_start));
        }
        None
    }
    fn check_source_category(&self, comparison_category: &str) -> bool {
        self.source_category == comparison_category
    }

    fn print_values(&self) {
        println!("source: {}, destination: {}", self.source_category, self.destination_category);
        println!(
            "source_range_start: {}, destination_range_start: {}, range_length: {}",
            self.source_range_start, self.destination_range_start, self.range_length
        );
    }
    fn get_destination(&self) -> &str {
        &self.destination_category
    }
}

// takes in multiple lines starting with the description and generates conversion maps
fn generate_maps(multiline: &str) -> Result<(String, Vec<ConversionMap>), String> {
    let mut split_lines: VecDeque<&str> = multiline.lines().collect::<VecDeque<&str>>();
    let first_line = split_lines.pop_front().expect("Coould not take first line");
    let mut info_list = first_line.split_ascii_whitespace().nth(0).unwrap().split("-");
    let source_category: String =
        info_list.next().expect("Couldnt extract source category").to_owned();
    info_list.next();
    let destination_category = info_list.next().expect("Couldnt extract destination category");
    let mut ranges: Vec<ConversionMap> = Vec::new();
    for line in split_lines {
        if line.len() > 0 {
            //println!("line: {line}");
            ranges.push(
                generate_map(&source_category, destination_category, line)
                    .expect("Could not convert line to ConversionMap"),
            )
        }
    }
    Ok((source_category, ranges))
}

fn generate_map(
    source_category: &str,
    destination_category: &str,
    line: &str,
) -> Result<ConversionMap, String> {
    let mut values: VecDeque<usize> = line
        .split_ascii_whitespace()
        .map(|x| x.parse().expect("could not convert value to usize"))
        .collect();

    Ok(ConversionMap {
        source_category: source_category.to_owned(),
        destination_category: destination_category.to_owned(),
        destination_range_start: values.pop_front().expect("Could not get destination_range_start"),
        source_range_start: values.pop_front().expect("Could not get source_range_start"),
        range_length: values.pop_front().expect("Could not get range_length"),
    })
}

fn convert_source_to_dest(
    source_category: &str,
    number: usize,
    ranges: &Vec<ConversionMap>,
) -> (String, usize) {
    let mut destination_target: String = "".to_owned();
    for conversion in ranges {
        destination_target = conversion.get_destination().to_owned();
        match conversion.convert_number(number) {
            Some(converted_number) => {
                //println!("Found conversion: {converted_number}");
                //conversion.print_values();
                return (destination_target, converted_number);
            }
            None => {}
        }
    }
    return (destination_target, number);
}
