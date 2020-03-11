pub fn get_diamond(c: char) -> Vec<String> {
    let c = c as u8;
    let len = (2 * (c - b'A') + 1) as usize;
    let mut ans = vec![vec![b' '; len]; len];
    ans.iter_mut()
        .zip((b'A'..=c).enumerate().chain((b'A'..c).enumerate().rev()))
        .for_each(|(s, (i, c))| {
            s[(len - 1) / 2 - i] = c;
            s[(len - 1) / 2 + i] = c;
        });
    ans.into_iter()
        .map(|s| String::from_utf8(s).unwrap())
        .collect()
}
