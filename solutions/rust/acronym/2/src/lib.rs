pub fn abbreviate(phrase: &str) -> String {
    phrase
        .split(|c: char| c.is_whitespace() || c == '-' || c == '_')
        .flat_map(|word| {
            word.chars()
                .zip(std::iter::once(' ').chain(word.chars()))
                .filter_map(|(curr, prev)| {
                    if !curr.is_alphabetic() {
                        return None;
                    }
                    let is_word_start = !prev.is_alphabetic() && prev != '\'';
                    let is_camel_transition = curr.is_uppercase() && prev.is_lowercase();
                    if is_word_start || is_camel_transition {
                        Some(curr)
                    } else {
                        None
                    }
                })
        })
        .map(|c| c.to_ascii_uppercase())
        .collect()
}
