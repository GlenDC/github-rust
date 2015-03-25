#![deny(warnings)]
extern crate github;

use github::Client;
use github::activity;

fn main() {
    let client = Client::new_default("github-rust");
    client.hello();

    println!("Listing all events...");
    activity::events::list(&client);

    println!("Listing all repos events...");
    activity::events::list_repos(&client, "glendc", "glendc.com");

    println!("Listing all repos issue events...");
    activity::events::list_repos_issue(&client, "glendc", "glendc.com");
}
