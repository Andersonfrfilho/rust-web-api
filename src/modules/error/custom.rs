use std::io::Split;

use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use derive_more::{Display, Error};
use serde::Serialize;
use utoipa::openapi::Array;
use validator::ValidationErrors;

use super::constant::{INVALID_AUTHORIZATION_HEADER, INVALID_PAYLOAD};

#[derive(utoipa::ToSchema, Serialize)]

pub struct ErroContent {
    /// Property represented custom code error
    #[schema(example = 4000, default = 0)]
    code: u16,
    /// Property represented custom message error
    #[schema(example = "Bad request error in flow", default = "")]
    message: &'static str,

    /// Property represented custom message error
    #[schema(example = "Bad request error in flow", default = "")]
    contents: Option<Vec<Content>>,
}

#[derive(utoipa::ToSchema, Serialize)]
pub struct BadRequest {
    /// Property represented custom code error
    #[schema(example = 4000, default = 0)]
    code: i32,
    /// Property represented custom message error
    #[schema(example = "Bad request error in flow", default = "")]
    message: String,
}

#[derive(Serialize, utoipa::ToSchema)]
#[schema(example = json!(NotFound{code: 4001, message: "Resource not found".to_string() }))]
pub struct NotFound {
    /// Property represented custom code error
    #[schema(example = 4004, default = 0)]
    code: i32,
    /// Property represented custom message error
    #[schema(example = "Route not found", default = "")]
    message: String,
}

#[derive(utoipa::ToSchema, Serialize)]
pub struct InternalServerError {
    /// Property represented custom code error
    #[schema(example = 5000, default = 0)]
    code: i32,
    /// Property represented custom message error
    #[schema(example = "Internal Server Error verify logs", default = "")]
    message: String,
}

#[derive(Debug, Serialize)]
pub enum CustomErrorType {
    DieselError,
    ValidationError,
    InvalidAuthorizationHeader,
    UserError(String),
}

#[derive(Debug, Serialize, Error)]
pub struct Content {
    field: Option<String>,
    message: Option<String>,
}

impl Content {
    pub fn new(field: String, message: String) -> Self {
        Content {
            field: Some(field),
            message: Some(message),
        }
    }
}
#[derive(Debug, Serialize, Error)]
pub struct CustomError {
    pub message: &'static str,
    pub err_type: CustomErrorType,
    pub code: u16,
    pub contents: Option<Vec<Content>>,
}

impl CustomError {
    pub fn message(self) -> &'static str {
        self.message
    }

    pub fn code(&self) -> u16 {
        self.code.clone()
    }

    pub fn body_error(&self) -> ErroContent {
        let contents = match &self.contents {
            Some(contents) => contents
                .iter()
                .map(|content| Content {
                    field: content.field.clone(),
                    message: content.message.clone(),
                })
                .collect(),
            None => Vec::new(),
        };
        ErroContent {
            code: self.code,
            message: self.message,
            contents: Some(contents),
        }
    }
}

impl std::fmt::Display for Content {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::fmt::Display for CustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

// impl From<diesel::result::Error> for CustomError {
//     fn from(err: diesel::result::Error) -> CustomError {
//         CustomError {
//             message: Some(err.to_string()),
//             err_type: CustomErrorType::DieselError,
//         }
//     }
// }

// impl From<diesel::result::Error> for CustomError {
//     fn from(err: diesel::result::Error) -> CustomError {
//         CustomError {
//             message: Some(err.to_string()),
//             err_type: CustomErrorType::DieselError,
//         }
//     }
// }
//implementador de erro do parse do header bearer
impl From<actix_web_httpauth::headers::authorization::ParseError> for CustomError {
    fn from(err: actix_web_httpauth::headers::authorization::ParseError) -> CustomError {
        CustomError {
            code: INVALID_AUTHORIZATION_HEADER.0,
            message: INVALID_AUTHORIZATION_HEADER.1,
            err_type: INVALID_AUTHORIZATION_HEADER.2,
            contents: None,
        }
    }
}
// Implement erro de validação de path, body e query
impl From<validator::ValidationErrors> for CustomError {
    fn from(err: validator::ValidationErrors) -> CustomError {
        let errors_string = err.to_string();
        let errors_content: Vec<&str> = errors_string.split_terminator('\n').collect();
        let mut contents: Vec<Content> = Vec::new();
        for error_content in errors_content {
            let error_content: Vec<&str> = error_content.splitn(2, ':').collect();
            if error_content.len() == 2 {
                let field = error_content[0].trim().to_string();
                let message = error_content[1].trim().to_string();
                let content = Content {
                    field: Some(field),
                    message: Some(message),
                };
                contents.push(content);
            }
        }

        CustomError {
            code: INVALID_PAYLOAD.0,
            message: INVALID_PAYLOAD.1,
            err_type: INVALID_PAYLOAD.2,
            contents: Some(contents),
        }
    }
}
// Implement erro do Custom error
impl From<(u16, &'static str, CustomErrorType)> for CustomError {
    fn from(err: (u16, &'static str, CustomErrorType)) -> CustomError {
        CustomError {
            code: err.0,
            message: err.1,
            err_type: err.2,
            contents: None,
        }
    }
}
// Implement de resposta de erro de validação
impl ResponseError for CustomError {
    fn status_code(&self) -> StatusCode {
        match self.err_type {
            CustomErrorType::DieselError => StatusCode::INTERNAL_SERVER_ERROR,
            CustomErrorType::ValidationError => StatusCode::BAD_REQUEST,
            CustomErrorType::InvalidAuthorizationHeader => StatusCode::UNAUTHORIZED,
            // CustomErrorType::R2D2Error => StatusCode::INTERNAL_SERVER_ERROR,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(self.body_error())
    }
}
