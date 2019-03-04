#[macro_use] extern crate hex_literal;
extern crate aes_soft as aes;
extern crate block_padding;

use block_padding::{Pkcs7, Padding};
use aes::block_cipher_trait::generic_array::GenericArray;
use aes::block_cipher_trait::BlockCipher;
use aes::Aes128;

fn cbc_decrypt_block(cipher: &Aes128, prev_block: &[u8], block: &[u8]) -> Vec<u8> {
    let mut buf = GenericArray::clone_from_slice(block);

    cipher.decrypt_block(&mut buf);

    buf.iter()
        .zip(prev_block)
        .map(|(a, b)| a ^ b)
        .collect()
}

fn cbc_decrypt(key: &[u8], ciphertext: &[u8]) -> Vec<u8> {
    let key = GenericArray::from_slice(key);
    let cipher = Aes128::new(&key);

    let mut padded_msg: Vec<u8> = ciphertext.chunks(16)
        .collect::<Vec<_>>()
        .windows(2)
        .map(|pair| {
            cbc_decrypt_block(&cipher, pair[0], pair[1])
        })
        .flatten()
        .collect();

    let n = Pkcs7::unpad(&padded_msg).unwrap().len();
    padded_msg.truncate(n);
    padded_msg
}

fn bytes_to_u128(bytes: &[u8]) -> u128 {
    let mut fixed: [u8; 16] = Default::default();
    fixed.copy_from_slice(bytes);
    u128::from_be_bytes(fixed)
}

fn ctr_decrypt(key: &[u8], ciphertext: &[u8]) -> Vec<u8> {
    let mut iter = ciphertext.chunks(16);
    let iv = iter.next().unwrap();
    let iv = bytes_to_u128(iv);

    let key = GenericArray::from_slice(key);
    let cipher = Aes128::new(&key);

    iter.enumerate()
        .map(|(i, block)| {
            let ctr_blk = (iv + i as u128).to_be_bytes();
            let mut buf = GenericArray::clone_from_slice(&ctr_blk);
            cipher.encrypt_block(&mut buf);
            buf.iter()
                .zip(block)
                .map(|(a, b)| a ^ b)
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect()
}

fn main() {
    let key = hex!("140b41b22a29beb4061bda66b6747e14");
    let ciphertext = hex!("4ca00ff4c898d61e1edbf1800618fb28
                           28a226d160dad07883d04e008a7897ee
                           2e4b7465d5290d0c0e6c6822236e1daa
                           fb94ffe0c5da05d9476be028ad7c1d81");

    let decoded = cbc_decrypt(&key, &ciphertext);

    println!("{:?}", String::from_utf8_lossy(&decoded));

    let ctr_key = hex!("36f18357be4dbd77f050515c73fcf9f2");
    let ctr_ct = hex!("69dda8455c7dd4254bf353b773304eec
                       0ec7702330098ce7f7520d1cbbb20fc3
                       88d1b0adb5054dbd7370849dbf0b88d3
                       93f252e764f1f5f7ad97ef79d59ce29f
                       5f51eeca32eabedd9afa9329");

    let ctr_decoded = ctr_decrypt(&ctr_key, &ctr_ct);

    println!("{:?}", String::from_utf8_lossy(&ctr_decoded));
}
