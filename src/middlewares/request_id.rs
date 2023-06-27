use std::{
    future::{ready, Ready},
    str::FromStr,
    task::{Context, Poll},
};

use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    http::header::{HeaderName, HeaderValue},
    Error,
};
use uuid::Uuid;

use crate::constants::REQUEST_ID;

#[doc(hidden)]
pub struct RequestIdService<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for RequestIdService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error>,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = S::Future;

    fn poll_ready(&self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, mut req: ServiceRequest) -> Self::Future {
        let req_headers = req.headers();
        let uuid_v4 = Uuid::new_v4().to_string();
        let header_value_optional = HeaderValue::from_str(&uuid_v4).unwrap();
        let request_id_information = match req_headers.get(REQUEST_ID) {
            Some(request_id) => request_id,
            _ => &header_value_optional,
        };
        let request_id_info_string = match request_id_information.to_str() {
            Ok(request_id_string) => request_id_string,
            _ => &uuid_v4,
        };
        let header_name = HeaderName::from_str(&REQUEST_ID).unwrap();
        let header_value = HeaderValue::from_str(&request_id_info_string).unwrap();
        req.headers_mut().insert(header_name, header_value);
        self.service.call(req)
    }
}

#[derive(Clone, Debug)]
pub struct RequestId {}

impl RequestId {
    pub fn default() -> Self {
        Self {}
    }
}

impl<S, B> Transform<S, ServiceRequest> for RequestId
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error>,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;
    type Transform = RequestIdService<S>;
    type InitError = ();

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(RequestIdService { service }))
    }
}
