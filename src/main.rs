#[macro_use] extern crate lazy_static;

use regex::Regex;
use std::collections::HashSet;
use std::fs;
use std::process;
use std::error::Error;
use structopt::StructOpt;

mod scriptureregex;
#[path = "citation/roman_numerals.rs"] mod roman_numerals;
#[path = "citation/book_linking.rs"] mod book_linking;
#[path = "citation/address.rs"] mod address;

// Extract all of the Scripture Citations out of A text
#[derive(StructOpt)]
struct Cli {
    // The file to search in
    filename: String,
}
fn main() {
    let args = Cli::from_args();
    let new_regex = scriptureregex::regex_creator();
    println!("{}", new_regex);

    if let Err(e) = run(args) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}

fn run(args: Cli) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(args.filename)?;
    let matches = find_scipture_in_text(&contents);

    for mat in matches {
        let form_mat = mat.replace("\n", " ");
        println!("{}", form_mat);
        if roman_numerals::is_roman_numeral(mat) {
            println!("\tMatches!");
        }
    }

    Ok(())
}

fn find_scipture_in_text(text: &str) -> HashSet<&str> {
    let regex_string = scriptureregex::regex_creator();
    let scripture_regex = Regex::new(&regex_string).unwrap();
    scripture_regex.find_iter(text).map(|mat| mat.as_str()).collect()
}
