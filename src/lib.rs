#![feature(collections)]

extern crate curl;
extern crate chrono;
extern crate rustc_serialize;

pub mod http;
pub mod activity;

use http::HttpClient;
use http::CurlClient;
use http::DecodableHeader;

use std::str;
use std::collections::HashMap;
use std::default::Default;

static DEFAULT_BASE_URL: &'static str = "https://api.github.com/";
static UPLOAD_BASE_URL: &'static str = "https://uploads.github.com/";

pub static API_ACCEPT_HEADER: &'static str = "application/vnd.github.v3+json";

#[derive(Default)]
pub struct XRateLimit {
    limit: u32,
    remaining: u32,
    reset: String, // todo decode properly
}

impl XRateLimit {
    pub fn new() -> XRateLimit {
        XRateLimit { ..Default::default() }
    }

    pub fn get_limit(&self) -> u32 {
        self.limit
    }

    pub fn get_remaining(&self) -> u32 {
        self.remaining
    }

    pub fn get_reset(&self) -> &str {
        &self.reset
    }
}

impl DecodableHeader for XRateLimit {
    fn decode_header(&mut self, data: &HashMap<String, Vec<String>>) {
        self.limit = str::parse(&data["x-ratelimit-limit"][0])
            .ok().expect("Couldn't parse x-ratelimit-limit from header");
        self.remaining = str::parse(&data["x-ratelimit-remaining"][0])
            .ok().expect("Couldn't parse x-ratelimit-remaining from header");
        self.reset = String::from_str(&data["x-ratelimit-reset"][0])
    }
}

pub struct Client<C: HttpClient> {
    pub base_url: String,
    pub upload_url: String,
    pub http_client: C,
    pub x_rate_limit: XRateLimit,

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
            x_rate_limit: XRateLimit::new(),
        }
    }

    pub fn hello(&self) {
        println!("Hello I am, {}.", self.user_agent);
    }

    pub fn get_user_agent(&self) -> &str {
        &self.user_agent
    }
}
