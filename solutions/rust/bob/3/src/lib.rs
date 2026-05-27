pub fn reply(message: &str) -> &str {
    let trimmed = message.trim();
    let is_empty = trimmed.is_empty();
    let is_question = trimmed.ends_with('?');

    let (has_letter, all_caps) =
        message
            .chars()
            .fold((false, true), |(_has_letter, _all_caps), c| {
                if c.is_alphabetic() {
                    (true, _all_caps && !c.is_lowercase())
                } else {
                    (_has_letter, _all_caps)
                }
            });

    let is_yelling = has_letter && all_caps;

    if is_empty {
        "Fine. Be that way!"
    } else if is_question {
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
