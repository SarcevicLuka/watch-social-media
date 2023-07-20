use actix_web::{HttpResponse, ResponseError};
use heck::ToLowerCamelCase;
use r2d2::Error as R2D2Error;
use std::{
    convert::From, error::Error as StdError, fmt,
};
use diesel::result::Error as DieselError;
use reqwest::Error as RequestError;
use validr::error::ValidationErrors;
use jsonwebtoken::errors::Error as JwtError;

#[derive(Debug)]
pub enum Error {
    R2D2Error(R2D2Error),
    NotFound,
    NotFoundWithCause(String),
    Diesel(DieselError),
    Request(String),
    JsonDataNotFound(String),
    PasswordHashing,
    Validation(ValidationErrors),
    Jwt,
    Unauthorized(String),
    Forbidden(String),
    Connection,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Error::R2D2Error(ref cause) => write!(f, "R2D2_DB pool error:{}", cause),
            Error::NotFound => write!(f, "Not found"),
            Error::NotFoundWithCause(ref cause) => write!(f, "Not found: {}", cause),
            Error::Diesel(ref cause) => write!(f, "DB Error: {}", cause),
            Error::Request(ref cause) => write!(f, "Request error: {}", cause),
            Error::JsonDataNotFound(ref cause) => write!(f, "Data not found in Json: {}", cause),
            Error::PasswordHashing => write!(f, "Error while hashing password"),
            Error::Validation(ref cause) => {write!(f, "Validation error: {}", cause)},
            Error::Jwt => write!(f, "Jwt verification error"),
            Error::Unauthorized(ref cause) => write!(f, "Authorization error: {}", cause),
            Error::Forbidden(ref cause) => write!(f, "Forbidden error: {}", cause),
            Error::Connection => write!(f, "Cannot establish connection with database"),
        }
    }
}

impl StdError for Error {
    fn cause(&self) -> Option<&dyn StdError> {
        match *self {
            Error::R2D2Error(ref cause) => Some(cause),
            Error::NotFound => None,
            Error::NotFoundWithCause(ref _cause) => None,
            Error::Diesel(ref cause) => Some(cause),
            Error::Request(ref _cause) => None,
            Error::JsonDataNotFound(ref _cause) => None,
            Error::PasswordHashing => None,
            Error::Validation(ref cause) => Some(cause),
            Error::Jwt => None,
            Error::Unauthorized(ref _cause) => None,
            Error::Forbidden(ref _cause) => None,
            Error::Connection => None,
        }
    }
}

impl From<R2D2Error> for Error {
    fn from(cause: R2D2Error) -> Self {
        Error::R2D2Error(cause)
    }
}

impl From<DieselError> for Error {
    fn from(cause: DieselError) -> Self {
        Error::Diesel(cause)
    }
}

impl From<RequestError> for Error {
    fn from(cause: RequestError) -> Self {
        let url = match cause.url() {
            Some(url) => url.as_str(),
            None => "n/a",
        };
        let source = match cause.source() {
            Some(error) => error.to_string(),
            None => "n/a".to_string(),
        };
        let error = format!("Error: {} - {}", url, source);
        Error::Request(error)
    }
}

impl From<ValidationErrors> for Error {
    fn from(cause: ValidationErrors) -> Self {
        Error::Validation(cause)
    }
}

impl From<JwtError> for Error {
    fn from(_cause: JwtError) -> Self {
        Error::Jwt
    }
}

impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        if let Error::Validation(ref cause) = *self {
            return validation_errors_to_camel_case(cause).error_response();
        }

        let mut response = match self {
            Error::Jwt | Error::Unauthorized(_) => {
                HttpResponse::Unauthorized()
            },
            Error::NotFound => HttpResponse::NotFound(),
            Error::NotFoundWithCause(_) => HttpResponse::NotFound(),
            Error::Request(_) => HttpResponse::BadRequest(),
            Error::Forbidden(_) => HttpResponse::Forbidden(),
            _ => HttpResponse::InternalServerError(),
        };

        response.json(self.into_error_body())
    }
}

impl Error {
    pub fn add_cause_if_not_found(self, cause: &str) -> Error {
        match &self {
            Error::NotFound => Error::NotFoundWithCause(cause.to_string()),
            Error::NotFoundWithCause(_) => Error::NotFoundWithCause(cause.to_string()),
            _ => self,
        }
    }

