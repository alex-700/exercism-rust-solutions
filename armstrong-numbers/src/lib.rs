pub fn is_armstrong_number(num: u32) -> bool {
    let mut v = Vec::with_capacity(9);
    let mut n = num;
    while n > 0 {
        v.push(n % 10);
        n /= 10;
    }
    let l = v.len() as u32;
    v.into_iter().map(|x| x.pow(l)).sum::<u32>() == num
}
