
use std::collections::HashSet;
use regex::Regex;

pub mod book_linking;
mod roman_numerals;

#[derive(Debug)]
#[derive(Clone)]
pub struct ScriptureCitation {
    book: String, 
    start_chap: Option<i16>,
    start_verse: Option<i16>,
    end_chap: Option<i16>,
    end_verse: Option<i16>,
}

pub struct CitationList {
    book: Option<String>,
    ranges: HashSet<String>,
    dividers: HashSet<String>,
    additions: HashSet<String>,
    curr_citation: Option<ScriptureCitation>,
    pub scrip_vec: Vec<ScriptureCitation>,

}

enum CitationParts {
    StartChap,
    StartVerse,
    Verse,
    EndChap,
    EndVerse,

}

impl std::fmt::Display for ScriptureCitation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut printstring = self.book.clone();
        if !self.start_chap.is_none() {
            printstring = format!("{} {}", printstring, self.start_chap.unwrap());
        }
        if !self.start_verse.is_none() {
            printstring = format!("{}: {}", printstring, self.start_verse.unwrap());
        }
        if !self.end_chap.is_none() {
            printstring = format!("{} - {}", printstring, self.end_chap.unwrap());
        }
        if !self.end_verse.is_none() {
            printstring = format!("{}: {}", printstring, self.end_verse.unwrap());
        }
        write!(f, "{}", printstring)
    }

}

impl ScriptureCitation {
    
    pub fn new(name: &String, start_chap: Option<i16>) -> ScriptureCitation {
        ScriptureCitation { 
            book: name.to_owned(), 
            start_chap: start_chap,
            start_verse: None,
            end_chap: None,
            end_verse: None,
        }
    }

}

impl CitationList {

//    pub fn add(&mut self, scrip_cit: ScriptureCitation) {
//        self.scrip_vec.push(scrip_cit);
//    }
//
//    fn update_divider(&mut self, divider: &String) {
//        let mut divid: HashSet<_> = HashSet::new();
//        divid.insert(divider.to_owned());
//        self.additions.remove(divider);
//        self.dividers = divid;
//    }
//
    fn chapter_previous(&mut self, element: &String) -> Address  {
        if self.ranges.contains(element) {
            let prev_element = Address::ChapterRange;
            return prev_element
        }
        if self.dividers.len() == 3 && !self.ranges.contains(element){
            let mut temp_set: HashSet<String> = HashSet::new();
            temp_set.insert(element.to_owned());
            self.dividers = temp_set;
            let prev_element = Address::Divider;
            return prev_element
        }
        if self.dividers.len() == 1 && !self.dividers.contains(element) {
            let prev_element = Address::Addition;
            return prev_element
        }
        if self.dividers.contains(element) {
            let prev_element = Address::Divider;
            return prev_element
        }
        Address::Error
    }


    fn update_curr_citation(&mut self, citation_part: CitationParts, element: &String) {
        let mut citation = match self.curr_citation.clone() {
            Some(ScriptureCitation) => {
                self.curr_citation.clone().unwrap()
            }
            None => {
                let book = self.book.clone().unwrap();
                ScriptureCitation::new(&book, None)
            },
        };

        let num = convert_str_to_address_num(element);

        match citation_part {
            CitationParts::StartChap => { citation.start_chap = num; }, 
            CitationParts::StartVerse => {citation.start_verse = num; }, 
            CitationParts::Verse => {
                if citation.start_verse.is_none() { 
                    citation.start_verse = num 
                } else { 
                    citation.end_chap = citation.start_chap.clone();
                    citation.end_verse = num;
                };},
            CitationParts::EndChap => { citation.end_chap = num; },
            CitationParts::EndVerse => { 
                if citation.end_chap.is_none() {
                    citation.end_chap = citation.start_chap.clone();
                }
                citation.end_verse = num; 
            },
        }

        self.curr_citation = Some(citation);
    }

    fn handeling_ranges(&mut self, next_element: Option<&String>, curr_element: &String) -> Address {
        let curr_citation = &self.curr_citation.clone().map(|x| x).unwrap();
        let end = String::from("End");
        let next_element: &String = next_element.unwrap_or(&end);
        if self.dividers.contains(&next_element.clone()) {
            self.update_curr_citation(CitationParts::EndChap, curr_element);
            return Address::Chapter
        }
        if curr_citation.start_verse.is_none() {
            self.update_curr_citation(CitationParts::EndChap, curr_element);
            return Address::Chapter
        }
        self.update_curr_citation(CitationParts::Verse, curr_element);
        Address::Verse
    }

