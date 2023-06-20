use std::{
    future::{ready, Ready},
    str::FromStr,
};

use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    http::header::{self, HeaderName, HeaderValue},
    Error,
};
use futures_util::future::LocalBoxFuture;
use uuid::Uuid;

use crate::constants::REQUEST_ID;

// There are two steps in middleware processing.
// 1. Middleware initialization, middleware factory gets called with
//    next service in chain as parameter.
// 2. Middleware's call method gets called with normal request.
pub struct RequestId;

// Middleware factory is `Transform` trait
// `S` - type of the next service
// `B` - type of response's body
impl<S, B> Transform<S, ServiceRequest> for RequestId
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = RequestIdMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(RequestIdMiddleware { service }))
    }
}

pub struct RequestIdMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for RequestIdMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, mut req: ServiceRequest) -> Self::Future {
        let req_headers = req.headers();
        let uuid_v4 = Uuid::new_v4().to_string();
        let header_value_optional = HeaderValue::from_str(&uuid_v4).unwrap();
        let request_id_information = match req_headers.get("x-request-id") {
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

        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?;

            Ok(res)
        })
    }
}
