#![deny(warnings)]

extern crate github;

use github::Client;
use github::error::*;
use github::activity::events::*;

fn main() {
    let client = &Client::new("glendc");
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
                    println!("    {}", error);;
                }
            }
        }
    }
}
