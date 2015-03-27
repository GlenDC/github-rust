use curl::http;
use rustc_serialize::json;
use rustc_serialize::Decodable;
use std::str;

use API_ACCEPT_HEADER;

pub trait HttpClient {
    fn new() -> Self;
    fn get<ResponseType: Decodable>(&self, user: &str, url: &str) -> (Vec<ResponseType>, u32);
}

pub struct CurlClient;

impl HttpClient for CurlClient {
    fn new() -> CurlClient {
        CurlClient
    }

    // todo:
    // + return header decoded
    // + handle custom header parameters
    
    fn get<ResponseType: Decodable>(&self, user: &str, url: &str) -> (Vec<ResponseType>, u32) {
        let response = http::handle()
            .get(url)
            .header("User-Agent", user)
            .header("Accept", API_ACCEPT_HEADER)
            .exec().unwrap();

        let raw_body = match str::from_utf8(response.get_body()) {
            Ok(b) => b,
            Err(e) => panic!("Error parsing response body: {:?}", e),
        };

        let body: Vec<ResponseType> = match json::decode(raw_body) {
            Ok(b) => b,
            Err(e) => panic!("Error parsing raw body: {:?}", e),
        };

        (body, response.get_code())
    }
}