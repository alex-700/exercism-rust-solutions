use unicode_segmentation::UnicodeSegmentation;

/// Compute the Scrabble score for a word.
pub fn score(word: &str) -> u64 {
    word.graphemes(true)
        .filter_map(|s| if s.is_ascii() { s.chars().next() } else { None })
        .map(|c| match c.to_ascii_uppercase() {
            'A' | 'E' | 'I' | 'O' | 'U' | 'L' | 'N' | 'R' | 'S' | 'T' => 1,
            'D' | 'G' => 2,
            'B' | 'C' | 'M' | 'P' => 3,
            'F' | 'H' | 'V' | 'W' | 'Y' => 4,
            'K' => 5,
            'J' | 'X' => 8,
            'Q' | 'Z' => 10,
            _ => 0,
        })
        .sum()
}
