use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use derive_more::{Display, Error};
use serde::Serialize;

#[derive(utoipa::ToSchema, Serialize)]

pub struct ErroContent {
    /// Property represented custom code error
    #[schema(example = 4000, default = 0)]
    code: u16,
    /// Property represented custom message error
    #[schema(example = "Bad request error in flow", default = "")]
    message: &'static str,
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
    UserError(String),
}

#[derive(Debug, Serialize, Error)]
pub struct CustomError {
    pub message: &'static str,
    pub err_type: CustomErrorType,
    pub code: u16,
}

impl CustomError {
    pub fn message(self) -> &'static str {
        self.message
    }

    pub fn code(&self) -> u16 {
        self.code.clone()
    }

    pub fn body_error(&self) -> ErroContent {
        ErroContent {
            code: self.code,
            message: self.message,
        }
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

// impl From<validator::ValidationErrors> for CustomError {
//     fn from(err: validator::ValidationErrors) -> CustomError {
//         CustomError {
//             message: Some(err.to_string()),
//             err_type: CustomErrorType::ValidationError,
//         }
//     }
// }

impl From<(u16, &'static str, CustomErrorType)> for CustomError {
    fn from(err: (u16, &'static str, CustomErrorType)) -> CustomError {
        CustomError {
            code: err.0,
            message: err.1,
            err_type: err.2,
        }
    }
}

impl ResponseError for CustomError {
    fn status_code(&self) -> StatusCode {
        match self.err_type {
            CustomErrorType::DieselError => StatusCode::INTERNAL_SERVER_ERROR,
            CustomErrorType::ValidationError => StatusCode::BAD_REQUEST,
            // CustomErrorType::UserError => StatusCode::INTERNAL_SERVER_ERROR,
            // CustomErrorType::R2D2Error => StatusCode::INTERNAL_SERVER_ERROR,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(self.body_error())
    }
}
