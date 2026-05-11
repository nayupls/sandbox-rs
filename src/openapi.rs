use utoipa::OpenApi;

use crate::{
    api,
    models::{
        ErrorBody, ErrorResponse, ExecuteRequest, ExecuteResponse, HealthResponse, LanguageInfo,
        LanguagesResponse,
    },
};

#[derive(OpenApi)]
#[openapi(
    info(
        title = "sandbox-rs API",
        version = env!("CARGO_PKG_VERSION"),
        description = "Execute mainstream programming-language snippets in sandboxed containers."
    ),
    paths(api::health, api::languages, api::execute),
    components(schemas(
        ErrorBody,
        ErrorResponse,
        ExecuteRequest,
        ExecuteResponse,
        HealthResponse,
        LanguageInfo,
        LanguagesResponse
    )),
    tags(
        (name = "system", description = "Service health and metadata"),
        (name = "execution", description = "Sandboxed code execution")
    )
)]
pub struct ApiDoc;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn documents_public_routes() {
        let doc = ApiDoc::openapi();

        assert!(doc.paths.paths.contains_key("/health"));
        assert!(doc.paths.paths.contains_key("/v1/languages"));
        assert!(doc.paths.paths.contains_key("/v1/execute"));
    }
}
