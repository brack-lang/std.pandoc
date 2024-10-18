use bold::metadata_bold;
use brack_pdk_rs::metadata::Metadata;
use document::metadata_document;
use extism_pdk::{plugin_fn, FnResult, Json};
use headings::metadata_headings;
use stmt::metadata_stmt;
use text::metadata_text;

pub mod document;
pub mod bold;
pub mod stmt;
pub mod text;
pub mod headings;

#[plugin_fn]
pub fn get_metadata() -> FnResult<Json<Vec<Metadata>>> {
    let mut metadata = Vec::new();
    metadata.push(metadata_bold());
    metadata.push(metadata_document());
    metadata.push(metadata_stmt());
    metadata.push(metadata_text());
    for m in metadata_headings() {
        metadata.push(m);
    }
    Ok(Json(metadata))
}
