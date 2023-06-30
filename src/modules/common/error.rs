use serde::Serialize;

trait Custom {
    fn new(code: i32, message: &String) -> Self;
}

#[derive(utoipa::ToResponse, Serialize)]
pub struct BadRequest {
    code: i32,
    message: String,
}

impl Custom for BadRequest {
    fn new(code: i32, message: &String) -> BadRequest {
        return BadRequest {
            code,
            message: message.clone(),
        };
    }
}

#[derive(utoipa::ToResponse, Serialize)]
pub struct NotFound {
    code: i32,
    message: String,
}

impl Custom for NotFound {
    fn new(code: i32, message: &String) -> NotFound {
        return NotFound {
            code,
            message: message.clone(),
        };
    }
}

#[derive(utoipa::ToResponse, Serialize)]
pub struct InternalServerError {
    code: i32,
    message: String,
}

impl Custom for InternalServerError {
    fn new(code: i32, message: &String) -> InternalServerError {
        return InternalServerError {
            code,
            message: message.clone(),
        };
    }
}
