use brack_pdk_rs::{metadata::Metadata, values::Value, types::Type};
use extism_pdk::{plugin_fn, FnResult, Json, WithReturnCode};

pub(crate) fn metadata_document() -> Metadata {
    Metadata {
        command_name: "document".to_string(),
        call_name: "document".to_string(),
        argument_types: vec![("document".to_string(), Type::TBlock)],
        return_type: Type::TBlock,
    }
}

#[plugin_fn]
pub fn document(Json(args): Json<Vec<Value>>) -> FnResult<String> {
    if args.len() != 1 {
        return Err(WithReturnCode::new(anyhow::anyhow!("document failed"), 1));
    }
    let text = match &args[0] {
        Value::Text(t) => t,
        _ => {
            return Err(WithReturnCode::new(
                anyhow::anyhow!("text must be Value::Text"),
                1,
            ))
        }
    };
    Ok(format!("{{
    \"pandoc-api-version\": [1,23,1],
    \"meta\": {{}},
    \"blocks\": [
        {}
    ]
}}", text[1..text.len()-1].to_string()))
}
