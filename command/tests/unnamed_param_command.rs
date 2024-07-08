use command::BotCommand;

#[derive(BotCommand, Debug, PartialEq)]
enum Command {
    One(String),
    // Two(String, String),
}

#[test]
fn test_unnamed_enum_to_str() {
    assert_eq!(String::from(Command::One(String::from("one"))), "/one \"[\"one\"]\"");
    // assert_eq!(String::from(Command::Two(String::from("one"), String::from("two"))), "/one \"[\"test\",\"two\"]\"");
}

#[test]
fn test_unnamed_str_to_enum() {
    assert_eq!(Command::try_from("/one \"[\"one\"]\""), Ok(Command::One("\"[\"one\"]\"".to_owned())));
}

#[test]
fn test_pars_str_args() -> Vec<String> {
    let s = "\"[\"one\"]\"";
    let mut c = s.chars();
    match c.next() {
        None => vec![],
        Some(_) => {
            match c.next() {
                None => vec![],
                Some(f) => {
                    if f.to_string() == "[" {

                    }
                }
            }
        }
    }
}