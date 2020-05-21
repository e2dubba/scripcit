pub mod roman_numerals {

    use numerals::roman;
    use regex::Regex;

    pub fn is_roman_numeral(numeral: &str) -> bool {
        lazy_static! {
            static ref NUMERAL_RE: Regex = regex::Regex::new(
                // r"(?i)(^| )([MDCLXVI])M*(C[MD]|D?C{0,3})(X[CL]|L?X{0,3})(I[XV]|V?I{0,3})").unwrap();
                // r"(?i)^([MDCLXVI])M*(C[MD]|D?C{0,3})(X[CL]|L?X{0,3})(I[XV]|V?I{0,3})$").unwrap();
                r"(?i)^(M{0,3})(D?C{0,3}|CM|CD)(L?X{0,3}|XC|XL)(V?I{0,3}|IX|IV)$").unwrap();
        }
        NUMERAL_RE.is_match(numeral)
    }

    pub fn convert_to_numbers(numeral: &str) -> i16 {
        let numeral_parsed = roman::Roman::parse(numeral).unwrap();
        numeral_parsed.value()
    }
}

pub mod book_linking {

    use std::collections::HashMap;
    use std::cmp::Ordering;

    pub struct Book {
        pub num: Option<i16>,
        pub name: String,
        pub idx: HashMap<char, Vec<usize>>,
    }

    impl Book {
        pub fn new(name_str: &str) -> Book {
            // let mut idx: HashMap<char, Vec<usize>> = HashMap::new();
            let mut idx = HashMap::new();
            for c in name_str.chars() {
                idx.insert(c, Vec::new());
            }
            for (index, c) in name_str.chars().enumerate() {
                if let Some(x) = idx.get_mut(&c) {
                    x.push(index);
                }
            }
                
            Book {num: None, name: String::from(name_str), idx: idx}
        }

        fn ordered_name_match(&self, other_name: &str) -> bool {
            let mut book_iter = self.name.chars();
            for c in other_name.chars() {
                let book_c = book_iter.next();
                let value = book_c.as_ref().map(|n|  &c == n );
                if value.unwrap() == false {
                    return false
                }
            }
            true
        }

        pub fn name_match(&self, other_name: &str) -> bool {
            if self.ordered_name_match(other_name) { 
                return true
            }
            let mut last = 0;
            let char_hash_map = &self.idx;
            for c in other_name.chars() {
                for char_vec in char_hash_map.get(&c) {
                    if char_vec.is_empty() {
                        return false
                    }
                    let cur = *char_vec.iter().max().as_ref().unwrap();
                    match last.cmp(cur) {
                        Ordering::Less => last = *cur,
                        Ordering::Greater => return false,
                        Ordering::Equal => last = *cur,
                    }
                }
            }
            true
        }
    }
}


#[cfg(test)]
mod tests {
    use super::roman_numerals::*;

    #[test]
    fn is_roman_upper_test() {
        let upper_numeral = "VI";
        assert!(is_roman_numeral(upper_numeral));
    }
    
    #[test]
    fn is_roman_lower_test() {
        let lower_numeral = "iv";
        assert!(is_roman_numeral(lower_numeral));
    }

    #[test]
    fn is_not_roman_test() {
        let lower_numeral = "XM";
        assert_eq!(is_roman_numeral(lower_numeral), false);
    }
    #[test]
    fn pure_roman_numeral() {
        let roman_numeral = "CMXC";
        assert!(is_roman_numeral(roman_numeral));
    }

    #[test]
    fn test_number_conversion() {
        let roman_numeral = "CMXC";
        assert_eq!(convert_to_numbers(&roman_numeral), 990);
    }

    use super::book_linking::*;

    #[test]
    fn consec_abbrev_match_test() {
        let nbook = Book::new("Genesis"); // {num: None, name: String::from("Genesis"), idx: None};
        let abbrev = "Gen";
        assert_eq!(nbook.name_match(abbrev), true);
    }

    #[test]
    fn irregular_abbrev_match_test() {
        let nbook = Book::new("John");
        let abbrev = "Jn";
        assert!(nbook.name_match(abbrev));
        let nabbrev = "Joh";
        assert!(nbook.name_match(nabbrev));
        let wabbrev = "Jdg";
        assert!(!nbook.name_match(wabbrev));
    }
}
