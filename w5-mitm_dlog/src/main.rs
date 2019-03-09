extern crate num_bigint;

use num_bigint::BigUint;

fn main() {
    println!("Meet-in-the-Middle Attack (MITM)");

    let hv = vec![491];
    let h = BigUint::new(hv);
    let gv = vec![2];
    let g = BigUint::new(gv);
    let pv = vec![499];
    let p = BigUint::new(pv);
    let two = vec![2];
    let big2 = BigUint::new(two);
    let mut left: BigUint;

    let b = 2u32.pow(3);
    for x1 in 0..b {
        let bytes = x1.to_le_bytes();
        let big_x1 = BigUint::from_bytes_le(&bytes);
        let gx1 = g.modpow(&big_x1, &p);
        let p2_exp = &p - &big2;
        let gnx1 = gx1.modpow(&p2_exp, &p);
        left = &h * &gnx1 % &p;
        println!("{:?}", left);
    }
}
