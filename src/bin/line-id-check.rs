use std::env;

use std::collections::HashMap;
use strum::IntoEnumIterator;

use tfl_api_wrapper::{linemodels::LineID, Client, RequestBuilder};

fn get_client() -> Client {
    Client::new(env::var("APP_KEY").unwrap())
}

#[tokio::main]
async fn main() {
    println!("Checking LineIDs against the API...");

    let client = get_client();

    let mut lids = HashMap::new();
    for l in LineID::iter() {
        lids.insert(l.line().to_lowercase(), l);
    }

    let mut incorrect: HashMap<String, (String, String)> = HashMap::new();

    let mut missing: Vec<String> = Vec::new();

    let routes = client.routes().fetch().await.expect("Cannot fetch routes");

    for l in routes {
        match lids.remove(&l.id.to_lowercase()) {
            Some(r) => {
                if l.name != r.to_string() {
                    println!("Incorrect name: {}, \"{}\" -> \"{}\"", l.id, r, l.name);
                    incorrect.insert(l.id.to_string(), (r.to_string(), l.name));
                } else {
                    println!("OK line: {} / {}", l.id, l.name);
                }
            }
            None => {
                println!("Missing line: {}", l.id);
                missing.push(l.id);
            }
        }
    }

    println!("Incorrect names: {:?}", incorrect);
    println!("Extra lines: {:?}", lids);
    println!("Missing lines: {:?}", missing);
}
