use bold::metadata_bold;
use brack_pdk_rs::metadata::Metadata;
use document::metadata_document;
use extism_pdk::{plugin_fn, FnResult, Json};
use headings::metadata_headings;
use image::metadata_image;
use stmt::metadata_stmt;
use text::metadata_text;

pub mod bold;
pub mod document;
pub mod headings;
pub mod image;
pub mod stmt;
pub mod text;

#[plugin_fn]
pub fn get_metadata() -> FnResult<Json<Vec<Metadata>>> {
    let mut metadata = Vec::new();
    metadata.push(metadata_bold());
    metadata.push(metadata_document());
    metadata.push(metadata_stmt());
    metadata.push(metadata_text());
    metadata.push(metadata_image());
    for m in metadata_headings() {
        metadata.push(m);
    }
    Ok(Json(metadata))
}
