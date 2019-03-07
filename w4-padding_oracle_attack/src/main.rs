extern crate reqwest;

use reqwest::{Client, Url};

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

    fn query(&self, q: &str) {
        let uri = self.get_uri(q);

        match self.client.get(uri).send() {
            Ok(res) => println!("Response: {}", res.status()),
            Err(e) => println!("Error: {}", e),
        }
    }
}

fn main() {
    println!("Padding Oracle Attack!");

    let po = PaddingOracle::new(TARGET);
    po.query("");
}
