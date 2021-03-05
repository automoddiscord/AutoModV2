use regex::{Regex};

use lazy_static::lazy_static;

lazy_static! {
    static ref ID_RE: Regex = Regex::new(r"<@!?([0-9]+)>").unwrap();
    static ref MENTION_RE: Regex = Regex::new(r"<@!?\d+>").unwrap();
    static ref MENTION_RE_SOLO: Regex = Regex::new(r"^<@!?(\d+)>$").unwrap();
}

pub fn get_mention(msg: &str) -> Option<u64> {
    MENTION_RE_SOLO
        .captures(msg)
        .and_then(|content| content.get(1))
        .and_then(|message| message.as_str().parse().ok())
}