    fn handeling_additions(&mut self, next_element: Option<&String>, curr_element: &String) -> Address {
        let prev_citation = &self.curr_citation.clone().map(|x| x).unwrap();
        self.scrip_vec.push(prev_citation.clone());
        let book = self.book.clone().map(|x| x).unwrap();
        let mut add_citation = ScriptureCitation::new(&book, None);

        let num = convert_str_to_address_num(curr_element);
        let end = String::from("End");

        let next_element: &String = next_element.unwrap_or(&end);
        if self.dividers.contains(&next_element.clone()) {
            add_citation.start_chap = num;
            self.curr_citation = Some(add_citation);
            return Address::Chapter
        }
        add_citation.start_chap = if prev_citation.end_chap.is_none() { prev_citation.start_chap.clone() } else {prev_citation.end_chap.clone()};
        self.curr_citation = Some(add_citation);
        self.update_curr_citation(CitationParts::Verse, curr_element);
        Address::Verse
    }

    pub fn new<'b>() -> CitationList  {
        let ranges: HashSet<String> = [ "-", "–", "–", "—"].iter().map(|x| String::from(*x)).collect();
        let dividers: HashSet<String> = [":", ".", ","].iter().map(|x| String::from(*x)).collect();
        let additions: HashSet<String> = [";", ".", ","].iter().map(|x| String::from(*x)).collect(); 
 
        let scrip_vec: Vec<ScriptureCitation> = Vec::new();

        CitationList {book: None, ranges: ranges, dividers: dividers, additions: additions, curr_citation: None, scrip_vec: scrip_vec}
    }

    pub fn insert(&mut self,  scripture_string: &str, library: &book_linking::Library) {

        // let citation = ScriptureCitation::new(&script_book.unwrap(), &first_chap.unwrap());
        let mut prev_element = Address::Book;

        let (book_name, cit_address) = cleaned_book_abbr(scripture_string);
        let mut scripture_books = library.match_book(&book_name);
        // let mut script_book = None;

        for (num, book) in scripture_books.drain().enumerate() {
            if num == 0 {
                // script_book = Some(book);
                self.book = Some(book.clone());
            } else {
                println!("\x1b[93mDid you mean: {}?\x1b[0m", book)
            }
        };
        if self.book.is_none() {
            self.scrip_vec = vec![];
            return 
        }

        let address_vec: Vec<String> = split_keep(&cit_address);
        for (num, element) in address_vec.iter().enumerate() {
            match prev_element {
                Address::Book => {
                    self.update_curr_citation(CitationParts::StartChap, element);
                    prev_element = Address::Chapter;
                },
                Address::Chapter => {
                    prev_element = self.chapter_previous(element);
                }, 
                Address::ChapterRange => {
                    self.update_curr_citation(CitationParts::EndChap, element);
                    prev_element = Address::Chapter;
                },
                Address::Divider => {
                    self.update_curr_citation(CitationParts::Verse, element);
                    prev_element = Address::Verse;
                },
                Address::Range => {
                    let next_element = if (num + 1) == address_vec.len() { None } else { Some(&address_vec[num + 1]) };
                    prev_element = self.handeling_ranges(next_element, element)
                },
                Address::Addition => {
                    let next_element = if (num + 1) == address_vec.len() { None } else { Some(&address_vec[num + 1]) };
                    prev_element = self.handeling_additions(next_element, element);
                },
                Address::Verse => {
                    prev_element = if self.ranges.contains(element) { Address::Range } else {Address::Addition};
                },

                _ => {
                    println!("\x1b[91mError in Formating Citation:\x1b[0m {}", scripture_string);
                    println!("prev_element {:?}", prev_element);
                },
            }
        }
        if !self.curr_citation.is_none() {
            self.scrip_vec.push(self.curr_citation.clone().unwrap());
        }
    }
    
}



fn grab_book_abbr(scripture_string: &str) -> Option<regex::Match> {
    lazy_static! {
        static ref SCRIPT_ABBREVIATION_REGEX: Regex = Regex::new(r"^(I{1,3}V?|i{1,3}v?|\d{1,3})? ?(\w+).?").unwrap();
    }
    SCRIPT_ABBREVIATION_REGEX.find(scripture_string)

}

fn convert_str_to_address_num(num: &String) -> Option<i16> {
    let is_roman = roman_numerals::is_roman_numeral(num);
    let i16_num = match is_roman {
        true => Some(roman_numerals::convert_to_numbers(num)),
        false => num.parse::<i16>().ok(),
    };
    i16_num
}

