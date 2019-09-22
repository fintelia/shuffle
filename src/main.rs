use clap::{App, Arg};
use failure::Error;
use std::fs;

mod grammer;
mod parser;

fn main() -> Result<(), Error> {
    let matches = App::new("Shuffle")
        .version("0.1")
        .author("Jonathan Behrens <fintelia@gmail.com>>")
        .about("Does awesome things")
        .arg(
            Arg::with_name("output")
                .short("o")
                .long("output")
                .value_name("OUTPUT")
                .help("Name of '.S' file to generate")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("INPUT")
                .help("Sets the input file to use")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("v")
                .short("v")
                .multiple(true)
                .help("Sets the level of verbosity"),
        )
        .get_matches();

    let input_filename = matches.value_of("input").unwrap();
    let source = fs::read(input_filename).expect(&format!("Failed to read '{}'", input_filename));
    let utf8 = std::str::from_utf8(&source).expect("Unable to parse source as UTF-8");

    let parsed = parser::parse(&input_filename, &utf8).unwrap().1;

    Ok(())
}
