extern crate num_bigint;

use std::collections::HashMap;

use num_bigint::BigUint;

type BigMap = HashMap<BigUint, u32>;

fn build_table(h: &BigUint, g: &BigUint, p: &BigUint, b: u32) -> BigMap {
    let mut table = HashMap::new();

    let two = vec![2];
    let two = BigUint::new(two);
    let p_minus_2 = p - &two;

    for x1 in 0..b {
        let bytes = x1.to_le_bytes();
        let big_x1 = BigUint::from_bytes_le(&bytes);
        let g_x1 = g.modpow(&big_x1, p);
        let g_x1_inverse = g_x1.modpow(&p_minus_2, p);
        let left = h * &g_x1_inverse % p;

        table.insert(left, x1);
    }

    table
}

fn lookup_x0_x1(table: &BigMap, g: &BigUint, p: &BigUint, b: u32) -> Option<(u32, u32)> {
    let big_b = BigUint::from_bytes_le(&b.to_le_bytes());

    for x0 in 0..b {
        let g_b = g.modpow(&big_b, p);

        let bytes = x0.to_le_bytes();
        let big_x0 = BigUint::from_bytes_le(&bytes);
        let right = g_b.modpow(&big_x0, p);

        if let Some(&x1) = table.get(&right) {
            return Some((x0, x1));
        }
    }
    None
}

fn find_x(x0: u32, x1: u32, b: u32) -> u64 {
    u64::from(x0) * u64::from(b) + u64::from(x1)
}

fn main() {
    println!("Meet-in-the-Middle Attack (MITM)");

    let h = vec![491];
    let h = BigUint::new(h);
    let g = vec![2];
    let g = BigUint::new(g);
    let p = vec![499];
    let p = BigUint::new(p);
    let b = 2u32.pow(3);

    let table = build_table(&h, &g, &p, b);
    match lookup_x0_x1(&table, &g, &p, b) {
        Some((x0, x1)) => {
            println!("x0: {}, x1: {}", x0, x1);
            let x = find_x(x0, x1, b);
            println!("x: {}", x);
        },
        None => println!("x not found"),
    };
}
