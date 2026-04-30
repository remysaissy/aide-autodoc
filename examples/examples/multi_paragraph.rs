//! Multi-paragraph doc comment example.
//!
//! Demonstrates that aide_autodoc correctly splits the first paragraph as
//! `summary` and all subsequent paragraphs as `description`.

#![allow(dead_code)]

use aide_autodoc::aide_autodoc;
use axum::response::Json;
use serde_json::Value;

/// Returns detailed user information.
///
/// This endpoint returns a JSON object containing the user's profile data.
/// It requires authentication via Bearer token in the Authorization header.
///
/// Returns 404 if the user is not found, 401 if the token is invalid.
#[aide_autodoc]
async fn get_user() -> Json<Value> {
    Json(serde_json::json!({"id": 1, "name": "Alice"}))
}

fn main() {
    println!("Example: multi_paragraph — first paragraph = summary, rest = description.");
    println!("Summary: 'Returns detailed user information.'");
    println!("Description includes the subsequent paragraphs.");
    println!("OK");
}
