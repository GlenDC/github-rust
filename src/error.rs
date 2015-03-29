use rustc_serialize::json;
use rustc_serialize::Decoder;
use rustc_serialize::Decodable;

use std::str;
use std::fmt;

/// TODO: documentation
/// https://developer.github.com/v3/#client-errors

pub enum ErrorCode {
    Missing,
    MissingField,
    Invalid,
    AlreadyExists,
    Unknown(String),
}

impl fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg: String = String::from_str(match *self {
            ErrorCode::Missing => "resource does not exist",
            ErrorCode::MissingField => "required field on the resource has not been set",
            ErrorCode::Invalid => "the formatting of the field is invalid",
            ErrorCode::AlreadyExists => "another resource has the same value as this field",
            ErrorCode::Unknown(ref msg) => &msg,
        });

        write!(f, "{}", msg)
    }
}

impl Decodable for ErrorCode {
    fn decode<D: Decoder>(d: &mut D) -> Result<ErrorCode, D::Error> {
        match d.read_str() {
            Ok(code) => Ok(match &*code {
                "missing" => ErrorCode::Missing,
                "missing_field" => ErrorCode::MissingField,
                "invalid" => ErrorCode::Invalid,
                "already_exists" => ErrorCode::AlreadyExists,
                unknown => ErrorCode::Unknown(String::from_str(unknown)),
            }),
            Err(err) => Err(err),
        }
    }
}

const STATUS_OK: u32 = 200;
const STATUS_BAD_REQUEST: u32 = 400;
const STATUS_FORBIDDEN: u32 = 403;
const STATUS_UNPROCCESSABLE_ENTITY: u32 = 422;

#[derive(RustcDecodable)]
pub struct ErrorContext {
    pub resource: String,
    pub field: String,
    pub code: ErrorCode,
}

impl fmt::Display for ErrorContext {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error found in {}.{}: {}", self.resource, self.field, self.code)
    }
}

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
        write!(f, "HTTP Error: {}. Found {} error description(s)!", self.code, self.errors.len())
    }
}

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

pub enum ClientError {
    Http(RequestError),
    Internal(InternalError)
}

impl fmt::Display for ClientError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &ClientError::Http(ref e) => write!(f, "{}", e),
            &ClientError::Internal(ref e) => write!(f, "{}", e),
        }
    }
}

pub fn check_status_code(code: u32) -> bool {
    code == STATUS_OK
}