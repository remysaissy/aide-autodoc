//! No-doc-comment example.
//!
//! Demonstrates that aide_autodoc handles a function with no doc comments
//! gracefully — summary and description are both empty strings.

#![allow(dead_code)]

use aide_autodoc::aide_autodoc;
use axum::response::Json;
use serde_json::Value;

#[aide_autodoc]
async fn undocumented() -> Json<Value> {
    Json(serde_json::json!({}))
}

fn main() {
    println!("Example: no_docs — aide_autodoc on a function with no doc comment.");
    println!("Summary and description will both be empty strings.");
    println!("OK");
}
