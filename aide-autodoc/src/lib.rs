//! # aide-autodoc
//!
//! Extract OpenAPI summary and description from doc comments for use with
//! [aide](https://crates.io/crates/aide) and [axum](https://crates.io/crates/axum).
//!
//! Compatible with axum 0.8+ and aide 0.15+.
//!
//! Forked from `aidecomment 0.1.1` on crates.io, via the internal weavly fork,
//! and updated for axum 0.8 (native async traits, no `axum::async_trait`) and
//! aide 0.15 (`aide::generate::GenContext` instead of `aide::gen::GenContext`).
//!
//! See the [README](https://github.com/remysaissy/aide-autodoc) for usage examples.

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{Expr, FnArg, Ident, ItemFn, Lit};

/// Extract summary and description from doc comments into an aide `OperationInput`.
///
/// The first paragraph of the doc comment becomes the OpenAPI summary.
/// Everything after the first blank line becomes the description.
///
/// ```ignore
/// /// This is a summary
/// ///
/// /// This is a longer description of the endpoint.
/// #[aide_autodoc]
/// async fn my_handler() -> &'static str {
///     "hello world"
/// }
/// ```
///
/// Requires `axum` (0.8+) and `aide` (0.15+) as dependencies.
#[proc_macro_attribute]
pub fn aide_autodoc(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut fn_def = syn::parse_macro_input!(item as ItemFn);

    let doc_comments = fn_def
        .attrs
        .iter()
        .filter_map(|attr| match &attr.meta {
            syn::Meta::NameValue(nvmeta) => Some(nvmeta),
            _ => None,
        })
        .filter(|nvmeta| nvmeta.path.get_ident().map(|i| i.to_string()) == Some("doc".to_owned()))
        .filter_map(|nvmeta| match &nvmeta.value {
            Expr::Lit(literal) => Some(literal),
            _ => None,
        })
        .filter_map(|literal| match &literal.lit {
            Lit::Str(string) => Some(string.value()),
            _ => None,
        })
        .collect::<Vec<_>>();

    let doc_comment = doc_comments.join("\n");
    let mut lines = doc_comment.lines().collect::<Vec<_>>();

    // separate summary from description
    let first_empty_idx = lines
        .iter()
        .position(|line| line.trim().is_empty())
        .unwrap_or(lines.len());

    let summary = lines.drain(0..first_empty_idx).collect::<Vec<_>>().join("");
    let summary = summary.trim();

    let description = lines.join("\n");
    let description = description.trim();

    let struct_name = fn_def.sig.ident.to_string() + "_AideComment";
    let struct_name = Ident::new(&struct_name, Span::mixed_site());

    let vis = fn_def.vis.clone();

    let arg = syn::parse_str::<FnArg>(&format!("_: {struct_name}")).unwrap();
    fn_def.sig.inputs.insert(0, arg);

    quote! {
        #vis struct #struct_name;

        impl ::aide::OperationInput for #struct_name {
            fn operation_input(_ctx: &mut ::aide::generate::GenContext, operation: &mut ::aide::openapi::Operation) {
                operation.summary = Some(#summary.to_owned());
                operation.description = Some(#description.to_owned());
            }
        }

        impl<S: Send + Sync> ::axum::extract::FromRequestParts<S> for #struct_name {
            type Rejection = ::std::convert::Infallible;
            async fn from_request_parts(
                _parts: &mut ::axum::http::request::Parts,
                _state: &S,
            ) -> Result<Self, Self::Rejection> {
                Ok(#struct_name)
            }
        }

        #fn_def
    }
    .into()
}
