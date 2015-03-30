use ::response::*;
use ::error::*;

use std::str;

use curl::http as curl_http;

use rustc_serialize::json;
use rustc_serialize::Decodable;

/// The `API_ACCEPT_HEADER` value is specified under the Accept header,
/// to enforce the use of the supported GitHub API, which is version 3.
static API_ACCEPT_HEADER: &'static str = "application/vnd.github.v3+json";

/// A simplistic function that wraps around the behaviour of an
/// http get-request as defined in `curl`.
/// As the library gets more complete, a more complete and complex
/// approach might be needed.
pub fn get<R: Decodable>(user: &str, url: &str, opts: Option<Vec<(&str, &str)>>) -> Result<(Vec<R>, Response), ClientError> {
    // Creating an empty request with header info needed for all requests.
    let mut handle = curl_http::handle();
    let mut request = handle.get(url)
        .header("User-Agent", user).header("Accept", API_ACCEPT_HEADER);

    // In case extre header options are needed,
    // it can be defined and given via the `opts` parameter.
    if opts.is_some() {
        for (name, val) in opts.unwrap() {
            request = request.header(name, val);
        }
    }

    // Executing the actual request via curl and storing the response.
    let response = request.exec().unwrap();
    // Retrieving the status code from the response object.
    let status_code = response.get_code();

    // Decoding the header and body in a controlled fashion,
    // throwing an error in case something went wrong internally,
    // replacing a panic, or when a response was negative.
    if check_status_code(status_code) {
        match str::from_utf8(response.get_body()) {
            Ok(raw_body) => {
                match json::decode(raw_body) {
                    Ok(b) => {
                        let body: Vec<R> = b;
                        return Ok((body, Response::populate(response.get_headers())));
                    }
                    Err(e) => {
                        return InternalError::new(&format!("{}", e));
                    }
                };
            }
            Err(e) => {
                return InternalError::new(&format!("{}", e));
            }
        }
    } else {
        return RequestError::new(status_code, response.get_body());
    }
}
