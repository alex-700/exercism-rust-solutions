use bit_vec::BitVec;

pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let n = (upper_bound + 1) as usize;
    let mut prime = BitVec::from_elem(n + 1, true);
    (2..n)
        .filter_map(|i| {
            if prime[i] {
                for j in (i * 2..n).step_by(i) {
                    prime.set(j, false);
                }
                Some(i as u64)
            } else {
                None
            }
        })
        .collect()
}
