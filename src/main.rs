use std::{fmt, fs::File, io::prelude::*};

use anyhow::Context;
use clap::{Arg, Command};

fn read_input_file(input_path: &str) -> Result<String, anyhow::Error> {
    let mut file =
        File::open(input_path).context(format!("Error opening input file '{}'", input_path))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .context(format!("Error reading file '{}'", input_path))?;
    Ok(contents)
}

fn parse_input(input: &str) -> Result<Vec<Elf>, anyhow::Error> {
    unimplemented!()
}

fn part_1(elves: Vec<Elf>) -> Result<u32, anyhow::Error> {
    unimplemented!()
}

fn part_2(elves: Vec<Elf>) -> Result<u32, anyhow::Error> {
    unimplemented!()
}

fn main() -> Result<(), anyhow::Error> {
    let matches = Command::new("AoC 2022: {{crate_name}}")
        .version("1.0")
        .author("Ricky Hosfelt <ricky@hosfe.lt>")
        .about("Solution to AoC {{crate_name}}")
        .subcommand_required(true)
        .subcommand(
            Command::new("part1").about("{{crate_name}} part 1").arg(
                Arg::new("input")
                    .short('i')
                    .long("input")
                    .help("/path/to/puzzle.input")
                    .required(true),
            ),
        )
        .subcommand(
            Command::new("part2").about("{{crate_name}} part 2").arg(
                Arg::new("input")
                    .short('i')
                    .long("input")
                    .help("/path/to/puzzle.input")
                    .required(true),
            ),
        )
        .get_matches();

    // day_1 part_1
    if let Some(ref commands) = matches.subcommand_matches("part1") {
        if commands.try_contains_id("input")? {
            let total_inputs = read_input_file(
                commands
                    .get_one::<String>("input")
                    .context("Error no value supplied for --input")?,
            )?;

            println!(
                "{{crate_name}} part 1: {:?}",
                part_1(parse_input(&total_inputs)?)?
            );
        }
    }
    // day_1 part_2
    /*if let Some(ref commands) = matches.subcommand_matches("part2") {
        if commands.try_contains_id("input")? {
            let total_inputs = read_input_file(
                commands
                    .get_one::<String>("input")
                    .context("Error no value supplied for --input")?,
            )?;

            println!("{{crate_name}} part 2: {:?}", part_2(parse_input(&total_inputs)?)?);
        }
    }*/

    Ok(())
}
