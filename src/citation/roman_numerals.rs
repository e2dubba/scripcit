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
    let striped_numeral = numeral.trim();
    let numeral_parsed = roman::Roman::parse(striped_numeral).unwrap();
    numeral_parsed.value()
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
