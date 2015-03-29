use rustc_serialize::json;
use rustc_serialize::Decoder;
use rustc_serialize::Decodable;

use std::str;
use std::fmt;

/// Documentation References:
/// https://developer.github.com/v3/#client-errors

/// `ErrorCode` represents the type of error that was reported
/// as a response on a request to th Github API.
#[derive(Debug)]
pub enum ErrorCode {
    /// This means a resource does not exist.
    Missing,
    /// This means a required field on a resource has not been set.
    MissingField,
    /// This means the formatting of a field is invalid.
    /// The documentation for that resource should be able
    /// to give you more specific information.
    Invalid,
    /// This means another resource has the same value as this field.
    /// This can happen in resources that must
    /// have some unique key (such as Label names).
    AlreadyExists,
    /// `Unknown(String)` is used as a last resort when an error code is unknown.
    /// This should never happen, please report/resolve the issue when it does happen.
    Unknown(String),
}

/// Allowing `ErrorCode` to be printed via `{}`.
impl fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg: String = String::from_str(match *self {
            ErrorCode::Missing => "resource does not exist",
            ErrorCode::MissingField => "required field on the resource has not been set",
            ErrorCode::Invalid => "the formatting of the field is invalid",
            ErrorCode::AlreadyExists => "another resource has the same value as this field",
            ErrorCode::Unknown(ref s) => &s,
        });

        write!(f, "{}", msg)
    }
}

/// Allowing `ErrorCode` to be decoded from json values.
/// Linked to the `error` key as defind by the `ErrorContext` struct's member.
impl Decodable for ErrorCode {
    fn decode<D: Decoder>(d: &mut D) -> Result<ErrorCode, D::Error> {
        match d.read_str() {
            Ok(code) => Ok(match &*code {
                "missing" => ErrorCode::Missing,
                "missing_field" => ErrorCode::MissingField,
                "invalid" => ErrorCode::Invalid,
                "already_exists" => ErrorCode::AlreadyExists,
                unknown => ErrorCode::Unknown(String::from_str(unknown))
            }),
            Err(err) => Err(err),
        }
    }
}

/// When a request was successful.
const STATUS_OK: u32 = 200;
/// There was a problem with the data sent with the request.
const STATUS_BAD_REQUEST: u32 = 400;
/// Given as a response to requests the user has insufficient permissions for.
const STATUS_FORBIDDEN: u32 = 403;
/// Given when a field or resource couldn't be processed properly.
const STATUS_UNPROCCESSABLE_ENTITY: u32 = 422;

/// When a negative status was given as a response to a request,
/// there might be one or several error descriptions embedded in the
/// body to tell more about the details of what was wrong.
/// `ErrorContext` is the representation for each of the errors that are given.
#[derive(RustcDecodable, Debug)]
pub struct ErrorContext {
    pub resource: String,
    pub field: String,
    pub code: ErrorCode,
}

/// Allowing `ErrorContext` to be printed via `{}` in a controlled manner.
impl fmt::Display for ErrorContext {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error found in {}.{}: {}", self.resource, self.field, self.code)
    }
}

/// `ErrorStatus` represents the status code given in the header of a negative response.
/// Look at const definitions such as `STATUS_OK` for more information for each value.
#[derive(Debug)]
pub enum ErrorStatus{
    BadRequest,
    UnprocessableEntity,
    Forbidden,
    Unknown(u32),
}

/// Allowing `ErrorStatus` to be printed via `{}` in a controlled manner.
impl fmt::Display for ErrorStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (code, msg) = match *self {
            ErrorStatus::BadRequest => (STATUS_BAD_REQUEST, "Bad Request"),
            ErrorStatus::UnprocessableEntity => (STATUS_UNPROCCESSABLE_ENTITY, "Unprocessable Entity"),
            ErrorStatus::Forbidden => (STATUS_FORBIDDEN, "Forbidden Request"),
            ErrorStatus::Unknown(e) => (e, "Unknown"),
        };

        write!(f, "status {}: {}", code, msg)
    }
}

impl ErrorStatus {
    /// Simple way to construct an `ErrorStatus`
    /// based on its constant value as defined by the official docs.
    pub fn new(code: u32) -> ErrorStatus {
        match code {
            STATUS_BAD_REQUEST => ErrorStatus::BadRequest,
            STATUS_FORBIDDEN => ErrorStatus::Forbidden,
            STATUS_UNPROCCESSABLE_ENTITY => ErrorStatus::UnprocessableEntity,
            unknown => ErrorStatus::Unknown(unknown),
        }
    }
}

/// `RequestError` will be returned as a `Result<T, ClientError>` in case
/// a request responds negatively populated by information from
/// both the header and body.
#[derive(Debug)]
pub struct RequestError {
    /// `code` represents the given status code
    /// stored in the form of `ErrorStatus`.
    pub code: ErrorStatus,
    /// In case detailed errors are available
    // they will be accessible via `errors`, stored as an `ErrorContext`.
    pub errors: Vec<ErrorContext>,
}

impl RequestError {
    /// Simple way to construct a `Result<T, ClientError>` based on
    /// the status code given in the header and the body in a raw utf8 buffer.
    pub fn new<T>(code: u32, buffer: &[u8]) -> Result<T, ClientError> {
        Err(ClientError::Http(RequestError {
            code: ErrorStatus::new(code),
            errors: match str::from_utf8(buffer) {
                Err(..) => Vec::new(),
                Ok(body) => json::decode(body).unwrap_or(Vec::new()),
            },
        }))
    }
}

/// Allowing `RequestError` to be printed via `{}` in a controlled manner.
impl fmt::Display for RequestError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "HTTP Error: {}. Found {} error description(s)!", self.code, self.errors.len())
    }
}

/// `InternalError` will be given in the form of Result<T, ClientError> in
/// case something went wrong within this Client Library.
/// It replaces panics so that you can freely choose the behaviour.
/// Please file an issue and/or resolve the bug yourself when you get this error.
#[derive(Debug)]
pub struct InternalError {
    /// `msg` is the actual description of the problem.
    /// future versions of this library might store extra info
    /// where it would help the debugging of an error.
    pub msg: String,
}

impl InternalError {
    /// Simple way to construct a `Result<T, ClientError>` based on
    /// information known for an internal error.
    pub fn new<T>(msg: &str) -> Result<T, ClientError> {
        Err(ClientError::Internal(InternalError { msg: String::from_str(msg) }))
    }
}

/// Allowing `InternalError` to be printed via `{}` in a controlled manner.
impl fmt::Display for InternalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Internal Error: {}", self.msg)
    }
}

/// `ClientError` enumerates all the possible errors that a public
/// client (request) function of this library might be given.
#[derive(Debug)]
pub enum ClientError {
    /// Read the documentation for `RequestError`
    /// for more information on this error.
    Http(RequestError),
    /// Read the documentation for `InternalError`
    /// for more information on this error..
    Internal(InternalError)
}

/// Allowing `ClientError` to be printed via `{}` in a controlled manner.
impl fmt::Display for ClientError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &ClientError::Http(ref e) => write!(f, "{}", e),
            &ClientError::Internal(ref e) => write!(f, "{}", e),
        }
    }
}

/// Simplistic function internally used to check
/// if a returned status code is positive.
/// Which means that the request was succesful.
pub fn check_status_code(code: u32) -> bool {
    code == STATUS_OK
}
