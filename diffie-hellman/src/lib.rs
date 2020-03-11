use rand::{thread_rng, Rng};

fn modpow(a: u64, b: u64, p: u64) -> u64 {
    let mut res = 1;
    let mut a = a;
    let mut b = b;
    while b != 0 {
        if b % 2 == 1 {
            res = res * a % p;
        }
        b /= 2;
        a = a.pow(2) % p;
    }
    res
}

pub fn private_key(p: u64) -> u64 {
    thread_rng().gen_range(2, p)
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    modpow(g, a, p)
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    modpow(b_pub, a, p)
}
