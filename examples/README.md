# Examples

This workspace member contains runnable examples for the `aide-autodoc` crate.

## `basic_usage`

Minimal handler with a single doc comment paragraph.

```sh
cargo run --example basic_usage -p aide-autodoc-examples
```

## `multi_paragraph`

Handler with a summary and a multi-paragraph description.

```sh
cargo run --example multi_paragraph -p aide-autodoc-examples
```

## `no_docs`

Handler with no doc comment (empty summary/description).

```sh
cargo run --example no_docs -p aide-autodoc-examples
```

## `full_app`

Complete axum + aide example with multiple routes.

```sh
cargo run --example full_app -p aide-autodoc-examples
```
