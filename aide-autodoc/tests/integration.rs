use aide::axum::ApiRouter;
use aide::openapi::OpenApi;
use serde_json::Value;

/// First paragraph summary
///
/// Second paragraph description
#[aide_autodoc::aide_autodoc]
async fn handler_with_docs() -> &'static str {
    "hello"
}

/// Single line only
#[aide_autodoc::aide_autodoc]
async fn handler_single_line() -> &'static str {
    "ok"
}

#[aide_autodoc::aide_autodoc]
async fn handler_no_docs() -> &'static str {
    "no docs"
}

/// Multi line
/// continued
///
/// Description paragraph
/// also multi line
#[aide_autodoc::aide_autodoc]
async fn handler_multi() -> &'static str {
    "multi"
}

///   Spaced summary   
#[aide_autodoc::aide_autodoc]
async fn handler_whitespace_summary() -> &'static str {
    "ws"
}

fn build_openapi(router: ApiRouter) -> Value {
    let mut api = OpenApi::default();
    let _ = router.finish_api(&mut api);
    serde_json::to_value(&api).unwrap()
}

#[test]
fn test_summary_extracted_from_first_paragraph() {
    let router = ApiRouter::new().api_route(
        "/x",
        aide::axum::routing::get(handler_with_docs),
    );
    let json = build_openapi(router);

    let op = get_operation(&json, "/x", "get");
    assert_eq!(op["summary"].as_str().unwrap(), "First paragraph summary");
    assert!(op["description"]
        .as_str()
        .unwrap()
        .contains("Second paragraph description"));
}

#[test]
fn test_summary_only_no_description() {
    let router = ApiRouter::new().api_route(
        "/y",
        aide::axum::routing::get(handler_single_line),
    );
    let json = build_openapi(router);

    let op = get_operation(&json, "/y", "get");
    assert_eq!(op["summary"].as_str().unwrap(), "Single line only");
    let desc = op["description"].as_str().unwrap_or("");
    assert_eq!(desc, "");
}

#[test]
fn test_no_doc_comments() {
    let router = ApiRouter::new().api_route(
        "/z",
        aide::axum::routing::get(handler_no_docs),
    );
    let json = build_openapi(router);

    let op = get_operation(&json, "/z", "get");
    let summary = op["summary"].as_str().unwrap_or("");
    assert_eq!(summary, "");
    let desc = op["description"].as_str().unwrap_or("");
    assert_eq!(desc, "");
}

#[test]
fn test_multiline_description() {
    let router = ApiRouter::new().api_route(
        "/m",
        aide::axum::routing::get(handler_multi),
    );
    let json = build_openapi(router);

    let op = get_operation(&json, "/m", "get");
    assert_eq!(op["summary"].as_str().unwrap(), "Multi line continued");
    assert!(op["description"]
        .as_str()
        .unwrap()
        .contains("Description paragraph"));
}

#[test]
fn test_summary_trims_whitespace() {
    let router = ApiRouter::new().api_route(
        "/w",
        aide::axum::routing::get(handler_whitespace_summary),
    );
    let json = build_openapi(router);

    let op = get_operation(&json, "/w", "get");
    assert_eq!(op["summary"].as_str().unwrap(), "Spaced summary");
}

fn get_operation<'a>(json: &'a Value, path: &str, method: &str) -> &'a Value {
    &json["paths"][path][method]
}
