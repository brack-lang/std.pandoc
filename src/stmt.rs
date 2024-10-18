use brack_pdk_rs::{metadata::Metadata, types::Type, values::Value};
use extism_pdk::{plugin_fn, FnResult, Json, WithReturnCode};
use serde_json::Value as JsonValue;

pub(crate) fn metadata_stmt() -> Metadata {
    Metadata {
        command_name: "stmt".to_string(),
        call_name: "stmt".to_string(),
        argument_types: vec![("stmt".to_string(), Type::TBlock)],
        return_type: Type::TBlock,
    }
}

#[plugin_fn]
pub fn stmt(Json(args): Json<Vec<Value>>) -> FnResult<String> {
    if args.len() != 1 {
        return Err(WithReturnCode::new(anyhow::anyhow!("stmt failed"), 1));
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
    let inner: JsonValue = serde_json::from_str(&format!("[{}]", text[0..text.len()-1].to_string()))?;
    let inner_first = inner
        .get(0)
        .ok_or_else(|| anyhow::anyhow!("No first element"))?;
    match inner_first.get("t").and_then(|t| t.as_str()) {
        Some("Str") | Some("Emph") | Some("Underline") | Some("Strong") | Some("Strikeout")
        | Some("Superscript") | Some("Subscr") => Ok(format!(
            "{{
    \"t\": \"Para\",
    \"c\": [
        {}
    ]
}},",
            text[0..text.len() - 1].to_string()
        )),
        Some(_) => Ok(text[0..text.len() - 1].to_string()),
        None => Err(WithReturnCode::new(anyhow::anyhow!("No t field"), 1)),
    }
}
