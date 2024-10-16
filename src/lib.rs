use bold::metadata_bold;
use brack_pdk_rs::metadata::Metadata;
use document::metadata_document;
use extism_pdk::{plugin_fn, FnResult, Json};
use stmt::metadata_stmt;

pub mod document;
pub mod bold;
pub mod stmt;

#[plugin_fn]
pub fn get_metadata() -> FnResult<Json<Vec<Metadata>>> {
    let mut metadata = Vec::new();
    metadata.push(metadata_bold());
    metadata.push(metadata_document());
    metadata.push(metadata_stmt());
    Ok(Json(metadata))
}
