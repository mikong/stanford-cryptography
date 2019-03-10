extern crate num_bigint;

use std::collections::HashMap;

use num_bigint::BigUint;

type BigMap = HashMap<BigUint, u32>;

fn build_table(h: &BigUint, g: &BigUint, p: &BigUint, b: u32) -> BigMap {
    let mut table = HashMap::with_capacity(b as usize);

    // Instead of doing modular inversion (g^x1)^(p-2) in the loop,
    // we can calculate g^(p-2) ahead of time
    let two = BigUint::new(vec![2]);
    let g_inverse = g.modpow(&(p - &two), p);

    // start with exponentiation base h instead of multiplying h,
    // h * g^(-x1), on every iteration
    let mut left = h.clone();
    table.insert(left.clone(), 0);
    for x1 in 1..b {
        // reuse exponentiation: simply multiply
        // by g^(-1) to increase exponent by 1
        left = &left * &g_inverse % p;
        table.insert(left.clone(), x1);
    }

    table
}

fn lookup_x0_x1(table: &BigMap, g: &BigUint, p: &BigUint, b: u32) -> Option<(u32, u32)> {
    let big_b = BigUint::from_bytes_le(&b.to_le_bytes());
    let g_b = g.modpow(&big_b, p);
    let mut right = BigUint::new(vec![1]);

    for x0 in 0..b {
        if let Some(&x1) = table.get(&right) {
            return Some((x0, x1));
        }

        // reuse exponentiation: simply multiply
        // by g^b to increase exponent by 1
        right = &right * &g_b % p;
    }
    None
}

fn find_x(x0: u32, x1: u32, b: u32) -> u64 {
    u64::from(x0) * u64::from(b) + u64::from(x1)
}

fn main() {
    println!("Meet-in-the-Middle Attack (MITM)");

    let h = BigUint::parse_bytes(b"3239475104050450443565264378728065788649097520952449527834792452971981976143292558073856937958553180532878928001494706097394108577585732452307673444020333", 10).unwrap();
    let g = BigUint::parse_bytes(b"11717829880366207009516117596335367088558084999998952205599979459063929499736583746670572176471460312928594829675428279466566527115212748467589894601965568", 10).unwrap();
    let p = BigUint::parse_bytes(b"13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084171", 10).unwrap();
    let b = 2u32.pow(20);

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
