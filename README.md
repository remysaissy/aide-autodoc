# aide-autodoc

[![CI](https://github.com/remysaissy/aide-autodoc/actions/workflows/ci.yml/badge.svg)](https://github.com/remysaissy/aide-autodoc/actions/workflows/ci.yml)
[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)
[![Crates.io](https://img.shields.io/crates/v/aide-autodoc.svg)](https://crates.io/crates/aide-autodoc)

A procedural macro for automatically deriving [aide](https://crates.io/crates/aide) API documentation from Rust doc comments.

`aide-autodoc` extracts documentation from your handler functions and injects it into the [aide](https://docs.rs/aide) OpenAPI documentation generation framework. It reads the `#[doc]` attributes, using the first paragraph as the `summary` and the remaining paragraphs as the `description`.

This eliminates the need to maintain documentation in two places. Your Rust doc comments serve as the source of truth for both your code documentation and your OpenAPI metadata.

This crate was forked from [`aidecomment 0.1.1`](https://crates.io/crates/aidecomment) on crates.io.

## Features

- Automatically extracts the first doc comment paragraph as the OpenAPI `summary`
- Automatically extracts remaining doc comment paragraphs as the OpenAPI `description`
- Works with [axum](https://crates.io/crates/axum) handlers via `aide`'s `axum` feature
- Zero runtime overhead, all processing happens at compile time
- Handles missing doc comments gracefully with empty summary and description

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
aide-autodoc = "0.2"
aide = { version = "0.15", features = ["axum"] }
axum = "0.8"
```

Or using `cargo add`:

```shell
cargo add aide-autodoc
cargo add aide --features axum
cargo add axum
```

## Quick Start

Apply the `#[aide_autodoc]` attribute to your axum handler:

```rust
use aide_autodoc::aide_autodoc;
use axum::response::Json;
use serde_json::Value;

/// Returns a greeting for the given user.
///
/// This endpoint looks up the user by ID and returns a personalised greeting.
/// Returns 404 if the user is not found.
#[aide_autodoc]
async fn get_greeting() -> Json<Value> {
    Json(serde_json::json!({"message": "Hello, world!"}))
}
```

The macro sets the OpenAPI `summary` to `"Returns a greeting for the given user."` and the `description` to `"This endpoint looks up the user by ID and returns a personalised greeting.\nReturns 404 if the user is not found."`.

## Examples

Runnable examples are in the [`examples/`](examples/) directory:

- [`basic_usage`](examples/basic_usage.rs) — minimal handler with a single doc comment paragraph
- [`multi_paragraph`](examples/multi_paragraph.rs) — handler with summary and multi-paragraph description
- [`no_docs`](examples/no_docs.rs) — handler with no doc comment (empty summary/description)
- [`full_app`](examples/full_app.rs) — complete axum + aide application with multiple routes

Run an example:

```shell
cargo run --example basic_usage
```

## How It Works

The `#[aide_autodoc]` attribute macro reads `#[doc]` attributes from the annotated function. It splits the text on blank lines to separate paragraphs. The first paragraph becomes the `summary`. All other paragraphs are concatenated to form the `description`. The macro then generates an `impl aide::OperationOutput` block that returns this metadata to the `aide` framework.

## Development

See [CONTRIBUTING.md](CONTRIBUTING.md) for how to build, test, and contribute.

```shell
cargo build
cargo test
./test.sh          # full test suite with coverage check
./format.sh        # format + clippy
```

## License

Licensed under the [Apache License, Version 2.0](LICENSE).

This crate was forked from [`aidecomment 0.1.1`](https://crates.io/crates/aidecomment) on crates.io.
