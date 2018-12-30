extern crate clap;
extern crate keel;

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use logos::Logos;
use clap::{App, Arg};
use keel::keelc::fast_lexer::Token;

fn main() {
    let matches = App::new("keelc")
        .arg(
            Arg::with_name("input")
                .help("the input file to compile")
                .required(true)
                .index(1),
        )
        .get_matches();

    // It's safe to call unwrap() because of the required options we set above
    let file_path = Path::new(matches.value_of("input").unwrap());
    let mut file = match File::open(&file_path) {
        Ok(file) => file,
        Err(reason) => panic!("could not open {} because {}", file_path.display(), reason)
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        // Ok(_) => println!("{:?}", lexer::Lexer::new(s.as_ref()).scan().unwrap()),
        Err(reason) => panic!(reason),
        Ok(_) => {
            let mut lex = Token::lexer::<&str>(s.as_ref());
            while lex.token != Token::EndOfFile {
                println!("{:?} - {:?}", lex.token, lex.slice());
                lex.advance()
            }
        }
    }
}
