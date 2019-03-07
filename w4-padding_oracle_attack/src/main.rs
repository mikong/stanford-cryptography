extern crate hyper;

use hyper::Client;
use hyper::rt::{self, Future};

fn main() {
    println!("Padding Oracle Attack!");

    rt::run(rt::lazy(|| {

        let client = Client::new();
        let uri = "http://crypto-class.appspot.com/po?er=".parse().unwrap();

        client
            .get(uri)
            .map(|res| {
                println!("Response: {}", res.status());
            })
            .map_err(|err| {
                println!("Error: {}", err);
            })

    }));
}
