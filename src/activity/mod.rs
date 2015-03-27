//use chrono::*;

pub mod events;

#[derive(RustcDecodable, RustcEncodable)]
pub struct Repository {
    pub id: u64,
    pub name: String,
    pub url: String,
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct Actor {
    pub id: u64,
    pub login: String,
    pub gravatar_id: String,
    pub avatar_url: String,
    pub url: String,
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct Organisation {
    pub id: u64,
    pub login: String,
    pub gravatar_id: String,
    pub avatar_url: String,
    pub url: String,
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct EventResponse  {
    pub public: bool,
    pub repo: Repository,
    pub actor: Actor,
    pub org: Option<Organisation>,
    // todo: replace with proper time
    pub created_at: String,
    pub id: String,
}