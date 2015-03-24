#![feature(collections)]

extern crate hyper;

pub mod http;

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

impl<C: HttpClient> Client<C> {
    pub fn new(user: &str, client: Option<C>) -> Client<C> {
        Client {
            user_agent: String::from_str(user),
            base_url: String::from_str(DEFAULT_BASE_URL),
            upload_url: String::from_str(UPLOAD_BASE_URL),
            http_client: client.unwrap_or_else(|| C::new()),
        }
    }
}