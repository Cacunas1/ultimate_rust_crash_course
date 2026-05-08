use std::collections::HashSet;
use unicode_segmentation::UnicodeSegmentation;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    // A helper closure that takes a &str and returns a sorted Vec<String>
    let sorted_graphemes = |s: &str| -> Vec<String> {
        let mut v: Vec<String> = s.graphemes(true).map(|c| c.to_lowercase()).collect();
        v.sort();
        v
    };

    let target = sorted_graphemes(word);

    // Then in your filter:
    possible_anagrams
        .iter()
        .filter(|&&candidate| {
            // condition 1: not the same word (case insensitive)
            candidate.to_lowercase() != word.to_lowercase()
        })
        .filter(|&&candidate| {
            let sorted_candidate = sorted_graphemes(candidate);
            sorted_candidate == target
        })
        .copied()
        .collect()
}
