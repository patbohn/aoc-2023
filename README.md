Note: this is a fork from sstadick as the modular approach of adding solutions is very nice for reducing clutter! 

# AOC 2023

This repo contains solutions for the [2023 Advent of Code](https://adventofcode.com/) in Rust.

## Running solutions

```bash
cargo run --bin aoc -- day0 --input test.txt
```

## Adding a new day

`aoc/src/commands/day0.rs` is a template for all coming days create quick and easy subcommands for running solutions.

Copy the template to the new day and update mod.rs and main.rs. 
