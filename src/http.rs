use hyper::Client;
use hyper::header::UserAgent;

pub trait HttpClient {
    fn new() -> Self;
    fn get(&self, user: &str, url: &str);
}

pub struct HyperClient;

impl HttpClient for HyperClient {
    fn new() -> HyperClient {
        HyperClient
    }

    // todo:
    // + check if we want to replace url with a proper object that
    //   handles invalid urls, etc;
    // + support (header)option list;
    
    fn get(&self, user: &str, url: &str) {
        let mut client = Client::new();

        let res = match client.get(url).header(UserAgent(String::from_str(user))).send() {
            Ok(res) => res,
            Err(err) => panic!("Failed to connect: {:?}", err)
        };

        println!("Response: {}", res.status);
        println!("Headers:\n{}", res.headers);
    }
}