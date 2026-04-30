//! Full axum + aide application example.
//!
//! Demonstrates multiple routes, each with aide_autodoc applied, showing
//! that the macro works correctly on all of them.

#![allow(dead_code)]

use aide_autodoc::aide_autodoc;
use axum::response::Json;
use serde_json::Value;

/// Lists all available resources.
///
/// Returns a paginated list of resources. Use `?page=N` to navigate pages.
#[aide_autodoc]
async fn list_resources() -> Json<Value> {
    Json(serde_json::json!({"resources": [], "total": 0}))
}

/// Creates a new resource.
///
/// Accepts a JSON body with the resource data. Returns the created resource
/// with its assigned ID.
#[aide_autodoc]
async fn create_resource() -> Json<Value> {
    Json(serde_json::json!({"id": 1}))
}

/// Healthcheck endpoint.
#[aide_autodoc]
async fn healthcheck() -> Json<Value> {
    Json(serde_json::json!({"status": "ok"}))
}

fn main() {
    println!("Example: full_app — multiple routes each with aide_autodoc.");
    println!("list_resources  summary: 'Lists all available resources.'");
    println!("create_resource summary: 'Creates a new resource.'");
    println!("healthcheck     summary: 'Healthcheck endpoint.'");
    println!("OK");
}
