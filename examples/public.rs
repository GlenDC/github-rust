#![deny(warnings)]
extern crate github;

use github::Client;
use github::activity;

fn main() {
    let client = Client::new_default("github-rust");
    client.hello();

    println!("Listing all events...");
    activity::events::list(client);
}
