extern crate regex;

use regex::Regex;
use std::collections::HashMap;

// take a regex with groups and return a data structure of some kind

pub struct Matcher {
    regex: regex::Regex,
}

impl Matcher {
    fn new(regex: Regex) -> Matcher {
        Matcher {
            regex: regex,
        }
    }

    fn match_line(&self, line: &str) -> HashMap<String, String> {
        let mut map = HashMap::new();
        self.regex.captures(&line).map(|cap| {
            for group in cap.iter_named() {
                map.insert(group.0.to_owned(), group.1.unwrap_or("").to_owned());
            }
        });

        map
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use regex::Regex;

    #[test]
    pub fn can_create_a_matcher() {
        let re = Regex::new(r"\w+").unwrap();
        let matcher = Matcher::new(re);

        assert!(matcher.regex.is_match("hello"));
    }

    #[test]
    pub fn create_hash_map_of_groups_in_regex() {
        let re = Regex::new(r"(?P<title>\w+\.) \w+").unwrap();
        let matcher = Matcher::new(re);

        let line = "Ms. Person".to_string();
        let re_map = matcher.match_line(&line);
        assert!(re_map.contains_key("title"));
    }
}
