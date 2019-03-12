extern crate num_bigint;
extern crate num_traits;

use num_bigint::BigUint;
use num_traits::One;

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
}
