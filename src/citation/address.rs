
use std::collections::HashSet;
use regex::Regex;

mod book_linking;

pub struct ScriptureCitation {
    book: String, 
    start_chap: i16,
    start_verse: Option<i16>,
    end_chap: Option<i16>,
    end_verse: Option<i16>,
}


impl ScriptureCitation {
    
    pub fn new(name: &str, start_chap: &i16) -> ScriptureCitation {
        ScriptureCitation { 
            book: name.to_owned(), 
            start_chap: start_chap.to_owned(),
            start_verse: None,
            end_chap: None,
            end_verse: None,
        }
    }

}


fn grab_book_abbr(scripture_string: &str) -> Option<regex::Match> {
    lazy_static! {
        static ref SCRIPT_ABBREVIATION_REGEX: Regex = Regex::new(r"^(I{1,3}V?|i{1,3}V?|\d{1,3})? ?(\w+).?").unwrap();
    }
    SCRIPT_ABBREVIATION_REGEX.find(scripture_string)

}


fn cleaned_book_abbr(scripture_string: &str) -> (String, String) {
    let mat = grab_book_abbr(scripture_string).unwrap();
    let book = mat.as_str().replace(".", "");
    let address = String::from(&scripture_string[mat.end()..]);
    (book, address)
}

fn create_set_from_vec(vector: Vec<&str>) -> HashSet<&str> {
    let mut new_set: HashSet<&str> = HashSet::new();
    for i in vector {
        new_set.insert(i);
    }
    new_set 
}

fn split_keep(text: &str) -> Vec<&str> {
    lazy_static! {
        static ref SPLIT_RE: Regex = Regex::new(r"(–|–|—|:|\.|,|;|,| ) ?").unwrap();
    }
    let mut result = Vec::new();
    let mut last = 0;
    for mat in SPLIT_RE.find_iter(text) {
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

enum Address {
    Book,
    Chapter,
    Verse,
    Range,
    Addition,
    Divider,
}

pub fn scripture_parser(scripture_string: &str, library: book_linking::Library) -> Vec<ScriptureCitation> {
    let mut ranges = create_set_from_vec(vec![ "–", "–", "—"]);
    let mut dividers = create_set_from_vec(vec![":", ".", ","]);
    let mut additions = create_set_from_vec(vec![";", ".", ","]); 
    let mut scrip_vec = Vec::new();
    let (book_name, address) = cleaned_book_abbr(scripture_string);
    let scripture_books = library.match_book(&book_name);
    let mut script_book = None;
    let mut num = 0;
    for book in scripture_books {
        if num == 0 {
            script_book = Some(book);
        } else {
            println!("Did you mean: {}?", book)
        }
        num += 1;
    }
    
    let mut address_vec = split_keep(&address);

    // let citation = ScriptureCitation::new(&script_book.unwrap(), &first_chap.unwrap());
    let mut prev_element = Address.Book;
    let mut specific_divider: Option<&str> = None;
        
    for chunk in address_vec.chunks(3) {
        match prev_element {
            "book" => {
                chapter = chunk[0], 
                if ranges.contains(chunk[1]) {
                    end_chap = chunk[2];
                    push_citation
                }
                if dividers.contains(chunk[1]) {
                    start_verse = chunk[2];
                    specific_divider = chunk[1]
                }

            _ => println!("Goodby")

        }
    }

    }


    // let book = book_linking::book_split(name_match);

    scrip_vec
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

