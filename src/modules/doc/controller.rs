use crate::modules;
use actix_web::web;
use utoipa::{openapi, OpenApi};
use utoipa_swagger_ui::{SwaggerUi, Url};

pub fn doc_scope_config(cfg: &mut web::ServiceConfig) {
    #[derive(OpenApi)]
    #[openapi(paths(modules::health::controller::healthcheck))]
    struct ApiDoc;

    let swagger = SwaggerUi::new("/doc/{_:.*}").urls(vec![(
        Url::new("api", "/api-docs/openapi.json"),
        ApiDoc::openapi(),
    )]);

    cfg.service(swagger);
}
