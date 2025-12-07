use boa_engine::property::PropertyKey;
use boa_engine::{Context, JsString};
use crate::core::built_in_filter::is_builtin;

pub fn find_exported_functions(context: &mut Context) -> Result<Vec<String>, String> {
    let global = context.global_object();
    let mut exported = Vec::new();

    let keys = global
        .own_property_keys(context)
        .map_err(|e| format!("Failed to get property keys: {}", e))?;

    for key in keys {
        // Convert PropertyKey to string
        let name = match &key {
            PropertyKey::String(s) => s.to_std_string_escaped(),
            PropertyKey::Symbol(_) => continue,
            PropertyKey::Index(idx) => idx.get().to_string(),
        };

        // Skip built-in functions
        if is_builtin(&name) {
            continue;
        }

        let value = global
            .get(key, context)
            .map_err(|e| format!("Failed to get value: {}", e))?;

        if value.is_callable() {
            let value_obj = value.as_object().unwrap();

            let has_property = value_obj
                .has_own_property(PropertyKey::String(JsString::from("path")), context)
                .map_err(|e| format!("Failed to get property: {}", e))?;
            if has_property {
                println!("has path props!");
            }

            exported.push(name);
        }
    }

    Ok(exported)
}


