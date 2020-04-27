pub fn nth(n: u32) -> u32 {
    fn is_prime(n: u32) -> bool {
        (2_u32..).take_while(|i| i * i <= n).all(|i| n % i != 0)
    }
    (2..).filter(|&p| is_prime(p)).nth(n as usize).unwrap()
}
