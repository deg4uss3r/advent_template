use std::{fs::File, io::prelude::*};

use anyhow::Context;
use clap::{App, Arg, SubCommand};

fn read_input_file(input_path: &str) -> Result<String, anyhow::Error> {
    let mut file =
        File::open(input_path).context(format!("Error opening input file '{}'", input_path))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .context(format!("Error reading file '{}'", input_path))?;
    Ok(contents)
}

fn parse_input(input: &str) -> Result<(), anyhow::Error> {
    unimplemented!()
}

fn part_1() -> Result<(), anyhow::Error> {
    unimplemented!()
}

//fn part_2() -> Result<(), anyhow::Error> {
//    unimplemented!()
//}

fn main() -> Result<(), anyhow::Error> {
    let matches = App::new("AoC 2021: {{crate_name}}")
        .version("1.0")
        .author("{{authors}}")
        .about("Solution to AoC {{crate_name}}")
        .subcommand(
            SubCommand::with_name("ex1")
                .about("{{crate_name}} part_1 example")
                .arg(
                    Arg::with_name("input")
                        .short("i")
                        .long("input")
                        .help("/path/to/puzzle.example")
                        .takes_value(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("part_1").about("{{crate_name}} part_1").arg(
                Arg::with_name("input")
                    .short("i")
                    .long("input")
                    .help("/path/to/puzzle.input")
                    .takes_value(true),
            ),
        )
        .subcommand(
            SubCommand::with_name("ex2")
                .about("{{crate_name}} part_2 example")
                .arg(
                    Arg::with_name("input")
                        .short("i")
                        .long("input")
                        .help("/path/to/puzzle.example")
                        .takes_value(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("part_2").about("{{crate_name}} part_2").arg(
                Arg::with_name("input")
                    .short("i")
                    .long("input")
                    .help("/path/to/puzzle.input")
                    .takes_value(true),
            ),
        )
        .get_matches();

    //{{crate_name}} part_1 example
    if let Some(ref matches) = matches.subcommand_matches("ex1") {
        if matches.is_present("input") {
            let total_inputs = read_input_file(
                matches
                    .value_of("input")
                    .context("Error no value supplied for --input")?,
            )?;

            println!("{{crate_name}} part 1 example: {}", part_1(parse_input(&total_inputs)?)?);
        }
    }

    // {{crate_name}} part_1
    if let Some(ref matches) = matches.subcommand_matches("part_1") {
        if matches.is_present("input") {
            let total_inputs = read_input_file(
                matches
                    .value_of("input")
                    .context("Error no value supplied for --input")?,
            )?;

            println!("{{crate_name}} part 1: {}", part_1(parse_input(&total_inputs)?)?);
        }
    }
/*
    // {{crate_name}} part_2 example
    if let Some(ref matches) = matches.subcommand_matches("ex2") {
        if matches.is_present("input") {
         let total_inputs = read_input_file(
                matches
                    .value_of("input")
                    .context("Error no value supplied for --input")?,
            )?;

            println!("{{crate_name}} part 2 example: {}", part_2(parse_input(&total_inputs)?)?);
        }
    }

    // {{crate_name}} part_2
    if let Some(ref matches) = matches.subcommand_matches("part_2") {
        if matches.is_present("input") {
            let total_inputs = read_input_file(
                matches
                    .value_of("input")
                    .context("Error no value supplied for --input")?,
            )?;

            println!("{{crate_name}} part 2: {}", part_2(parse_input(&total_inputs)?)?);
        }
    }
*/
    Ok(())
}
