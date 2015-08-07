extern crate argparse;

use argparse::{ArgumentParser, StoreTrue, Store};

fn main() {
    let mut verbose = false;
    let mut filename = "".to_string();
    let mut pattern = "".to_string();
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Greet somebody");
        ap.refer(&mut verbose)
            .add_option(&["-v", "--verbose"], StoreTrue,
                    "verbose output");
        ap.refer(&mut pattern)
            .add_argument("pattern", Store, "pattern")
            .required();
        ap.refer(&mut filename)
            .add_argument("file", Store, "file to search")
            .required();

        ap.parse_args_or_exit();
    }

    if verbose {
        println!("Searching file {} with pattern {}", filename, pattern);
    }

    println!("{}", filename);
}
