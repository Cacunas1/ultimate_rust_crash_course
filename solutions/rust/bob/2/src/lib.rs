pub fn reply(message: &str) -> &str {
    let is_empty = message.trim().is_empty();
    let is_question = message.trim().ends_with('?');
    let has_letter = false;
    let all_caps = true;

    let (has_letter, all_caps) = message.chars().fold((has_letter, all_caps), |mut acc, c| {
        if c.is_alphabetic() {
            acc.0 = true;
            if c.is_lowercase() {
                acc.1 = false;
            }
        }
        acc
    });

    let is_yelling = has_letter && all_caps;

    if is_empty {
        "Fine. Be that way!"
    } else {
        if is_question {
            if is_yelling {
                "Calm down, I know what I'm doing!"
            } else {
                "Sure."
            }
        } else if is_yelling {
            "Whoa, chill out!"
        } else {
            "Whatever."
        }
    }
}
