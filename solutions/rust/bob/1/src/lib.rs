pub enum Reply {
    Silence,
    CapsQuestion,
    Yell,
    Question,
    AnythingElse,
}

pub fn classify_reply(message: &str) -> Reply {
    if message.trim().is_empty() {
        Reply::Silence
    } else if message.trim().ends_with("?") {
        if message
            .chars()
            .all(|c| c.is_uppercase() || !c.is_alphabetic())
            && message.chars().any(|c| c.is_uppercase())
        {
            Reply::CapsQuestion
        } else {
            Reply::Question
        }
    } else if message
        .chars()
        .all(|c| c.is_uppercase() || !c.is_alphabetic())
        && message.chars().any(|c| c.is_uppercase())
    {
        Reply::Yell
    } else {
        Reply::AnythingElse
    }
}

pub fn reply(message: &str) -> &str {
    match classify_reply(message) {
        Reply::AnythingElse => "Whatever.",
        Reply::CapsQuestion => "Calm down, I know what I'm doing!",
        Reply::Question => "Sure.",
        Reply::Silence => "Fine. Be that way!",
        Reply::Yell => "Whoa, chill out!",
    }
}
