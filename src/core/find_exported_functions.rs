use crate::core::built_in_filter::is_builtin;
use crate::core::data_struct::FunctionForExecute;
use boa_engine::property::PropertyKey;
use boa_engine::{Context, JsObject, JsString, JsValue};

pub fn find_exported_functions(context: &mut Context) -> Result<Vec<FunctionForExecute>, String> {
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

            let mut current_value = FunctionForExecute {
                path: "".to_string(),
                function_name: name,
                lang: "".to_string(),
            };

            let has_property = get_property(value_obj, "path", context);
            if !has_property.is_empty() {
                current_value.path = has_property
                    .to_string()
            }

            let has_lang = get_property(value_obj, "lang", context);
            if !has_lang.is_empty() {
                current_value.lang = has_lang
            }

            exported.push(current_value);
        }
    }

    Ok(exported)
}

fn get_property(object: &JsObject, name: &str, context: &mut Context) -> String {
    let property = object
        .get(PropertyKey::String(JsString::from(name)), context)
        .map_err(|e| format!("Failed to get property: {}", e)).unwrap();
    let value = if property.is_string() {
        property
    } else {
        JsValue::from(JsString::from(""))
    };
    value.to_string(context).unwrap().to_std_string_escaped()
}
