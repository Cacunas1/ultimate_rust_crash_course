use std::collections::HashSet;
use unicode_segmentation::UnicodeSegmentation;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    // first, I create a placeholder for the solution
    let mut answer: HashSet<&'a str> = HashSet::new();
    // then, I sort the original word, for easier comparison
    let mut sorted_target: Vec<String> =
        word
        .graphemes(true)
        .map(|c| c.to_lowercase())
        .collect::<Vec<String>>();
    println!("Target word in graphemes lowercased: {:#?}", sorted_target);

    // I check for every possible anagram
    for candidate in possible_anagrams {
        // if the candidate is the same as the given word, I discard it
        if *candidate == word {
            continue;
        }
        // I also sort the candidate, so I compare only once to see if it is an anagram
        let mut sorted_candidate: Vec<String> = candidate
            .graphemes(true)
            .map(|c| c.to_lowercase())
            .collect::<Vec<String>>();
        println!("Candidate word in graphemes lowercased: {:#?}", sorted_candidate);

        // if both words sorted are equal, then they are actually an anagram of each other
        if sorted_target != sorted_candidate {
            sorted_target.sort();
            sorted_candidate.sort();
            if sorted_target == sorted_candidate {
                answer.insert(candidate);
            }
        }
    }

    answer
}
