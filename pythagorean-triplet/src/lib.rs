use std::collections::HashSet;

// a = k(m^2 - n^2)
// b = k(2mn)
// c = k(m^2 + n^2)
// a + b + c = 2km^2 + 2kmn = 2km(m + n) = sum
pub fn find(sum: u32) -> HashSet<[u32; 3]> {
    if sum % 2 != 0 {
        HashSet::new()
    } else {
        let half_of_sum = sum / 2;
        (2..)
            .filter(|&i| half_of_sum % i == 0)
            .take_while(|&i| i * i < half_of_sum)
            .flat_map(|m| {
                let sum_divided_by_2m = half_of_sum / m;
                (m + 1..=std::cmp::min(sum_divided_by_2m, 2 * m - 1))
                    .filter(move |&n_plus_m| sum_divided_by_2m % n_plus_m == 0)
                    .map(move |n_plus_m| {
                        let k = sum_divided_by_2m / n_plus_m;
                        let n = n_plus_m - m;
                        assert_eq!(2 * k * m * (m + n), sum);
                        let mut ans = [
                            k * (m.pow(2) - n.pow(2)),
                            k * 2 * m * n,
                            k * (m.pow(2) + n.pow(2)),
                        ];
                        ans.sort();
                        ans
                    })
            })
            .collect()
    }
}
