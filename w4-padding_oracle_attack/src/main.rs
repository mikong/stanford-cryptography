#[macro_use] extern crate hex_literal;
extern crate hex;
extern crate reqwest;

use reqwest::{Client, StatusCode, Url};

const TARGET: &str = "http://crypto-class.appspot.com/po?er=";

#[derive(Debug)]
struct PaddingOracle {
    client: Client,
    target_uri: String,
}

impl PaddingOracle {
    fn new(target_uri: &str) -> PaddingOracle {
        let client = Client::new();
        let target_uri = target_uri.to_string();

        PaddingOracle { client, target_uri }
    }

    fn get_uri(&self, q: &str) -> Url {
        let uri = format!("{}{}", self.target_uri, q);
        Url::parse(&uri).unwrap()
    }

    fn query(&self, q: &str) -> StatusCode {
        let uri = self.get_uri(q);

        let res = self.client.get(uri).send().unwrap();
        res.status()
    }
}

fn main() {
    println!("Padding Oracle Attack!");

    let ciphertext = hex!("f20bdba6ff29eed7b046d1df9fb70000
                           58b1ffb4210a580f748b4ac714c001bd
                           4a61044426fb515dad3f21f18aa577c0
                           bdf302936266926ff37dbf7035d5eeb4");

    let mut blocks_iter = ciphertext.chunks(16);
    let iv = blocks_iter.next().unwrap();
    let c0 = blocks_iter.next().unwrap();
    let mut ivp = [0u8; 16];
    let mut m0 = [0u8; 16];

    let po = PaddingOracle::new(TARGET);

    for (i, pad) in (1..=16).enumerate() {
        for guess in 0..=127 {
            let index = 15 - i;
            ivp[index] = iv[index] ^ pad ^ guess;
            for k in index+1..=15 {
                ivp[k] = iv[k] ^ pad ^ m0[k];
            }

            let q = format!("{}{}", hex::encode(ivp), hex::encode(c0));
            if let StatusCode::NOT_FOUND = po.query(&q) {
                println!("valid padding: {}", guess);
                m0[index] = guess;
                break;
            }
        }
    }

    println!("m0: {}", String::from_utf8_lossy(&m0));
}
