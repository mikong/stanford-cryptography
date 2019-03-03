#[macro_use] extern crate hex_literal;
extern crate aes_soft as aes;
extern crate block_padding;

use block_padding::{Pkcs7, Padding};
use aes::block_cipher_trait::generic_array::GenericArray;
use aes::block_cipher_trait::BlockCipher;
use aes::Aes128;

fn cbc_decrypt_block(key: &[u8], prev_block: &[u8], block: &[u8]) -> Vec<u8> {
    let key = GenericArray::from_slice(key);
    let mut buf = GenericArray::clone_from_slice(block);

    let cipher = Aes128::new(&key);
    cipher.decrypt_block(&mut buf);

    buf.iter()
        .zip(prev_block)
        .map(|(a, b)| a ^ b)
        .collect()
}

fn cbc_decrypt(key: &[u8], ciphertext: &[u8]) -> Vec<u8> {
    let mut padded_msg: Vec<u8> = ciphertext.chunks(16)
        .collect::<Vec<_>>()
        .windows(2)
        .map(|pair| {
            cbc_decrypt_block(key, pair[0], pair[1])
        })
        .flatten()
        .collect();

    let n = Pkcs7::unpad(&padded_msg).unwrap().len();
    padded_msg.truncate(n);
    padded_msg
}

fn main() {
    let key = hex!("140b41b22a29beb4061bda66b6747e14");
    let ciphertext = hex!("4ca00ff4c898d61e1edbf1800618fb28
                           28a226d160dad07883d04e008a7897ee
                           2e4b7465d5290d0c0e6c6822236e1daa
                           fb94ffe0c5da05d9476be028ad7c1d81");

    let decoded = cbc_decrypt(&key, &ciphertext);

    println!("{:?}", String::from_utf8_lossy(&decoded));
}
