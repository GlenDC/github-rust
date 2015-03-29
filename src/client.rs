/// By default and in most scenarios, `DEFAULT_BASE_URL`
/// will be the base url for requests via this Client library.
static DEFAULT_BASE_URL: &'static str = "https://api.github.com/";
/// By default and in most scenarios, `DEFAULT_BASE_URL`
/// will be the base upload url for requests via this Client library.
static DEFAULT_UPLOAD_BASE_URL: &'static str = "https://uploads.github.com/";

/// The `Client` struct represent the user agent and base URLs.
/// Functions in this library will never mutate a `Client` object
/// and for th sake of parallel processing, you should try to keep it immutable.
pub struct Client {
    /// `user_agent` represents the value given
    /// under the User-Agent key as part of
    /// the header of each request.
    pub user_agent: String,
    /// The base url for non-upload requests.
    pub base_url: String,
    /// The base url for upload requests.
    pub upload_url: String,
}

impl Client {
    /// Construct a `Client` for a custom domain, other than GitHub.
    pub fn custom(user: &str, base_url: &str, upload_url: &str) -> Client {
        Client {
            user_agent: String::from_str(user),
            base_url: String::from_str(base_url),
            upload_url: String::from_str(upload_url),
        }
    }

    /// Construct a `Client` using the default URLs as defined by GitHub.
    pub fn new(user: &str) -> Client {
        Client::custom(user, DEFAULT_BASE_URL, DEFAULT_UPLOAD_BASE_URL)
    }
}
