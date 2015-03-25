#![deny(warnings)]
extern crate github;

use github::Client;

fn main() {
    let client = Client::new_default("glendc");
    client.hello();
}
