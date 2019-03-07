extern crate hyper;

use hyper::{Body, Client};
use hyper::client::HttpConnector;
use hyper::http::Uri;
use hyper::rt::{self, Future};

const TARGET: &str = "http://crypto-class.appspot.com/po?er=";

#[derive(Debug)]
struct PaddingOracle {
    client: Client<HttpConnector, Body>,
    target_uri: String,
}

impl PaddingOracle {
    fn new(target_uri: &str) -> PaddingOracle {
        let client = Client::new();
        let target_uri = target_uri.to_string();

        PaddingOracle { client, target_uri }
    }

    fn get_uri(&self, q: &str) -> Uri {
        let uri = format!("{}{}", self.target_uri, q);
        uri.parse::<Uri>().unwrap()
    }

    fn query(&self, q: &str) -> impl Future<Item=(), Error=()> {
        let uri = self.get_uri(q);

        self.client
            .get(uri)
            .map(|res| {
                println!("Response: {}", res.status()); 
            })
            .map_err(|err| {
                println!("Error: {}", err);
            })
    }
}

fn main() {
    println!("Padding Oracle Attack!");

    rt::run(rt::lazy(|| {
        let po = PaddingOracle::new(TARGET);
        po.query("")
    }));
}
