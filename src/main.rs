extern crate argparse;
extern crate regex;

use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use argparse::{ArgumentParser, StoreTrue, Store};
use regex::Regex;

fn main() {
    let mut verbose = false;
    let mut filename = "".to_string();
    let mut pattern = "".to_string();
    let mut case_insensitive = false;
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Greet somebody");
        ap.refer(&mut verbose)
            .add_option(&["-v", "--verbose"], StoreTrue,
                    "verbose output");
        ap.refer(&mut case_insensitive)
            .add_option(&["-i", "--case-insensitive"], StoreTrue,
                    "case insensitive search");
        ap.refer(&mut pattern)
            .add_argument("pattern", Store, "pattern")
            .required();
        ap.refer(&mut filename)
            .add_argument("file", Store, "file to search")
            .required();

        ap.parse_args_or_exit();
    }

    if verbose {
        println!("Searching file {} with pattern {}", &filename, &pattern);
    }

    // try! isn't working here, not sure why
    let file = match File::open(&filename) {
        Err(why) => panic!("couldn't open {} {}", &filename, &why),
        Ok(file) => file,
    };
    let file = BufReader::new(file);

    let insensive_pattern = if case_insensitive {
        "(?i)"
    } else {
        ""
    };

    let full_pattern = insensive_pattern.to_string() + &pattern;
    let regex = Regex::new(&full_pattern).unwrap();
    for line in file.lines() {
        let l = line.unwrap().to_string();
        if regex.is_match(&l) {
            println!("{}", &l);
        }
    }
}
