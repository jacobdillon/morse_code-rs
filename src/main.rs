extern crate docopt;
extern crate rustc_serialize;
extern crate morse_code;

use docopt::Docopt;
use std::io::{self, BufRead};
use morse_code::translate_string;

const USAGE: &'static str = "
Usage:
    morse [--totext]
    morse (-h | --help)

Options:
    -h --help   Show this screen.
    --totext    Translate from morse code to English
";

fn main() {
    let args = Docopt::new(USAGE)
                   .and_then(|dopt| dopt.parse())
                   .unwrap_or_else(|e| e.exit());

    let mut input = String::new();
    let stdin = io::stdin();

    // Read line by line from stdin, assembling a String replacing newlines
    // with spaces and store in input variable
    for line in stdin.lock().lines() {
        input = format!("{}{} ", input, line.unwrap().trim());
    }

    // Print result
    println!("{}",
             translate_string(input.trim().to_string(), args.get_bool("--totext")).trim());
}
