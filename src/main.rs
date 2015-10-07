extern crate argparse;
extern crate regex;

use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use argparse::{ArgumentParser, StoreTrue, Store};
use regex::Regex;

mod matcher;

struct Options {
    verbose: bool,
    case_insensitive: bool,
    pattern: String,
    filename: String
}

fn parse_arguments() -> Options {
    let mut options = Options {
        verbose: false,
        case_insensitive: false,
        pattern: "".to_string(),
        filename: "".to_string(),
    };

    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Search through a file, like grep");
        ap.refer(&mut options.verbose)
            .add_option(&["-v", "--verbose"], StoreTrue,
                    "verbose output");
        ap.refer(&mut options.case_insensitive)
            .add_option(&["-i", "--case-insensitive"], StoreTrue,
                    "case insensitive search");
        ap.refer(&mut options.pattern)
            .add_argument("pattern", Store, "pattern")
            .required();
        ap.refer(&mut options.filename)
            .add_argument("file", Store, "file to search")
            .required();

        ap.parse_args_or_exit();
    }

    options
}

// turn this essentially into a grok-lite project so I could give a pattern something like
// %{DATETIME: date} %{WORD:someword} - %{WORD:anotherword}
// and we turn that into a regex basically like this:
// (?P<date>\d{4}-\d{2}-\d{2}) (?P<someword>\w+) - (?P<anotherword>w*)
// The above example would be using a PATTERNS FILE somewhere to do that
// OR
// You could just pass in straight regex.
// Then we turn this into a structure somehow... not sure how yet
// Then we can spit the data out in a JSON format
// something like:
// {
//   "date": "2015-04-01",
//   "someword": "match",
//   "anotherword": "another match"
// }
//
// Basically a more dynamic version of the log scraper I wrote earlier

fn main() {
    let options = parse_arguments();
    if options.verbose {
        println!("Searching file {} with pattern {}", &options.filename, &options.pattern);
    }

    // try! isn't working here, not sure why
    let file = match File::open(&options.filename) {
        Err(why) => panic!("couldn't open {} {}", &options.filename, &why),
        Ok(file) => file,
    };
    let file = BufReader::new(file);

    let insensive_pattern = if options.case_insensitive {
        "(?i)"
    } else {
        ""
    };

    let full_pattern = insensive_pattern.to_string() + &options.pattern;
    let regex = Regex::new(&full_pattern).unwrap();
    for line in file.lines() {
        let l = line.unwrap().to_string();
        if regex.is_match(&l) {
            println!("{}", &l);
        }
    }
}
