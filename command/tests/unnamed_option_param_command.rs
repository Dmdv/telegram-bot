// use command::BotCommand;
//
// #[derive(BotCommand, Debug, PartialEq)]
// enum TestEnum1 {
//     Start(Option<String>),
//     Unknown(String)
// }
//
// #[test]
// fn test_unnamed_option_enum_to_str() {
//     assert_eq!(format!("{}", TestEnum1::Start(Some(String::from("test")))), "/start \"[\"test\"]\"");
// }