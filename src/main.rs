extern crate clap;
use clap::{Arg, App, ArgMatches};
use std::fs::File;
use std::path::Path;
use std::io::{BufReader, BufRead, Write, Read};
use csv::ReaderBuilder;

fn main() {
    let matches = get_cli_args();

    let mut output_file= File::create(Path::new(&matches.value_of("OUTPUT").unwrap()))
        .expect("Couldn't open OUTPUT file");
    let skip_header = matches.is_present("skip-header");

    let mut csv = ReaderBuilder::new()
        .flexible(true)
        .has_headers(skip_header)
        .from_path(Path::new(&matches.value_of("INPUT").unwrap())).unwrap();

    for line_result in csv.records() {
        let trimmed_line = line_result.unwrap().iter()
            .filter(|part| !part.is_empty())
            .map(|part| part.replace("\r", "").replace("\n", "\\n").to_string())
            .collect::<Vec<String>>()
            .join(";");

        output_file.write(trimmed_line.as_bytes()).expect("Couldn't write to OUTPUT file");

        if trimmed_line.len() > 0 {
            output_file.write(b"\n");
        }

        print!(".");
    }
}

fn get_cli_args() -> ArgMatches<'static> {
    App::new("CSV to LNG converter for This Land Is My Land")
        .version("0.1")
        .arg(Arg::with_name("INPUT")
            .help("Sets the input .csv file to use")
            .required(true)
            .index(1))
        .arg(Arg::with_name("OUTPUT")
            .help("Sets the output .lng file to use")
            .required(false)
            .default_value("output.lng")
            .index(2))
        .arg(Arg::with_name("skip-header")
            .help("Sets if csv file has header needed to skip")
            .short("s"))
        .get_matches()
}
