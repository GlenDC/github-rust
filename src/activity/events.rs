use Client;
use http::HttpClient;

pub fn list<C: HttpClient>(client: &Client<C>) {
    client.http_client.get(
        &client.user_agent,
        &format!("{}events", client.base_url))
}

pub fn list_repos<C: HttpClient>(
    client: &Client<C>, user: &str, repository: &str) {
    client.http_client.get(
        &client.user_agent,
        &format!("{}repos/{}/{}/events", client.base_url, user, repository))
}

pub fn list_repos_issue<C: HttpClient>(
    client: &Client<C>, user: &str, repository: &str) {
    client.http_client.get(
        &client.user_agent,
        &format!("{}repos/{}/{}/issues/events", client.base_url, user, repository))
}