use rustc_serialize::Decodable;
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

#[derive(RustcDecodable)]
pub struct ErrorContext {
    resource: String,
    field: String,
    code: String,
}

pub enum ErrorType {
    BadRequest,
    UnprocessableEntity,
    Forbidden,
    Unknown(u32),
}

pub struct RequestError {
    code: ErrorType,
    errors: Vec<ErrorContext>,
}

impl RequestError {
    pub fn new(code: u32, buffer: &[u8]) -> RequestError {
        RequestError {
            code: match code {
                400 => ErrorType::BadRequest,
                403 => ErrorType::Forbidden,
                422 => ErrorType::UnprocessableEntity,
                unknown => ErrorType::Unknown(unknown),
            },
            errors: match str::from_utf8(buffer) {
                Err(..) => Vec::new(),
                Ok(body) => json::decode(body).unwrap_or(Vec::new()),
            },
        }
    }

    pub fn get_error<T>(code: u32, buffer: &[u8]) -> Result<T, ClientError> {
        Err(ClientError::HttpError(RequestError::new(code, buffer)))
    }
}

pub enum ClientError {
    HttpError(RequestError),
    InternalError(String)
}

pub fn is_ok(code: u32) -> bool {
    code == 200
}

pub fn get_internal_error<T>(message: &str) -> Result<T, ClientError> {
    Err(ClientError::InternalError(String::from_str(message)))
}