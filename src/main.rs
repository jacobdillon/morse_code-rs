#![feature(plugin)]
#![plugin(docopt_macros)]

extern crate docopt;
extern crate rustc_serialize;
extern crate morse_code;

use std::io::{self, BufRead};
use morse_code::translate_string;

docopt!(Args, "Usage: morse <input>", arg_input: String);

fn main() {
    // Get arguments
    let args: Args = Args::docopt().decode().unwrap_or_else(|e| e.exit());

    let mut input = String::new();
    let stdin = io::stdin();

    if args.arg_input == "-" {
        // Read line by line from stdin, assembling a String replacing newlines
        // with spaces and store in input variable
        for line in stdin.lock().lines() {
            input = format!("{}{} ", input, line.unwrap());
        }
    } else {
        // Just read the supplied argument into input
        input = args.arg_input.clone();
    }

    // Print result
    println!("{}", translate_string(input.trim().to_string()).trim());
}
