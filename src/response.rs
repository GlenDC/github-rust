use std::collections::HashMap;
use std::default::Default;
use std::str::FromStr;

/// `HttpHeaderType` defines the type used for raw http headers
pub type HttpHeaderType = HashMap<String, Vec<String>>;

/// `Populatable` provides functionality to construct
/// an object based on info found in the http response header
pub trait Populatable {
    /// `populate` constructs a new object based on the
    /// information found in the http response header
    fn populate(raw_header: &HttpHeaderType) -> Self;
}

/// `Rate` represents the X-Rate-Limit data
/// provided by the Github v3 API and provided for each response
#[derive(Debug)]
pub struct Rate {
    /// the maximum limit of requests
    pub limit: u32,
    /// remaining requests possible
    pub remaining: u32,
    /// the date when this limit resets
    /// TODO: replace with proper DateTime<T>
    pub reset: String
}

/// `Page` represents a link related to the response
#[derive(Debug)]
pub struct Page {
    /// the actual page number
    pub number: u64,
}

/// TODO: implement functionality

/// `Response` represents the exposed data given with each
/// request and populated by the Github v3 API
#[derive(Debug)]
pub struct Response {
    /// the raw response header
    pub resp: HttpHeaderType,
    /// the immediate next page of result
    pub next: Option<Page>,
    /// the last page of results
    pub last: Option<Page>,
    /// the first page of results
    pub first: Option<Page>,
    /// the immediate previous page of results
    pub prev: Option<Page>,
    /// the latest X-Rate-Limit info
    pub rate: Rate,
}

/// Get a single raw header value for type `T`
/// using its default value when str::parse failed
fn get_single_header_value<T>(raw_data: &HttpHeaderType, key: &str) -> T where T: Default + FromStr {
    match str::parse(&raw_data[key][0]) {
        Ok(x) => x,
        Err(..) => Default::default(),
    }
}

impl Populatable for Rate {
    /// `populate` a `Rate<T>` object from the HTTP response header
    fn populate(raw_header: &HttpHeaderType) -> Rate {
        Rate {
            limit: get_single_header_value(raw_header, "x-ratelimit-limit"),
            remaining: get_single_header_value(raw_header, "x-ratelimit-remaining"),
            reset: raw_header["x-ratelimit-reset"][0].clone(),
        }
    }
}

impl Populatable for Response {
    /// `populate` a `Response<T>` object from the HTTP response header
    /// TODO: populate pages properly
    fn populate(raw_header: &HttpHeaderType) -> Response {
        Response {
            next: None,
            last: None,
            first: None,
            prev: None,
            rate: Rate::populate(raw_header),
            resp: raw_header.clone(),
        }
    }
}
