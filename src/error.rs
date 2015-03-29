use rustc_serialize::json;

use std::str;
use std::fmt;

/// TODO: documentation
/// https://developer.github.com/v3/#client-errors
/*
pub enum ErrorCode {
    Missing,
    MissingField,
    Invalid,
    AlreadyExists,
    String,
}

impl<D: Decodable::Decoder> Decodable<D> for ErrorCode {
    pub fn decode(d: &mut json::Decoder) -> Result<ErrorCode, Error> {
        match d.read_str() {
            Ok(code) => match code {
                "missing" => Ok(ErrorCode::Missing),
                "missing_field" => Ok(ErrorCode::MissingField),
                "invalid" => Ok(ErrorCode::Invalid),
                "already_exists" => Ok(ErrorCode::AlreadyExists),
                unknown => Ok(unknown),
            },
            err => err,
        }
    }
}
*/

const STATUS_OK: u32 = 200;
const STATUS_BAD_REQUEST: u32 = 400;
const STATUS_FORBIDDEN: u32 = 403;
const STATUS_UNPROCCESSABLE_ENTITY: u32 = 422;

#[derive(RustcDecodable, Debug)]
pub struct ErrorContext {
    pub resource: String,
    pub field: String,
    pub code: String,
}

#[derive(Debug)]
pub enum ErrorStatus{
    BadRequest,
    UnprocessableEntity,
    Forbidden,
    Unknown(u32),
}

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
    pub fn new(code: u32) -> ErrorStatus {
        match code {
            STATUS_BAD_REQUEST => ErrorStatus::BadRequest,
            STATUS_FORBIDDEN => ErrorStatus::Forbidden,
            STATUS_UNPROCCESSABLE_ENTITY => ErrorStatus::UnprocessableEntity,
            unknown => ErrorStatus::Unknown(unknown),
        }
    }
}

#[derive(Debug)]
pub struct RequestError {
    pub code: ErrorStatus,
    pub errors: Vec<ErrorContext>,
}

impl RequestError {
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

impl fmt::Display for RequestError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       write!(f, "HTTP Error: {}. Found {} error(s)!", self.code, self.errors.len())
    }
}

#[derive(Debug)]
pub struct InternalError {
    pub msg: String,
}

impl InternalError {
    pub fn new<T>(msg: &str) -> Result<T, ClientError> {
        Err(ClientError::Internal(InternalError { msg: String::from_str(msg) }))
    }
}

impl fmt::Display for InternalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       write!(f, "Internal Error: {}", self.msg)
    }
}

#[derive(Debug)]
pub enum ClientError {
    Http(RequestError),
    Internal(InternalError)
}

pub fn check_status_code(code: u32) -> bool {
    code == STATUS_OK
}