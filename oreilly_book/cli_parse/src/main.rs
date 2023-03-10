use std::env;

fn main() {
    let mut numbers: Vec<u64> = Vec::new();

    for arg in env::args().skip(1) {}
}

fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if (m < n) {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}
