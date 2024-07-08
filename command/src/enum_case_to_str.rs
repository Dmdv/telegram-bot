use regex::Regex;
use regex_split::RegexSplit;

pub fn to_command_str(text: &str) -> String {
    format!("/{}", Regex::new("[A-Z][^A-Z]*")
        .unwrap()
        .split_inclusive(text)
        .filter(|r| *r != "")
        .map(|r| r.to_lowercase())
        .collect::<Vec<String>>()
        .join("_"))
}

// fn to_upper_first(text: &str) -> String {
//     let mut c = text.chars();
//     match c.next() {
//         None => String::new(),
//         Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
//     }
// }
//
// pub fn from_command_str(text: &str) -> String {
//     text.replace("/", "").split("_").map(to_upper_first).collect::<Vec<_>>().join("")
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_command_str() {
        assert_eq!(to_command_str("StartTest"), "/start_test");
    }

    // #[test]
    // fn test_from_command_str() {
    //     assert_eq!(from_command_str("/start_test"), "StartTest");
    // }
}