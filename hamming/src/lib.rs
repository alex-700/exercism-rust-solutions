/// Return the Hamming distance between the strings,
/// or None if the lengths are mismatched.
pub fn hamming_distance(s1: &str, s2: &str) -> Option<usize> {
    if s1.len() == s2.len() {
        Some(s1.bytes().zip(s2.bytes()).filter(|(x, y)| x != y).count())
    } else {
        None
    }
}
