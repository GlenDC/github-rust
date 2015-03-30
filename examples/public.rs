#![deny(warnings)]

extern crate github;

// Example of some public requests

use github::Client;
use github::error::*;
use github::activity::events::*;

fn main() {
    // simplest and most-used way of creating a client
    // requiring a User-Agent used for requests.
    let client = &Client::new("glendc");

    // An example of getting and quickly summarizing the most recent public github events.
    // In this case we simply print all error information found.
    println!("# Example: list_events");
    match list_events(client) {
        Ok((events, resp)) => {
            println!("listing public events succesfull, we have {} requsts remaining of {}. Limit resets @ {}...",
                resp.rate.remaining, resp.rate.limit, resp.rate.reset);
            for event in events {
                println!("event #{} at {} by {}...",
                    event.id, event.created_at, event.actor.login);
            }
        }
        Err(err) => {
            println!("list_events => {}", err);
            if let ClientError::Http(http_error) = err {
                for error in http_error.errors {
                    println!("    {}", error);
                }
            }
        }
    }

    // An example of a request that we expect to fail,
    // because the repository doesn't exist (404).
    println!("# Example: failed list_my_repo_events");
    if let Err(err) = list_my_repo_events(client, "42") {
        println!("list_repo_events failed: {}", err);
    }

    // An example of a request that we expect to fail,
    // because we are unauthorized (404).
    println!("# Example: failed list_organisation_events");
    if let Err(err) = list_my_organisation_events(client, "PortalGaming") {
        println!("list_organisation_events failed: {}", err);
    }

    // An example of getting and quickly summarizing
    // the most recent public issue events of the repo of this Client Library.
    // Most structs and enums in this lib are also debug-able, as shown here.
    println!("# Example: list_my_repo_issue_events for `github-rust`");
    if let Ok((events, resp)) =  list_my_repo_issue_events(client, "github-rust") {
        println!("Debug info => {:?}", resp);
        for event in events {
            println!("event ({}) #{} at {} by {}...",
                event.event, event.id, event.created_at, event.actor.login);
        }
    }
}
