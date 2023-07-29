use serde::Serialize;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    Error,
};
use futures::future::ok;

use crate::modules::error::constant::INVALID_AUTHORIZATION_HEADER;
use crate::modules::error::custom::{CustomError, CustomErrorType};

#[derive(Debug, Serialize)]
struct ErrorResponse {
    code: i32,
    message: String,
}

#[doc(hidden)]
pub struct ValidateKeycloakService<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for ValidateKeycloakService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error>,
    S::Future: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        println!("#######Auth:");
        let auth_param = match req.headers().get("Authorization") {
            Some(value_param) => match value_param.to_str() {
                Ok(value) => value.to_string(),
                Err(_) => String::new(),
            },
            None => String::new(),
        };

        if auth_param.is_empty() {
            let custom_error = CustomError::from(INVALID_AUTHORIZATION_HEADER);
            return Box::pin(async { Err(custom_error.into()) });
        }

        // Restante da lógica...

        Box::pin(self.service.call(req))
    }
}

#[derive(Clone, Debug)]
pub struct ValidateKeycloak;

impl ValidateKeycloak {
    pub fn default() -> Self {
        Self {}
    }
}

impl<S, B> Transform<S, ServiceRequest> for ValidateKeycloak
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error> + 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Transform, Self::InitError>>>>;
    type Transform = ValidateKeycloakService<S>;
    type InitError = ();

    fn new_transform(&self, service: S) -> Self::Future {
        Box::pin(ok(ValidateKeycloakService { service }))
    }
}
