use Client;
use http::HttpClient;

pub fn list<C: HttpClient>(client: Client<C>) {
    client.http_client.get(
        &client.user_agent,
        &(client.base_url + "events"))
}