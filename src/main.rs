use num_bigint::BigUint;
use num_traits::{One, Zero};

fn find_order(n: &BigUint, a: &BigUint) -> BigUint {
    let mut k = BigUint::one();
    while k <= *n {
        if (k.clone() * a) % n == BigUint::zero() {
            return k;
        }
        k += BigUint::one();
    }
    n.clone()
}

fn main() {
    let n = BigUint::parse_bytes(b"9999922", 10).unwrap(); // Change this value to any positive integer to represent Z_n

    for a in 0..=10 {
        let a_biguint = BigUint::from(a as u32);
        let order = find_order(&n, &a_biguint);
        println!("Order of {} in Z_{} is {}", a, n, order);
    }
}




