//! scripcit 
//! 
//! a small command line utility for extracting scripture citations from free text

#[macro_use] extern crate lazy_static;

use regex::Regex;
use std::fs;
use std::process;
use std::error::Error;
use structopt::StructOpt;

mod scriptureregex;
#[path = "citation/roman_numerals/lib.rs"] mod roman_numerals;
#[path = "citation/address.rs"] mod address;

// Extract all of the Scripture Citations out of A text
#[derive(StructOpt)]
#[structopt(about="A small command line utility for extracting scripture citations from a text")]
struct Cli {
    /// Run a match on a specific citation 
    #[structopt(short, long)]
    citation: Option<String>,
    /// The file to search in
    filename: Option<String>,
}
fn main() {
    // This function runs the command line arguments. Whether just 
    // testing one citation, or running through a whole text file.
    let args = Cli::from_args();

    if !args.citation.is_none() {
        let library = address::book_linking::Library::create().unwrap();
        let mut scriptures = address::CitationList::new();
        let citation = args.citation.unwrap();
        println!("{}", citation);

        scriptures.insert(&citation, &library);
        for reference in scriptures.scrip_vec {
            println!("\t{}", reference);
        }
        process::exit(1)
    }


    let new_regex = scriptureregex::regex_creator();
    println!("{}", new_regex);
    if let Err(e) = run(args) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}

fn run(args: Cli) -> Result<(), Box<dyn Error>> {
    // Opens a given file, and iterates through every possible scripture match in the file
    // to see which matched regex patterns can be normalized into a scripture citation.
    let filename = args.filename.unwrap();
    let contents = fs::read_to_string(filename)?;
    let matches = find_scipture_in_text(&contents);
    let library = address::book_linking::Library::create().unwrap();

    for mat in matches {
        let form_mat = mat.replace("\n", " ");
        println!("{}", form_mat);
        let mut scriptures = address::CitationList::new();
        scriptures.insert(mat, &library);

        for reference in scriptures.scrip_vec {
            println!("\t{}", reference);
        }
    }

    Ok(())
}

fn find_scipture_in_text(text: &str) -> Vec<&str> {
    // This function calles the scripture regex and matches on a possible text
    let regex_string = scriptureregex::regex_creator();
    let scripture_regex = Regex::new(&regex_string).unwrap();
    scripture_regex.find_iter(text).map(|mat| mat.as_str()).collect()
}
