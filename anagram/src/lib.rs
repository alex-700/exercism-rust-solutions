use counter::Counter;
use std::collections::HashSet;
use unicode_segmentation::UnicodeSegmentation;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let word = word.to_lowercase();
    let word_letters: Counter<_> = word.graphemes(true).collect();
    possible_anagrams
        .iter()
        .filter(|pattern| {
            let pattern = pattern.to_lowercase();
            pattern != word && word_letters == pattern.graphemes(true).collect::<Counter<_>>()
        })
        .cloned()
        .collect()
}
