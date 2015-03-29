use ::response::*;
use ::error::*;

use std::str;

use curl::http as curl_http;

use rustc_serialize::json;
use rustc_serialize::Decodable;

static API_ACCEPT_HEADER: &'static str = "application/vnd.github.v3+json";

pub fn get<R: Decodable>(user: &str, url: &str, opts: Option<Vec<(&str, &str)>>) -> Result<(Vec<R>, Response), ClientError> {
    let mut handle = curl_http::handle();
    let mut request = handle.get(url);
    request = request.header("User-Agent", user)
        .header("Accept", API_ACCEPT_HEADER);

    if opts.is_some() {
        for (name, val) in opts.unwrap() {
            request = request.header(name, val);
        }
    }

    let response = request.exec().unwrap();
    let status_code = response.get_code();

    if check_status_code(status_code) {
        match str::from_utf8(response.get_body()) {
            Ok(raw_body) => {
                match json::decode(raw_body) {
                    Ok(b) => {
                        let body: Vec<R> = b;
                        return Ok((body, Response::populate(response.get_headers())));
                    }
                    Err(e) => {
                        return get_internal_error(&format!("InternalError: {}", e));
                    }
                };
            }
            Err(e) => {
                return get_internal_error(&format!("InternalError: {}", e));
            }
        }
    } else {
        return RequestError::new(status_code, response.get_body());
    }
}
