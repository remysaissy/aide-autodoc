//! Basic usage of aide_autodoc.
//!
//! Demonstrates the simplest possible use case: a single-paragraph doc comment
//! whose first (and only) paragraph becomes the OpenAPI summary.

#![allow(dead_code)]

use aide_autodoc::aide_autodoc;
use axum::response::Json;
use serde_json::Value;

/// Returns a greeting message.
#[aide_autodoc]
async fn hello() -> Json<Value> {
    Json(serde_json::json!({"message": "Hello, world!"}))
}

fn main() {
    println!("Example: basic_usage — aide_autodoc applied to a single-paragraph doc comment.");
    println!("Summary would be: 'Returns a greeting message.'");
    println!("OK");
}
