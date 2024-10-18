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
    let inner: JsonValue = serde_json::from_str(text)?;
    if let Some(t_value) = inner.get("t") {
        if let Some(t_str) = t_value.as_str() {
            match t_str {
                "Str" | "Emph" | "Underline" | "Strong" | "Strikeout" | "Superscript"
                | "Subscr" => {
                    return Ok(format!(
                        "{{
    \"t\": \"Para\",
    \"c\": [
        {}
    ]
}},",
                        text[0..text.len() - 1].to_string()
                    ));
                }
                _ => {
                    return Ok(text[0..text.len() - 1].to_string());
                }
            }
        } else {
            return Err(WithReturnCode::new(
                anyhow::anyhow!("invalid inner t field {}", t_value),
                1,
            ));
        }
    }
    Err(WithReturnCode::new(
        anyhow::anyhow!("missing inner t field"),
        1,
    ))
}
