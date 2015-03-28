use curl::http;
use rustc_serialize::json;
use rustc_serialize::Decodable;

use std::str;
use std::collections::HashMap;

use API_ACCEPT_HEADER;

pub trait DecodableHeader {
    fn decode_header(&mut self, data: &HashMap<String, Vec<String>>);
}

pub trait HttpClient {
    fn new() -> Self;
    fn get<R: Decodable, H: DecodableHeader>(
        &self, user: &str, header: &mut H, url: &str) -> (Vec<R>, u32);
}

pub struct CurlClient;

impl HttpClient for CurlClient {
    fn new() -> CurlClient {
        CurlClient
    }

    // todo:
    // + return header decoded
    // + handle custom header parameters
    
    fn get<R: Decodable, H: DecodableHeader>(
        &self, user: &str, header: &mut H, url: &str) -> (Vec<R>, u32) {
        let response = http::handle()
            .get(url)
            .header("User-Agent", user)
            .header("Accept", API_ACCEPT_HEADER)
            .exec().unwrap();

        let raw_body = match str::from_utf8(response.get_body()) {
            Ok(b) => b,
            Err(e) => panic!("Error parsing response body: {:?}", e),
        };

        let body: Vec<R> = match json::decode(raw_body) {
            Ok(b) => b,
            Err(e) => panic!("Error parsing raw body: {:?}", e),
        };

        header.decode_header(response.get_headers());

        (body, response.get_code())
    }
}
