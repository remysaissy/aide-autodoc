//! # aide-autodoc
//!
//! Extract OpenAPI summary and description from doc comments for use with
//! [aide](https://crates.io/crates/aide) and [axum](https://crates.io/crates/axum).
//!
//! Compatible with axum 0.8+ and aide 0.15+.
//!
//! Forked from `aidecomment 0.1.1` on crates.io and updated for axum 0.8
//! (native async traits, no `axum::async_trait`) and aide 0.15
//! (`aide::generate::GenContext` instead of `aide::gen::GenContext`).
//!
//! See the [README](https://github.com/remysaissy/aide-autodoc) for usage examples.

use proc_macro::TokenStream;

/// Extract summary and description from doc comments into an aide `OperationInput`.
///
/// **Note**: skeleton — implementation lands in the next PR.
#[proc_macro_attribute]
pub fn aide_autodoc(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // Skeleton no-op: return the function unchanged so the crate compiles
    // and downstream PRs can author tests against the stable public API.
    item
}
