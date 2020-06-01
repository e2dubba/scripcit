
use std::collections::HashSet;
use regex::Regex;

mod book_linking;
mod roman_numerals;

pub struct ScriptureCitation {
    book: String, 
    start_chap: Option<i16>,
    start_verse: Option<i16>,
    end_chap: Option<i16>,
    end_verse: Option<i16>,
}

pub struct CitationList <'a> {
    book: Option<String>,
    ranges: HashSet<&'a str>,
    dividers: HashSet<&'a str>,
    additions: HashSet<&'a str>,
    curr_citation: Option<ScriptureCitation>,
    scrip_vec: Vec<ScriptureCitation>,

}

enum CitationParts {
    StartChap,
    StartVerse,
    Verse,
    EndChap,
    EndVerse,

}

impl ScriptureCitation {
    
    pub fn new(name: &str, start_chap: Option<i16>) -> ScriptureCitation {
        ScriptureCitation { 
            book: name.to_owned(), 
            start_chap: start_chap,
            start_verse: None,
            end_chap: None,
            end_verse: None,
        }
    }

}

impl<'a> CitationList <'a> {

    pub fn add(&mut self, scrip_cit: ScriptureCitation) {
        self.scrip_vec.push(scrip_cit);
    }

    fn update_divider(&mut self, divider: &'a str) {
        let mut divid: HashSet<_> = HashSet::new();
        divid.insert(divider);
        self.additions.remove(divider);
        self.dividers = divid;
    }

    fn chapter_previous(&mut self, element: &'a str) -> Address  {
        if self.ranges.contains(element) {
            let prev_element = Address::ChapterRange;
            return prev_element
        }
        if self.dividers.len() == 3 && !self.ranges.contains(element){
            let mut temp_set: HashSet<&'a str> = HashSet::new();
            temp_set.insert(element);
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


    fn update_curr_citation(&mut self, citation_part: CitationParts, element: &str) {
        let mut citation = match self.curr_citation {
            Some(ScriptureCitation) => {
                self.curr_citation.unwrap()
            }
            None => {
                let book = self.book.clone().unwrap();
                ScriptureCitation::new(&book, None)
            },
        };

        let num = convert_str_to_address_num(element);

        match citation_part {
            StartChap => { citation.start_chap = num; }, 
            StartVerse => {citation.start_verse = num; }, 
            Verse => {if citation.start_verse.is_none() { citation.start_verse = num } else { citation.end_verse = num };},
            EndChap => { citation.end_chap = num; },
            EndVerse => { citation.end_verse = num; },
        }

        self.curr_citation = Some(citation);
    }

    fn handeling_additions(&mut self, next_element: &str, curr_element: &str) -> Address {
        let prev_citation = self.curr_citation.map(|x| x).unwrap();
        let prev_chapter = prev_citation.start_chap;
        self.scrip_vec.push(prev_citation);
        let book = self.book.map(|x| x).unwrap();
        let mut add_citation = ScriptureCitation::new(&book, None);

        let num = convert_str_to_address_num(curr_element);

        let prev_element = if self.dividers.contains(next_element) {
            add_citation.start_chap = num;
            Address::Chapter
        } else {
            add_citation.start_chap = prev_citation.start_chap;
            add_citation.start_verse = num;
            Address::Verse
        };
        self.curr_citation = Some(add_citation);
        prev_element
    }


    pub fn new<'b>() -> CitationList <'b> {
        let ranges: HashSet<_> = [ "–", "–", "—"].iter().cloned().collect();
        let dividers: HashSet<_> = [":", ".", ","].iter().cloned().collect();
        let additions: HashSet<_> = [";", ".", ","].iter().cloned().collect(); 
 
        let scrip_vec: Vec<ScriptureCitation> = Vec::new();

        CitationList {book: None, ranges: ranges, dividers: dividers, additions: additions, curr_citation: None, scrip_vec: scrip_vec}
    }

    pub fn insert(&mut self,  scripture_string: &str, library: &book_linking::Library) {

        // let citation = ScriptureCitation::new(&script_book.unwrap(), &first_chap.unwrap());
        let mut prev_element = Address::Book;

        let (book_name, cit_address) = cleaned_book_abbr(scripture_string);
        let scripture_books = library.match_book(&book_name);
        let mut script_book = None;
        for (num, book) in scripture_books.iter().enumerate() {
            if num == 0 {
                script_book = Some(book);
                self.book = Some(book.clone());
            } else {
                println!("\x1b]93mDid you mean: {}?\x1b]0m", book)
            }
        }
        let address_vec: Vec<&str> = split_keep(&cit_address);
        for (num, element) in address_vec.iter().enumerate() {
            match prev_element {
                Book => {
                    let chap = element.parse::<i16>().unwrap();
                    let mut citation = ScriptureCitation::new(&script_book.as_ref().unwrap(), Some(chap));
                    prev_element = Address::Chapter;
                },
                Chapter => {
                    prev_element = self.chapter_previous(&element);
                }, 
                ChapterRange => {
                    self.update_curr_citation(CitationParts::EndChap, element);
                    prev_element = Address::Chapter;
                },
                Divider => {
                    self.update_curr_citation(CitationParts::Verse, element);
                    prev_element = Address::Verse;
                },
                Addition => {
                    prev_element = self.handeling_additions(address_vec[num + 1], element);
                },
                Verse => {
                    prev_element = if self.ranges.contains(element) { Address::Range } else {Address::Addition};
                },

                _ => {
                    println!("\x1b[91mError in Formating Citation:\x1b[0m {}", scripture_string);
                    println!("prev_element {:?}", prev_element);
                },
            }
        }

     
    }
    
}



fn grab_book_abbr(scripture_string: &str) -> Option<regex::Match> {
    lazy_static! {
        static ref SCRIPT_ABBREVIATION_REGEX: Regex = Regex::new(r"^(I{1,3}V?|i{1,3}v?|\d{1,3})? ?(\w+).?").unwrap();
    }
    SCRIPT_ABBREVIATION_REGEX.find(scripture_string)

}

fn convert_str_to_address_num(num: &str) -> Option<i16> {
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

fn split_keep<'a>(text: &'a str) -> Vec<&'a str> {
    lazy_static! {
        static ref SPLIT_RE: Regex = Regex::new(r"(–|–|—|:|\.|,|;|,| ) ?").unwrap();
    }
    let mut result = Vec::new();
    let mut last = 0;
    for mat in SPLIT_RE.find_iter(&text) {
        if mat.start() != last {
            let input_section: &str = &text[last..mat.start()].trim();
            result.push(input_section);
        }
        result.push(&mat.as_str().trim());
        last = mat.end();
    }
    if last < text.len() {
        result.push(&text[last..].trim());
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
        let (num, book_name, address) = cleaned_book_abbr(scrip_string);

        assert_eq!((num, book_name, address), (Some(2), String::from("sam"), String::from(" iv. 3, 2")));
    }

    #[test]
    fn test_address_split() {
        let text = "vi. 1; vii. 3";
        let res_vec = split_keep(text);

        println!("{:?}", res_vec);
        assert_eq!(vec!["vi", ".", "1", ";", "vii", ".", "3"], res_vec);
    }
}

