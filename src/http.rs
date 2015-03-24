pub trait HttpClient {
    fn new() -> Self;
}

pub struct HyperClient;

impl HttpClient for HyperClient {
    fn new() -> HyperClient {
        HyperClient
    }
}