    pub fn is_not_found(&self) -> bool {
        matches!(self, Error::NotFound | Error::NotFoundWithCause(_))
    }

    pub fn is_validation(&self) -> bool {
        matches!(self, Error::Validation(_))
    }

    pub fn into_error_body(&self) -> ErrorBody {
        match *self {
            Error::R2D2Error(ref cause) => ErrorBody {
                message: Some("R2D2_DB Pool Error".to_string()),
                code: "no_connections_available".to_string(),
                cause: Some(cause.to_string()),
                payload: None,
            },
            Error::NotFound => ErrorBody {
                message: Some("Entity not found".to_string()),
                code: "not_found".to_string(),
                cause: None,
                payload: None,
            },
            Error::NotFoundWithCause(ref cause) => ErrorBody {
                message: Some("Entity not found".to_string()),
                code: "not_found".to_string(),
                cause: Some(cause.to_string()),
                payload: None,
            },
            Error::Diesel(ref cause) => ErrorBody { 
                message: Some("DB Error: {}".to_string()), 
                code: "db".to_string(), 
                cause: Some(cause.to_string()), 
                payload: None, 
            },
            Error::Request(ref cause) => ErrorBody { 
                message: Some("Request error".to_string()), 
                code: "broken_outbound_request".to_string(), 
                cause: Some(cause.to_string()), 
                payload: None, 
            },
            Error::JsonDataNotFound(ref cause) => ErrorBody { 
                message: Some("Data not found in json".to_string()), 
                code: "data not found".to_string(), 
                cause: Some(cause.to_string()), 
                payload: None, 
            },
            Error::PasswordHashing => ErrorBody { 
                message: Some("Something went wrong while hashing password".to_string()), 
                code: "server_error".to_string(), 
                cause: None, 
                payload: None,
            },
            Error::Jwt => ErrorBody {
                message: Some("JWT authorization error".to_string()),
                code: "jwt".to_string(),
                cause: None,
                payload: None,
            },
            Error::Unauthorized(ref cause) => ErrorBody { 
                message: Some("Unauthorized error".to_string()), 
                code: "unauthorized".to_string(), 
                cause: Some(cause.to_string()), 
                payload: None, 
            },
            Error::Forbidden(ref cause) => ErrorBody {
                message: Some("Forbidden error".to_string()),
                code: "forbidden".to_string(),
                cause: Some(cause.to_string()),
                payload: None,
            },
            _ => ErrorBody {
                message: Some("Something went wrong".to_string()),
                code: "server_error".to_string(),
                cause: None,
                payload: None,
            },
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct ErrorBody {
    pub message: Option<String>,
    pub code: String,
    pub cause: Option<String>,
    pub payload: Option<serde_json::Value>,
}

/// Fix the validation errors to match the field names used on frontend
pub fn validation_errors_to_camel_case(errors: &ValidationErrors) -> ValidationErrors {
    let mut new_errors = ValidationErrors::new();

    for (pascal_case_field, e) in errors.get_errors().iter() {
        let mut error = e.clone();
        error.set_field_name(&pascal_case_field.to_lower_camel_case());
        new_errors.add(error);
    }

    new_errors
}

pub fn handle_vector_validataion_errors(
    field: &str,
    errors: Vec<Option<ValidationErrors>>,
) -> ValidationErrors {
    let mut new_error = ValidationErrors::new();

    for (index, error) in errors.iter().cloned().enumerate() {
        if let Some(error) = error {
            let internal_errors = error.get_errors();
            for (f, e) in internal_errors.iter() {
                let mut new_e = e.clone();
                new_e.set_field_name(&format!("{}.{}.{}", field, index, f));
                new_error.add(new_e);
            }
        }
    }

    new_error
}

#[allow(dead_code)]
pub fn compile_validation_errors(errors: Vec<Option<ValidationErrors>>) -> Error {
    let mut new_error = ValidationErrors::new();

    for error in errors.iter().flatten() {
        let internal_errors = error.get_errors();
        for (f, e) in internal_errors.iter() {
            let mut new_e = e.clone();
            new_e.set_field_name(f);
            new_error.add(new_e);
        }
    }

    Error::Validation(new_error)
}