#![deny(warnings)]
extern crate github;

use github::Client;
use github::activity;

fn main() {
    let client = &mut Client::new("github-rust");
    client.hello();

    println!("Listing all repo events for glendc/github-rust:");
    let (repo_list_resp, _) = activity::events::list_repos(client, "glendc", "github-rust");
    for resp in repo_list_resp {
        println!("Event #{} @ '{}' by actor {};",
            resp.id, resp.created_at, resp.actor.login);
    }
}
