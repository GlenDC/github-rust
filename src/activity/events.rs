use Client;
use activity::*;

pub fn list_events(client: &Client) -> EventReturnType {
    ::http::get(
        &client.user_agent,
        &format!("{}events", client.base_url))
}

pub fn list_repo_events(client: &Client, user: &str, repository: &str) -> EventReturnType {
    ::http::get(
        &client.user_agent,
        &format!("{}repos/{}/{}/events", client.base_url, user, repository))
}