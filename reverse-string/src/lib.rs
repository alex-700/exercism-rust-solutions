pub fn reverse(input: &str) -> String {
    use unicode_segmentation::UnicodeSegmentation;
    input.graphemes(true).rev().collect()
}
