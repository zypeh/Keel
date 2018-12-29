extern crate keel;
extern crate clap;

use keel::keelc::parser;
use clap::{App, Arg};

fn main() {
    let matches = App::new("keelc")

    .arg(Arg::with_name("input")
        .help("the input file to compile")
        .required(true)
        .index(1)
    )

    .get_matches();

    if let Some(input_file) = matches.value_of("input") {
        // It's safe to call unwrap() because of the required options we set above
        println!("Compiling {}", input_file);
        parser::hello_scanner();
    }
}
