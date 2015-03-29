use rustc_serialize::json;

use std::str;

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

#[derive(RustcDecodable)]
#[allow(dead_code)]
pub struct ErrorContext {
    pub resource: String,
    pub field: String,
    pub code: String,
}

pub enum ErrorStatus{
    BadRequest,
    UnprocessableEntity,
    Forbidden,
    Unknown(u32),
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

#[allow(dead_code)]
pub struct RequestError {
    pub code: ErrorStatus,
    pub errors: Vec<ErrorContext>,
}

impl RequestError {
    pub fn new<T>(code: u32, buffer: &[u8]) -> Result<T, ClientError> {
        Err(ClientError::HttpError(RequestError {
            code: ErrorStatus::new(code),
            errors: match str::from_utf8(buffer) {
                Err(..) => Vec::new(),
                Ok(body) => json::decode(body).unwrap_or(Vec::new()),
            },
        }))
    }
}

pub enum ClientError {
    HttpError(RequestError),
    InternalError(String)
}

pub fn check_status_code(code: u32) -> bool {
    code == STATUS_OK
}

pub fn get_internal_error<T>(message: &str) -> Result<T, ClientError> {
    Err(ClientError::InternalError(String::from_str(message)))
}