extern crate curl;
extern crate chrono;

extern crate rustc_serialize;

pub mod http;
pub mod activity;

use http::HttpClient;
use http::CurlClient;

static DEFAULT_BASE_URL: &'static str = "https://api.github.com/";
static UPLOAD_BASE_URL: &'static str = "https://uploads.github.com/";

pub static API_ACCEPT_HEADER: &'static str = "application/vnd.github.v3+json";

pub struct Client<C: HttpClient> {
    pub base_url: String,
    pub upload_url: String,
    pub http_client: C,

    user_agent: String,
}

impl Client<CurlClient> {
    pub fn new(user: &str) -> Client<CurlClient> {
        Client::new_custom(user, Some(CurlClient::new()))
    }
}

impl<C: HttpClient> Client<C> {
    pub fn new_custom(user: &str, client: Option<C>) -> Client<C> {
        Client {
            user_agent: user.to_string(),
            base_url: DEFAULT_BASE_URL.to_string(),
            upload_url: UPLOAD_BASE_URL.to_string(),
            http_client: client.unwrap_or_else(|| C::new()),
        }
    }

    pub fn hello(&self) {
        println!("Hello I am, {}.", self.user_agent);
    }
}