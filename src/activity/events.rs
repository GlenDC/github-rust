use Client;

use activity::EventReturnType;
use activity::IssueEventReturnType;

/// Documentation References:
/// https://developer.github.com/v3/activity/events/

/// List public events.
pub fn list_events(client: &Client) -> EventReturnType {
    ::http::get(
        &client.user_agent,
        &format!("{}events", client.base_url),
        None)
}

/// List repository events.
pub fn list_repo_events(client: &Client, user: &str, repo: &str) -> EventReturnType {
    ::http::get(
        &client.user_agent,
        &format!("{}repos/{}/{}/events", client.base_url, user, repo),
        None)
}

/// List repository events for a repository from
/// the user defined in `Client` as `user_agent`.
pub fn list_my_repo_events(client: &Client, repo: &str) -> EventReturnType {
    list_repo_events(client, &client.user_agent, repo)
}

/// List events that a user has received.
///
/// These are events that you’ve received by watching repos and following users.
/// If you are authenticated as the given user, you will see private events.
/// Otherwise, you’ll only see public events.
pub fn list_received_user_events(client: &Client, user: &str) -> EventReturnType {
    ::http::get(
        &client.user_agent,
        &format!("{}users/{}/received_events", client.base_url, user),
        None)
}

/// List events that the user, defined in `Client` as `user_agent`, received.
pub fn list_my_received_events(client: &Client) -> EventReturnType {
    list_received_user_events(client, &client.user_agent)
}

/// List public events that a user has received.
pub fn list_received_public_user_events(client: &Client, user: &str) -> EventReturnType {
    ::http::get(
        &client.user_agent,
        &format!("{}users/{}/received_events/public", client.base_url, user),
        None)
}

/// List public events that the user, defined in `Client` as `user_agent`, received.
pub fn list_my_received_public_events(client: &Client) -> EventReturnType {
    list_received_public_user_events(client, &client.user_agent)
}

/// List events performed by a user.
///
/// If you are authenticated as the given user, you will see your private events.
/// Otherwise, you’ll only see public events.
pub fn list_user_events(client: &Client, user: &str) -> EventReturnType {
    ::http::get(
        &client.user_agent,
        &format!("{}users/{}/events", client.base_url, user),
        None)
}

/// List events performed by the user defined in `Client` as `user_agent`.
pub fn list_my_events(client: &Client) -> EventReturnType {
    list_user_events(client, &client.user_agent)
}

/// List public events performed by a user.
pub fn list_public_user_events(client: &Client, user: &str) -> EventReturnType {
    ::http::get(
        &client.user_agent,
        &format!("{}users/{}/events/public", client.base_url, user),
        None)
}

/// List public events performed by
/// the user defined in the `Client` as `user_agent`.
pub fn list_my_public_user_events(client: &Client) -> EventReturnType {
    list_public_user_events(client, &client.user_agent)
}

/// List public events for a network of repositories.
pub fn list_public_network_repo_events(client: &Client, user: &str, repo: &str) -> EventReturnType {
    ::http::get(
        &client.user_agent,
        &format!("{}networks/{}/{}/events", client.base_url, user, repo),
        None)
}

/// List public events for a network of repositories from
/// the owner defined in the `Client` as `user_agent`.
pub fn list_my_public_network_repo_events(client: &Client, repo: &str) -> EventReturnType {
    list_public_network_repo_events(client, &client.user_agent, repo)
}

/// List public events for an organization.
pub fn list_public_organisation_events(client: &Client, org: &str) -> EventReturnType {
    ::http::get(
        &client.user_agent,
        &format!("{}orgs/{}/events", client.base_url, org),
        None)
}

/// List events for an organization.
///
/// This is the user’s organization dashboard.
/// You must be authenticated as the user to view this.
pub fn list_organisation_events(client: &Client, user: &str, org: &str) -> EventReturnType {
    ::http::get(
        &client.user_agent,
        &format!("{}users/{}/events/orgs/{}", client.base_url, user, org),
        None)
}

/// List events for an organization as
/// the user defined in `Client` as `user_agent`.
pub fn list_my_organisation_events(client: &Client, org: &str) -> EventReturnType {
    list_organisation_events(client, &client.user_agent, org)
}

/// List issue events for a repository.
///
/// Repository issue events have a different format than other events,
/// as documented by the GitHub Events API and represented by `IssueEventResponse`.
pub fn list_repo_issue_events(client: &Client, user: &str, repo: &str) -> IssueEventReturnType {
    ::http::get(
        &client.user_agent,
        &format!("{}repos/{}/{}/issues/events", client.base_url, user, repo),
        None)
}

/// List issue events for a repository owned by
/// the user defined in `Client` as `user_agent`.
pub fn list_my_repo_issue_events(client: &Client, repo: &str) -> IssueEventReturnType {
    list_repo_issue_events(client, &client.user_agent, repo)
}
