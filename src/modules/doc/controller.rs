use crate::modules::{
    self,
    common::error::{BadRequest, InternalServerError, NotFound},
};
use actix_web::web;
use utoipa::OpenApi;
use utoipa_swagger_ui::{SwaggerUi, Url};

pub fn doc_scope_config(cfg: &mut web::ServiceConfig) {
    #[derive(OpenApi)]
    #[openapi(     servers(
                 (url = "http://localhost:8989", description = "Local server"),
                 (url = "http://api.{username}:{port}", description = "Remote API",
                     variables(
                         ("username" = (default = "demo", description = "Default username for API")),
                         ("port" = (default = "8080", enum_values("8080", "5000", "3030"), description = "Supported ports for API"))
                     )
                 )
    ))]
    #[openapi(components(schemas(BadRequest, NotFound, InternalServerError)))]
    #[openapi(paths(modules::health::controller::health))]
    struct ApiDoc;

    let swagger = SwaggerUi::new("/doc/{_:.*}").urls(vec![(
        Url::new("api", "/api-docs/openapi.json"),
        ApiDoc::openapi(),
    )]);

    cfg.service(swagger);
}
