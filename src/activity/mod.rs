use error::*;
use response::Response;

/// Documentation References:
/// https://developer.github.com/v3/activity/

/// All Activity::Events have the same response format.
/// The following structs represent this info found as a json response.

/// `Repository` contains all info regarding a git repository.
#[derive(Debug, RustcDecodable)]
pub struct Repository {
    pub id: u64,
    pub name: String,
    pub url: String,
}

/// `Actor` contains all info on the user creating the event.
#[derive(Debug, RustcDecodable)]
pub struct Actor {
    pub id: u64,
    pub login: String,
    pub gravatar_id: String,
    pub avatar_url: String,
    pub url: String,
}

/// `Organisation` contains all info on the organisation related to the event.
#[derive(Debug, RustcDecodable)]
pub struct Organisation {
    pub id: u64,
    pub login: String,
    pub gravatar_id: String,
    pub avatar_url: String,
    pub url: String,
}

/// `EventResponse` represents the response for all
/// requests found in `activity::events` except the issue ones.
#[derive(Debug, RustcDecodable)]
pub struct EventResponse {
    pub public: bool,
    pub repo: Repository,
    pub actor: Actor,
    pub org: Option<Organisation>,
    // todo: replace with proper time
    pub created_at: String,
    pub id: String,
}

/// `EventReturnType` is the return type for most public event-requests. 
pub type EventReturnType = Result<(Vec<EventResponse>, Response), ClientError>;

pub mod events;
