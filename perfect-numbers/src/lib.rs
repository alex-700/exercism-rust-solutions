use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

fn aliquot_sum(num: u64) -> Option<u64> {
    match num {
        0 => None,
        1 => Some(0),
        n => Some(
            (2..)
                .take_while(|&i| i * i <= n)
                .filter(|&i| n % i == 0)
                .map(|i| if i * i == n { i } else { i + n / i })
                .sum::<u64>()
                + 1,
        ),
    }
}

pub fn classify(num: u64) -> Option<Classification> {
    aliquot_sum(num).map(|aliquot| match aliquot.cmp(&num) {
        Ordering::Less => Classification::Deficient,
        Ordering::Equal => Classification::Perfect,
        Ordering::Greater => Classification::Abundant,
    })
}
