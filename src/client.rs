static DEFAULT_BASE_URL: &'static str = "https://api.github.com/";
static DEFAULT_UPLOAD_BASE_URL: &'static str = "https://uploads.github.com/";

pub struct Client {
    pub user_agent: String,
    pub base_url: String,
    pub upload_url: String,
}

impl Client {
    pub fn custom(user: &str, base_url: &str, upload_url: &str) -> Client {
        Client {
            user_agent: String::from_str(user),
            base_url: String::from_str(base_url),
            upload_url: String::from_str(upload_url),
        }
    }

    pub fn new(user: &str) -> Client {
        Client::custom(user, DEFAULT_BASE_URL, DEFAULT_UPLOAD_BASE_URL)
    }
}
