extern crate num_bigint;
extern crate num_traits;

use num_bigint::{BigInt, BigUint, ToBigInt};
use num_traits::{One, Zero};

// Returns the gcd and coefficients of Bézout's identity
fn extended_gcd(a: &BigInt, b: &BigInt) -> (BigInt, BigInt, BigInt) {
    if a.is_zero() {
        (b.clone(), Zero::zero(), One::one())
    } else {
        let (g, s, t) = extended_gcd(&(b % a), a);
        (g, t - (b / a) * &s, s)
    }
}

fn mod_inverse(a: &BigInt, m: &BigInt) -> Option<BigInt> {
    let (gcd, s, _) = extended_gcd(a, m);
    if gcd == One::one() {
        return Some((s % m + m) % m);
    }
    None
}

fn main() {
    println!("RSA Problem");

    // Factoring challenge #1:
    let n = BigUint::parse_bytes(b"1797693134862315907729305190789024733617\
                                   9769789423065727343008115773267580550562\
                                   0686985379449212982959585501387537164015\
                                   7101398586478337786069255834975410851965\
                                   9161512805757594075263500747593528871082\
                                   3649949940771895617054361149474865046711\
                                   0151015639406805275400715845608785776637\
                                   43040086340742855278549092581", 10).unwrap();

    let one: BigUint = One::one();
    // FIXME: do proper ceiling
    let a: BigUint = n.sqrt() + &one;
    let x = (&a * &a - &n).sqrt();
    let p = &a - &x;
    let q = &a + &x;
    if &p * &q == n {
        println!("1. Prime p: {:?}", p.to_string());
    }

    // Factoring challenge #2:
    let n = BigUint::parse_bytes(b"6484558428080716696628242653467722787263\
                                   4372070697626306043907037879730861808111\
                                   6462714015276061417569195587321840254520\
                                   6554249067198924288448418393532819729885\
                                   3131051173864896596258282150250499026445\
                                   2100885281673303711142296421027840289307\
                                   6574586452336833570778346897158386460882\
                                   39640236866252211790085787877", 10).unwrap();

    let a: BigUint = n.sqrt() + &one;

    for i in 0..100_000u32 {
        let big_i: BigUint = i.into();
        let avg = &a + big_i;
        let x2 = &avg * &avg - &n;
        let x = (&x2).sqrt();
        let p = &avg - &x;
        let q = &avg + &x;
        if &p * &q == n {
            println!("2. Prime p: {:?}", p.to_string());
            break;
        }
    }

    // Factoring challenge #3:
    let n = BigUint::parse_bytes(b"7200622637473504252795644355255837383380\
                                   8445147399984182665305798191635569018833\
                                   7790423408664187663938485175264994017897\
                                   0835240791356868774411551320151882793318\
                                   1230909199624636189683657364311917409496\
                                   1348524639707885238799396839230364676670\
                                   2216270183532994432411921738127292761475\
                                   30748597302192751375739387929", 10).unwrap();

    let b: BigUint = (BigUint::from(24u32) * &n).sqrt() + &one;
    let y = (&b * &b - BigUint::from(24u32) * &n).sqrt();
    let p = (&b + &y) / BigUint::from(4u32);
    let q = (&b - &y) / BigUint::from(6u32);
    if &p * &q == n {
        println!("3. Prime p: {:?}", p.to_string());
    }

    // Challenge #4:
    let c = BigUint::parse_bytes(b"2209645186741038177630656113488341801741\
                                   0069787892831071731839143676135600120538\
                                   0042823296504735094243439462197515122564\
                                   6583996794288946076454204058156474898801\
                                   3734864120452325229320176487916666402997\
                                   5091887299716905260832220677716000193292\
                                   6087000957999372407745896777369781757126\
                                   7229951148662959627934791540", 10).unwrap();

    let n = BigUint::parse_bytes(b"1797693134862315907729305190789024733617\
                                   9769789423065727343008115773267580550562\
                                   0686985379449212982959585501387537164015\
                                   7101398586478337786069255834975410851965\
                                   9161512805757594075263500747593528871082\
                                   3649949940771895617054361149474865046711\
                                   0151015639406805275400715845608785776637\
                                   43040086340742855278549092581", 10).unwrap();

    let a: BigUint = n.sqrt() + &one;
    let x = (&a * &a - &n).sqrt();
    let p = &a - &x;
    let q = &a + &x;

    // encryption exponent
    let e = BigInt::from(65_537);

    // Compute φ(N):
    let phi_n = (&p - &one) * (&q - &one);
    // Alternatively:
    // let phi_n = &n - &p - &q + &one;
    let signed_phi_n = phi_n.to_bigint().unwrap();

    // Obtain decryption exponent d; sk = (N, d):
    let d = mod_inverse(&e, &signed_phi_n).unwrap();
    let d = d.to_biguint().unwrap();

    // Decrypt ciphertext
    let m = c.modpow(&d, &n);
    let m_bytes = m.to_bytes_be();

    // Check first byte is 0x02
    if let Some(2) = m_bytes.first() {
        // Ignore bytes until after 0x00
        let plaintext: Vec<u8> = m_bytes.into_iter()
            .skip_while(|&b| b != 0)
            .skip(1)
            .collect();
        println!("4. plaintext: {:?}", String::from_utf8_lossy(&plaintext));
    }
}
