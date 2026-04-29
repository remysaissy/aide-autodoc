use axum::extract::Path;

/// Handler with existing extractor
#[aide_autodoc::aide_autodoc]
async fn handler_with_extractor(Path(id): Path<u32>) -> String {
    id.to_string()
}

fn main() {}
