use std::collections::HashSet;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    factors
        .iter()
        .filter(|&&x| x != 0)
        .flat_map(|&x| (x..limit).step_by(x as usize))
        .collect::<HashSet<_>>()
        .into_iter()
        .sum()
}
