use brack_pdk_rs::{metadata::Metadata, types::Type, values::Value};
use extism_pdk::{plugin_fn, FnResult, Json, WithReturnCode};

pub(crate) fn metadata_image() -> Metadata {
    Metadata {
        command_name: "img".to_string(),
        call_name: "image".to_string(),
        argument_types: vec![
            ("src".to_string(), Type::TInline),
            ("alt".to_string(), Type::TInline),
            ("caption".to_string(), Type::TInline),
        ],
        return_type: Type::TBlock,
    }
}

#[plugin_fn]
pub fn image(Json(args): Json<Vec<Value>>) -> FnResult<String> {
    if args.len() != 3 {
        return Err(WithReturnCode::new(
            anyhow::anyhow!("{{std.image src, alt, caption}}"),
            1,
        ));
    }
    let src = match &args[0] {
        Value::Text(t) => t,
        _ => {
            return Err(WithReturnCode::new(
                anyhow::anyhow!("src must be Value::Text"),
                1,
            ))
        }
    };
    let alt = match &args[1] {
        Value::Text(t) => t,
        _ => {
            return Err(WithReturnCode::new(
                anyhow::anyhow!("alt must be Value::Text"),
                1,
            ))
        }
    };
    let caption = match &args[2] {
        Value::Text(t) => t,
        _ => {
            return Err(WithReturnCode::new(
                anyhow::anyhow!("caption must be Value::Text"),
                1,
            ))
        }
    };

    Ok(format!(
        "{{
    \"t\": \"Image\",
    \"c\": [
        [
            \"\",
            [],
            [],
        ],
        [
            {{ \"t\": \"Str\", \"c\": {} }}
        ],
        [
            \"{}\",
            \"{}\",
        ],
    ]
}},",
        alt[0..alt.len() - 1].to_string(),
        src[0..src.len() - 1].to_string(),
        caption[0..caption.len() - 1].to_string(),
    ))
}
