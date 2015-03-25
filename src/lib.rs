#![feature(collections)]

extern crate hyper;

pub mod http;
pub mod activity;

use http::HttpClient;
use http::HyperClient;

static DEFAULT_BASE_URL: &'static str = "https://api.github.com/";
static UPLOAD_BASE_URL: &'static str = "https://uploads.github.com/";

pub struct Client<C: HttpClient> {
    pub user_agent: String,
    pub base_url: String,
    pub upload_url: String,
    pub http_client: C,
}

impl Client<HyperClient> {
    pub fn new_default(user: &str) -> Client<HyperClient> {
        Client::new(user, Some(HyperClient::new()))
    }
}

impl<C: HttpClient> Client<C> {
    pub fn new(user: &str, client: Option<C>) -> Client<C> {
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