#[path = "../structs/mod.rs"]
mod structs;
extern crate woothee;
use isbot::Bots;
use lazy_static::lazy_static;
use woothee::parser::Parser;

lazy_static! {
    static ref parser: woothee::parser::Parser = Parser::new();
    static ref bots: Bots = Bots::default();
}

pub fn parse_user_agent(ua: &str) -> crate::structs::UserAgentParseResult::UserAgentParseResult {
    let result = parser.parse(ua);
    if result.is_none() {
        return crate::structs::UserAgentParseResult::UserAgentParseResult {
            operating_system: "-".to_string(),
            category: "-".to_string(),
            isBot: false,
            browser: "-".to_string(),
        };
    }

    let resultu = result.unwrap();
    return crate::structs::UserAgentParseResult::UserAgentParseResult {
        operating_system: resultu.os.to_string(),
        category: resultu.category.to_string(),
        isBot: bots.is_bot(ua),
        browser: resultu.name.to_string(),
    };
}
