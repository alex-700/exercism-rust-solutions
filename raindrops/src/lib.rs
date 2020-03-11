pub fn raindrops(n: u32) -> String {
    let ans: String = [(3, "Pling"), (5, "Plang"), (7, "Plong")]
        .iter()
        .filter_map(|&(a, b)| if n % a == 0 { Some(b) } else { None })
        .collect();
    if ans.is_empty() {
        n.to_string()
    } else {
        ans
    }
}
