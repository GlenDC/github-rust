use Client;
use http::HttpClient;

use activity::EventResponse;

pub fn list<C: HttpClient>(client: &mut Client<C>) -> (Vec<EventResponse>, u32) {
    client.http_client.get(
        & client.user_agent,
        &mut client.x_rate_limit,
        &format!("{}events", client.base_url))
}

pub fn list_repos<C: HttpClient>(client: &mut Client<C>, user: &str, repository: &str) -> (Vec<EventResponse>, u32) {
    client.http_client.get(
        & client.user_agent,
        &mut client.x_rate_limit,
        & format!("{}repos/{}/{}/events", client.base_url, user, repository))
}

pub fn list_repos_issue<C: HttpClient>(client: &mut Client<C>, user: &str, repository: &str) -> (Vec<EventResponse>, u32) {
    client.http_client.get(
        & client.user_agent,
        &mut client.x_rate_limit,
        & format!("{}repos/{}/{}/issues/events", client.base_url, user, repository))
}
