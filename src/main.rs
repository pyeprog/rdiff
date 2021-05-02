mod core;
mod display;
mod util;

use util::file::readlines;
use crate::core::comparing_line::ComparingLine;
use crate::display::print_comparing_lines;
use clap::{App, Arg, ArgMatches};

fn main() {
    let matches = get_args();

    let file1 = matches.value_of("File1").unwrap_or("");
    let file2 = matches.value_of("File2").unwrap_or("");
    let width = matches.value_of("width").unwrap_or("50").parse::<usize>().unwrap_or(50);

    let lines1: Vec<String> = readlines(file1);
    let lines2: Vec<String> = readlines(file2);

    let comparing_lines = ComparingLine::from(lines1, lines2, Option::Some(width), Option::Some('|'));

    print_comparing_lines(&comparing_lines);
}

fn get_args() -> ArgMatches {
    App::new("rdiff")
        .version("v1.0")
        .author("written by: Pye<pyeprog@foxmail.com>")
        .about("description: a simple diff in rust")
        .arg(Arg::with_name("File1")
            .help("Sets the input file1 to use")
            .required(true)
            .index(1))
        .arg(Arg::with_name("File2")
            .help("Sets the input file2 to use")
            .required(true)
            .index(2))
        .arg(Arg::with_name("width")
            .short("w")
            .long("width")
            .value_name("width")
            .help("set the virtual win width")
            .default_value("50")
            .takes_value(true))
        .get_matches()
}
