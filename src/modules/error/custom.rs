use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use serde::Serialize;

#[derive(utoipa::ToSchema, Serialize)]
pub struct BadRequest {
    /// Property represented custom code error
    #[schema(example = 4000, default = 0)]
    code: i32,
    /// Property represented custom message error
    #[schema(example = "Bad request error in flow", default = "")]
    message: String,

    status_code: u16,
}

#[derive(Serialize, utoipa::ToSchema)]
#[schema(example = json!(NotFound{code: 4001, message: "Resource not found".to_string(), status_code:0}))]
pub struct NotFound {
    /// Property represented custom code error
    #[schema(example = 4004, default = 0)]
    code: i32,
    /// Property represented custom message error
    #[schema(example = "Route not found", default = "")]
    message: String,

    status_code: u16,
}

#[derive(utoipa::ToSchema, Serialize)]
pub struct InternalServerError {
    /// Property represented custom code error
    #[schema(example = 5000, default = 0)]
    code: i32,
    /// Property represented custom message error
    #[schema(example = "Internal Server Error verify logs", default = "")]
    message: String,

    status_code: u16,
}
// impl Custom for BadRequest {
//     fn new(code: i32, message: &String, status_code: u16) -> BadRequest {
//         return BadRequest {
//             code,
//             message: message.clone(),
//             status_code: StatusCode::BAD_REQUEST.as_u16(),
//         };
//     }

//     fn origin() -> Self {
//         Self {
//             code: 0,
//             message: String::new(),
//             status_code: StatusCode::BAD_REQUEST.as_u16(),
//         }
//     }

//     fn get_status_code(&self) -> u16 {
//         return self.status_code.clone();
//     }

//     fn get_code(&self) -> i32 {
//         return self.code.clone();
//     }

//     fn get_message(&self) -> String {
//         return self.message.clone();
//     }
// }

// impl Custom for NotFound {
//     fn new(code: i32, message: &String, status_code: u16) -> NotFound {
//         return NotFound {
//             code,
//             message: message.clone(),
//             status_code: StatusCode::NOT_FOUND.as_u16(),
//         };
//     }

//     fn origin() -> Self {
//         Self {
//             code: 0,
//             message: String::new(),
//             status_code: StatusCode::NOT_FOUND.as_u16(),
//         }
//     }

//     fn get_status_code(&self) -> u16 {
//         return self.status_code.clone();
//     }

//     fn get_code(&self) -> i32 {
//         return self.code.clone();
//     }

//     fn get_message(&self) -> String {
//         return self.message.clone();
//     }
// }

// impl Custom for InternalServerError {
//     fn new(code: i32, message: &String, status_code: u16) -> InternalServerError {
//         return InternalServerError {
//             code,
//             message: message.clone(),
//             status_code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
//         };
//     }

//     fn origin() -> Self {
//         Self {
//             code: 0,
//             message: String::new(),
//             status_code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
//         }
//     }

//     fn get_status_code(&self) -> u16 {
//         return self.status_code.clone();
//     }

//     fn get_code(&self) -> i32 {
//         return self.code.clone();
//     }

//     fn get_message(&self) -> String {
//         return self.message.clone();
//     }
// }

// #[derive(Debug)]
// pub enum AppErrorType {
//     DbError,
//     NotFoundError,
// }

// impl fmt::Display for AppErrorType {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "{:?}", self)
//     }
// }

#[derive(Debug)]
pub enum CustomErrorType {
    DbError,
}

#[derive(Debug)]
pub struct Custom {
    pub message: Option<String>,
    pub err_type: CustomErrorType,
    // pub code: Option<u16>,
    // pub status_code: Option<u16>,
}

impl Custom {
    pub fn message(&self) -> String {
        match &self.message {
            Some(c) => c.clone(),
            None => String::from(""),
        }
    }
}

impl std::fmt::Display for Custom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

// impl From<diesel::result::Error> for Custom {
//     fn from(err: diesel::result::Error) -> Custom {
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

impl From<String> for Custom {
    fn from(
        err: String,
        //code: Option<u16>,
        //status_code: Option<u16>
    ) -> Custom {
        Custom {
            message: Some(err),
            err_type: CustomErrorType::DbError,
            // code: match code {
            //     Ok(code_param_value) => code_param_value,
            //     _ => 0,
            // },
            // status_code: match status_code {
            //     Ok(status_code_value) => status_code_value,
            //     _ => 0,
            // },
        }
    }
}

impl ResponseError for Custom {
    fn status_code(&self) -> StatusCode {
        match self.err_type {
            CustomErrorType::DbError => StatusCode::INTERNAL_SERVER_ERROR,
            // CustomErrorType::ValidationError => StatusCode::BAD_REQUEST,
            // CustomErrorType::UserError => StatusCode::INTERNAL_SERVER_ERROR,
            // CustomErrorType::R2D2Error => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(self.message.clone())
    }
}
