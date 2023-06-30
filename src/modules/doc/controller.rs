use crate::modules::{
    self,
    common::error::{BadRequest, InternalServerError, NotFound},
    health::controller::HealthSuccessResponse,
};
use actix_web::web;
use utoipa::{openapi, OpenApi};
use utoipa_swagger_ui::{SwaggerUi, Url};

pub fn doc_scope_config(cfg: &mut web::ServiceConfig) {
    #[derive(OpenApi)]
    #[openapi(components(responses(BadRequest, NotFound, InternalServerError)))]
    #[openapi(paths(modules::health::controller::health))]
    struct ApiDoc;

    let swagger = SwaggerUi::new("/doc/{_:.*}").urls(vec![(
        Url::new("api", "/api-docs/openapi.json"),
        ApiDoc::openapi(),
    )]);

    cfg.service(swagger);
}
