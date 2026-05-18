pub fn abbreviate(phrase: &str) -> String {
    // todo!("Given the phrase '{phrase}', return its acronym");
    phrase
        .split([' ', '-'])
        .map(|word| word.trim_start_matches(|c: char| !c.is_alphabetic()))
        .flat_map(|word| {
            let chars: Vec<char> = word.chars().collect();
            chars
                .iter()
                .enumerate()
                .filter(|(i, c)| *i == 0 || (c.is_uppercase() && !chars[*i - 1].is_uppercase()))
                .map(|(_, c)| *c)
                .collect::<Vec<char>>()
        })
        .map(|chr: char| chr.to_ascii_uppercase())
        .filter(|chr| chr.is_alphabetic())
        .collect()
}
