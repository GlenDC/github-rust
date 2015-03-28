#![deny(warnings)]

extern crate github;

use github::Client;
use github::activity;

fn main() {
    let client = &mut Client::new("?");
    client.hello();

    println!("Listing all repo events for glendc/github-rust:");
    let (repo_list_resp, _) = activity::events::list_repos(client, "glendc", "github-rust");
    for resp in repo_list_resp {
        println!("Event #{} @ '{}' by actor {};",
            resp.id, resp.created_at, resp.actor.login);
    }
    println!("We have {} unauthed requests remaining, resets on {}.",
        client.x_rate_limit.get_remaining(), client.x_rate_limit.get_reset())
}
