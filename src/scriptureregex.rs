use itertools::Itertools;
fn regroup(options: Vec<String>) -> String {
    let mut combined_group: String = "(".to_owned();
    let grouped: String = options.into_iter().intersperse(String::from("|")).collect();
    combined_group.push_str(&grouped);
    combined_group.push_str(&String::from(")"));
    combined_group
}

fn create_group(addition: String) -> Box<dyn Fn(Vec<String>) -> String> {
    Box::new(move |x| {
        let mut combined_group = regroup(x);
        combined_group.push_str(&addition);
        combined_group
    })
}


pub fn regex_creator() -> String {
    let maybe = create_group(String::from("?"));
    let some = create_group(String::from("+"));

    let word = String::from(r"\w+.?");
    let roman_numerals = String::from(r"[ivxlc]+");

    // Create book number options
    let book_num_roman = String::from(r"I{1,3}");
    let lower_book_num_roman = String::from(r"i{1,3}");
    let three_digits = String::from(r"\d{1,3}");
    let book_num_vec = vec![book_num_roman, lower_book_num_roman, three_digits.clone()];
    let mut book_num = regroup(book_num_vec);
    book_num.push_str(&maybe(vec![String::from(r"\s")]));
    book_num = regroup(vec![book_num]);

    // Create Chapter Verse Address 
    let mut address = regroup(vec![three_digits.clone(), roman_numerals.clone()]);
    let seperators = String::from(r"[:,.]");
    address.push_str(&seperators);
    address.push_str(&maybe(vec![String::from(" ")]));
    address.push_str(&three_digits);
    // end_range for the verses
    let mut end_range = regroup(vec![String::from("-"), String::from("–")]);
    end_range.push_str(&three_digits);
    end_range.push_str(&maybe(vec![seperators.clone(), three_digits.clone()]));

    // Putting the book number with address for a citation
    let mut citation = book_num;
    citation.push_str(&String::from("?"));
    citation.push_str(&word);
    citation.push_str(&String::from(r"\s"));
    citation.push_str(&address);
    citation.push_str(&maybe(vec![end_range.clone()]));

    // possible additional addresses 
    let mut additional_address = String::from(r"[:,;.\-]"); // Seperators
    additional_address.push_str(&maybe(vec![String::from(r"\s")]));
    additional_address.push_str(&regroup(vec![three_digits.clone(), roman_numerals.clone()]));
    additional_address = some(vec![additional_address]);

    citation.push_str(&maybe(vec![additional_address]));
    citation
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_group() {
        let regex_frag = vec![String::from("a"), String::from("b"), String::from("c")];
        assert_eq!(regroup(regex_frag), "(a|b|c)");
    }

    #[test]
    fn test_maybe() {
        let regex_frag = vec![String::from("a"), String::from("b"), String::from("c")];
        let maybe = create_group(String::from("?"));
        assert_eq!("(a|b|c)?", maybe(regex_frag));
    }

    #[test]
    fn test_some() {
        let regex_frag = vec![String::from("a"), String::from("b"), String::from("c")];
        let some = create_group(String::from("+"));
        assert_eq!(some(regex_frag), "(a|b|c)+");
    }
}
