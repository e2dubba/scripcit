use std::collections::{HashMap, HashSet};
use std::cmp::Ordering;
use regex::Regex;
use std::io::{BufReader, BufRead};
use std::error::Error;
use std::fs::File;

#[path = "./roman_numerals.rs"] mod roman_numerals;

pub struct Book {
    pub num: Option<i16>,
    pub sort_on: char,
    pub name: String,
    pub canonical_name: String,
    pub idx: HashMap<char, Vec<usize>>,
}

pub struct Library {
    pub items: HashMap<char, Vec<Book>>,
}


impl Library {
    pub fn create() -> Result<Library, Box<dyn Error>> {
        let mut library_collection = Library::new();
        let file= File::open("data/books.csv")?;
        let reader = BufReader::new(file);
        for line in reader.lines() {
            let books = line.unwrap();
            let mut book_iter = books.split(",");
            let canonical_name = book_iter.next().unwrap();
            let first_book = Book::new(canonical_name, canonical_name);
            library_collection.add(first_book);
            for book in book_iter {
                if book != "" {
                    let next_book = Book::new(book, canonical_name);
                    library_collection.add(next_book);
                }
            }
        }
        Ok(library_collection)
    }

    pub fn new() -> Library {
        Library { items: HashMap::new() }
    }

    pub fn add(&mut self, book: Book) {
        if !&self.items.contains_key(&book.sort_on) {
            self.items.insert(book.sort_on, Vec::new());
        }
        if let Some(book_vec) = self.items.get_mut(&book.sort_on) {
            book_vec.push(book);
        }
    }

    pub fn match_book(&self, book_to_match: &str) -> HashSet<String> {
        let (num, other_book) = book_split(book_to_match);
        let mut possible_matches = HashSet::new();
        let first_char = other_book.chars().next().unwrap();
        if !&self.items.contains_key(&first_char) {
            return possible_matches
        }

        for book in self.items.get(&first_char).unwrap() {
            if book.name_match(&other_book, &num) {
                possible_matches.insert(book.canonical_name.clone());
            }
        }
        possible_matches
    }
}

impl Book {
    pub fn new(name_str: &str, canonical_name: &str) -> Book {
        // let mut idx: HashMap<char, Vec<usize>> = HashMap::new();
        let (num, name) = book_split(name_str);
        let mut idx = HashMap::new();
        for c in name.chars() {
            idx.insert(c, Vec::new());
        }
        let sort_on = name.chars().next().unwrap();
        for (index, c) in name.chars().enumerate() {
            if let Some(x) = idx.get_mut(&c) {
                x.push(index);
            }
        }
            
        Book {
            num: num, 
            sort_on: sort_on,
            name: String::from(name), 
            canonical_name: String::from(canonical_name),
            idx: idx
        }
    }

    fn ordered_name_match(&self, other_name: &str) -> bool {
        let mut book_iter = self.name.chars();
        for c in other_name.chars() {
            let book_c = book_iter.next();
            let value = book_c.as_ref().map(|n|  &c == n );
            if value.is_none() {
                return false
            }
            if value.unwrap() == false {
                return false
            }
        }
        true
    }

    pub fn name_match(&self, other_name: &str, other_num: &Option<i16>) -> bool {
        if other_num != &self.num {
            return false
        }
        if self.ordered_name_match(other_name) { 
            return true
        }
        let mut last = 0;
        for c in other_name.chars() {
            if !&self.idx.contains_key(&c) {
                return false
            }
            let char_vec = &self.idx.get(&c);
            for ivec in char_vec.iter() {
                for i in ivec.iter() {
                    match last.cmp(i) {
                        Ordering::Less => last = *i,
                        Ordering::Greater => return false,
                        Ordering::Equal => last = *i,
                    }
                }
            }
        }
        true
    }
}

pub fn book_split(book_name: &str) -> (Option<i16>, String) {
    lazy_static! {
        static ref BOOK_RE: Regex = Regex::new(r"^((\d+ ?)|([ivIV]+ ))").unwrap();
    }
    let mut name = String::new();
    let mat_opt = BOOK_RE.find(book_name);
    if mat_opt.is_none() {
        let name = book_name.trim().to_lowercase();
        return (None, String::from(name))
    }

    let mat = mat_opt.unwrap();
    let value = mat.as_str().trim();
    let num = value.parse::<i16>();
    let inum: Option<i16>  = match num.is_err() {
        false => Some(num.unwrap()),
        true => Some(roman_numerals::convert_to_numbers(value)),
    };
    for (i, chr) in book_name.chars().enumerate() {
        if i >= mat.end() {
            name.push(chr);
        }
    }
    name = String::from(name.to_lowercase().trim());
    (inum, name) 
}


#[cfg(test)]
mod tests {
    use super::*;
    // use crate::citation::roman_numerals::*;

    #[test]
    fn consec_abbrev_match_test() {
        let nbook = Book::new("Genesis", "Genesis"); // {num: None, name: String::from("Genesis"), idx: None};
        let (num, abbrev) = book_split("Gen");
        assert_eq!(nbook.name_match(&abbrev, &num), true);
    }

    #[test]
    fn irregular_abbrev_match_test() {
        let nbook = Book::new("John", "John");
        let (num, abbrev) = book_split("Jn");
        assert!(nbook.name_match(&abbrev, &num));
        let (nnum, nabbrev) = book_split("Joh");
        assert!(nbook.name_match(&nabbrev, &nnum));
        let (wnum, wabbrev) = book_split("Jdg");
        assert_eq!(nbook.name_match(&wabbrev, &wnum), false);
    }

    #[test]
    fn match_book_and_title() {
        let (num, abbrev) = book_split("2 Kgs");
        let book = Book::new("2 Kings", "2 Kings");
        assert!(book.name_match(&abbrev, &num));

    }

    #[test]
    fn book_split_test() {
        let book = "ii Samuel";
        let (num, name) = book_split(book);
        assert_eq!(num, Some(2));
        assert_eq!(name, "samuel");
    }

    #[test]
    fn archive_search_test() {
        let library = Library::create().unwrap();
        let abbrev = "ii Sam";
        let mut expected_return: std::collections::HashSet<String> = std::collections::HashSet::new();
        expected_return.insert(String::from("2 Samuel"));
        let actual_return = library.match_book(abbrev);
        let (num, nab) = book_split(abbrev);
        println!("{}, {}", num.unwrap(), nab);
        println!("{:?}", actual_return);
        assert_eq!(actual_return, expected_return);
    }

    #[test]
    fn kings_archive_search_test() {
       let library = Library::create().unwrap();
        let abbrev = "2Kng";
        let mut expected_return: std::collections::HashSet<String> = std::collections::HashSet::new();
        expected_return.insert(String::from("2 Kings"));
        let actual_return = library.match_book(abbrev);
        let (num, nab) = book_split(abbrev);
        println!("{}, {}", num.unwrap(), nab);
        println!("{:?}", actual_return);
        assert_eq!(actual_return, expected_return);
    }

    #[test]
    fn isa_search() {
        let library = Library::create().unwrap();
        let abbrev = "Isa";
        let mut expected_return: std::collections::HashSet<String> = std::collections::HashSet::new();
        expected_return.insert(String::from("Isaiah"));

        let actual_return = library.match_book(abbrev);
        assert_eq!(actual_return, expected_return);
    }

}