fn cleaned_book_abbr(scripture_string: &str) -> (String, String) {
    let mat = grab_book_abbr(scripture_string).unwrap();
    let book = mat.as_str().replace(".", "");

    let address = String::from(&scripture_string[mat.end()..]);
    (book, address)
}


fn split_keep(text: &str) -> Vec<String> {
    lazy_static! {
        static ref SPLIT_RE: Regex = Regex::new(r"(-|–|–|—|:|\.|,|;|,| ) ?").unwrap();
    }
    let mut result: Vec<String> = Vec::new();
    let mut last = 0;
    for mat in SPLIT_RE.find_iter(&text) {
        if mat.start() != last {
            let input_section: String = String::from(text[last..mat.start()].trim());
            if input_section != "" {
                result.push(input_section);
            }
        }
        let matching_str = mat.as_str().trim();
        if matching_str != "" {
            result.push(String::from(mat.as_str().trim()));
        }
        last = mat.end();
    }
    if last < text.len() {
        result.push(String::from(text[last..].trim()));
    }
    result
}

#[derive(Debug)]
enum Address {
    Book,
    Chapter,
    Verse,
    ChapterRange,
    Range,
    Addition,
    Divider,
    Error,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn retrive_book_match() {
        let scrip_string = "II Sam. iv. 3, 2";
        let mat = grab_book_abbr(scrip_string);
        let book = mat.unwrap().as_str();

        assert_eq!(book, String::from("II Sam."));
    }

    #[test]
    fn cleaning_abbr() {
        let scrip_string = "II Sam. iv. 3, 2";
        let (book_name, address) = cleaned_book_abbr(scrip_string);

        assert_eq!((book_name, address), (String::from("II Sam"), String::from(" iv. 3, 2")));
    }

    #[test]
    fn test_address_split() {
        let text = "vi. 1; vii. 3";
        let res_vec = split_keep(text);

        println!("{:?}", res_vec);
        assert_eq!(vec!["vi", ".", "1", ";", "vii", ".", "3"], res_vec);
    }

    #[test]
    fn string_to_num_test() {
        let num: i16 = 4;
        let value = String::from("iv");
        let new_value = convert_str_to_address_num(&value);
        assert_eq!(num, new_value.unwrap());
    }

    #[test]
    fn test_verse_range() {
        let test = "II Sam. iv. 3-4";
        let mut scriptures = CitationList::new();
        let library = book_linking::Library::create().unwrap();
        scriptures.insert(test, &library);
        let should_value = ScriptureCitation {book:String::from("2 Samuel"), start_chap:Some(4), start_verse:Some(3), end_chap:Some(4), end_verse:Some(4)};
        let script = scriptures.scrip_vec[0].clone();
        println!("Should: {:?}", should_value);
        println!("Script: {:?}", script);
        assert_eq!(should_value.book, script.book);
        assert_eq!(should_value.start_chap, script.start_chap);
        assert_eq!(should_value.start_verse, script.start_verse);
        assert_eq!(should_value.end_chap, script.end_chap);
        assert_eq!(should_value.end_verse, script.end_verse);
    }

    #[test]
    fn test_verse_addition() {
        let test = "II Sam. 1:3, 4";
        let mut scriptures = CitationList::new();
        let library = book_linking::Library::create().unwrap();
        scriptures.insert(test, &library);
        let should_vec = vec![
            ScriptureCitation {book:String::from("2 Samuel"), start_chap:Some(1), start_verse:Some(3), end_chap:None, end_verse:None}, 
            ScriptureCitation {book:String::from("2 Samuel"), start_chap:Some(1), start_verse:Some(4), end_chap:None, end_verse:None}, 
        ];
        assert_eq!(should_vec[0].start_chap, scriptures.scrip_vec[0].start_chap);
        assert_eq!(should_vec[0].start_verse, scriptures.scrip_vec[0].start_verse);
    }

    #[test]
    fn test_isa() {
        let test = "Isa. 3:1";
        let mut scriptures = CitationList::new();
        let library = book_linking::Library::create().unwrap();
        scriptures.insert(test, &library);
        let should = ScriptureCitation {book:String::from("Isaiah"), start_chap:Some(3), start_verse:Some(1), end_chap:None, end_verse:None};
        println!("{:?}", scriptures.scrip_vec);
        assert_eq!(should.book, scriptures.scrip_vec[0].book);
    }
}
