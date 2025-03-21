/// have Bob reply to the incoming message: {message}
pub fn reply(mut message: &str) -> &str {
    message = message.trim();

    let s = message.trim_matches(|c: char| !c.is_alphabetic());
    let all_upper = !s.is_empty() && s.to_uppercase() == s;

    if message.is_empty() {
        "Fine. Be that way!"
    } else if message.ends_with("?") && all_upper {
        "Calm down, I know what I'm doing!"
    } else if message.ends_with("?") {
        "Sure."
    } else if all_upper {
        "Whoa, chill out!"
    } else {
        "Whatever."
    }
}
