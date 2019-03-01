extern crate hex;

fn update_key(key: &mut [u8], c1: &[u8], c2: &[u8], c3: &[u8]) {
    let min = [key.len(), c1.len(), c2.len(), c3.len()].iter().min().unwrap().clone();

    for i in 0..min {
        if key[i] != 0 || c1[i] == c2[i] || c1[i] == c3[i] || c2[i] == c3[i] {
            continue;
        }

        let c12 = c1[i] ^ c2[i];
        let c13 = c1[i] ^ c3[i];
        let c23 = c2[i] ^ c3[i];

        if c12.is_ascii_alphabetic() && c13.is_ascii_alphabetic() {
            key[i] = c1[i] ^ b' ';
        } else if c12.is_ascii_alphabetic() && c23.is_ascii_alphabetic() {
            key[i] = c2[i] ^ b' ';
        } else if c13.is_ascii_alphabetic() && c23.is_ascii_alphabetic() {
            key[i] = c3[i] ^ b' ';
        }
    }
}

fn main() {
    let c1 = "315c4eeaa8b5f8aaf9174145bf43e1784b8fa00dc71d885a804e5ee9fa40b16349c146fb77";
    let c2 = "234c02ecbbfbafa3ed18510abd11fa724fcda2018a1a8342cf064bbde548b12b07df44ba71";
    let c3 = "32510ba9a7b2bba9b8005d43a304b5714cc0bb0c8a34884dd91304b8ad40b62b07df44ba6e";
    let c4 = "32510ba9aab2a8a4fd06414fb517b5605cc0aa0dc91a8908c2064ba8ad5ea06a029056f47a";
    let c5 = "3f561ba9adb4b6ebec54424ba317b564418fac0dd35f8c08d31a1fe9e24fe56808c213f17c";
    let c6 = "32510bfbacfbb9befd54415da243e1695ecabd58c519cd4bd2061bbde24eb76a19d84aba34";
    let c7 = "32510bfbacfbb9befd54415da243e1695ecabd58c519cd4bd90f1fa6ea5ba47b01c909ba76";
    let c8 = "315c4eeaa8b5f8bffd11155ea506b56041c6a00c8a08854dd21a4bbde54ce56801d943ba70";
    let c9 = "271946f9bbb2aeadec111841a81abc300ecaa01bd8069d5cc91005e9fe4aad6e04d513e96d";
    let c10 = "466d06ece998b7a2fb1d464fed2ced7641ddaa3cc31c9941cf110abbf409ed39598005b339";
    let target_ciphertext = "32510ba9babebbbefd001547a810e67149caee11d945cd7fc81a05e9f85aac650e9052ba6a";

    // TODO: full, variable-length ciphertext
    let ciphertexts = vec![c1, c2, c3, c4, c5, c6, c7, c8, c9, c10, target_ciphertext];
    let ciphertexts: Vec<Vec<u8>> = ciphertexts.iter()
        .map(|c| hex::decode(c).unwrap())
        .collect();

    // Note: assumes even length
    let keylen = target_ciphertext.len() / 2;
    let mut key = vec![0; keylen];

    // TODO: Optimize
    for i in 0..ciphertexts.len() - 2 {
        for j in i + 1..ciphertexts.len() - 1 {
            for k in j + 1..ciphertexts.len() {
                update_key(&mut key, &ciphertexts[i], &ciphertexts[j], &ciphertexts[k]);
            }
        }
    }

    // println!("{:?}", key);

    let target_hex = hex::decode(target_ciphertext).unwrap();
    let decode: Vec<u8> = target_hex.iter()
        .enumerate()
        .map(|(i, c)| key[i] ^ c)
        .collect();
    println!("{}", String::from_utf8_lossy(&decode));
}
