use brack_pdk_rs::{metadata::Metadata, types::Type, values::Value};
use extism_pdk::{plugin_fn, FnResult, Json, WithReturnCode};

pub(crate) fn metadata_headings() -> Vec<Metadata> {
    let mut metadata = Vec::new();
    for i in 1..=6 {
        metadata.push(Metadata {
            command_name: "*".repeat(i),
            call_name: format!("headings_level{}", i),
            argument_types: vec![("text".to_string(), Type::TInline)],
            return_type: Type::TBlock,
        });
    }
    metadata
}

#[plugin_fn]
pub fn headings_level1(Json(args): Json<Vec<Value>>) -> FnResult<String> {
    if args.len() != 1 {
        return Err(WithReturnCode::new(
            anyhow::anyhow!("Usage: {{std.* text}}"),
            1,
        ));
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
    Ok(format!(
        "{{
    \"t\": \"Header\",
    \"c\": [1, [], [
        {}
    ]]
}},",
        text[0..text.len() - 1].to_string()
    ))
}

#[plugin_fn]
pub fn headings_level2(Json(args): Json<Vec<Value>>) -> FnResult<String> {
    if args.len() != 1 {
        return Err(WithReturnCode::new(
            anyhow::anyhow!("Usage: {{std.** text}}"),
            1,
        ));
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
    Ok(format!(
        "{{
    \"t\": \"Header\",
    \"c\": [2, [], [
        {}
    ]]
}},",
        text[0..text.len() - 1].to_string()
    ))
}

#[plugin_fn]
pub fn headings_level3(Json(args): Json<Vec<Value>>) -> FnResult<String> {
    if args.len() != 1 {
        return Err(WithReturnCode::new(
            anyhow::anyhow!("Usage: {{std.*** text}}"),
            1,
        ));
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
    Ok(format!(
        "{{
    \"t\": \"Header\",
    \"c\": [3, [], [
        {}
    ]]
}},",
        text[0..text.len() - 1].to_string()
    ))
}

#[plugin_fn]
pub fn headings_level4(Json(args): Json<Vec<Value>>) -> FnResult<String> {
    if args.len() != 1 {
        return Err(WithReturnCode::new(
            anyhow::anyhow!("Usage: {{std.**** text}}"),
            1,
        ));
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
    Ok(format!(
        "{{
    \"t\": \"Header\",
    \"c\": [4, [], [
        {}
    ]]
}},",
        text[0..text.len() - 1].to_string()
    ))
}

#[plugin_fn]
pub fn headings_level5(Json(args): Json<Vec<Value>>) -> FnResult<String> {
    if args.len() != 1 {
        return Err(WithReturnCode::new(
            anyhow::anyhow!("Usage: {{std.***** text}}"),
            1,
        ));
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
    Ok(format!(
        "{{
    \"t\": \"Header\",
    \"c\": [5, [], [
        {}
    ]]
}},",
        text[0..text.len() - 1].to_string()
    ))
}

#[plugin_fn]
pub fn headings_level6(Json(args): Json<Vec<Value>>) -> FnResult<String> {
    if args.len() != 1 {
        return Err(WithReturnCode::new(
            anyhow::anyhow!("Usage: {{std.****** text}}"),
            1,
        ));
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
    Ok(format!(
        "{{
    \"t\": \"Header\",
    \"c\": [6, [], [
        {}
    ]]
}},",
        text[0..text.len() - 1].to_string()
    ))
}
