use crate::core::boa_js_specific::built_in_filter::is_builtin;
use crate::core::data_hold::data_struct::FunctionForExecute;
use crate::core::boa_js_specific::get_property::{get_property, has_property};
use boa_engine::property::PropertyKey;
use boa_engine::{Context, JsObject};

pub fn filter_functions_inside<F>(
    context: &mut Context,
    filter: F,
) -> Result<Vec<FunctionForExecute>, String>
where
    F: Fn(&JsObject, &mut Context, &mut FunctionForExecute) -> bool,
{
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
            };

            let can_push = filter(value_obj, context, &mut current_value);
            if !can_push {
                continue;
            }

            exported.push(current_value);
        }
    }

    Ok(exported)
}

pub fn find_exported_functions(context: &mut Context) -> Result<Vec<FunctionForExecute>, String> {
    Ok(filter_functions_inside(
        context,
        |value_obj, context, current_value| {
            let get_property = get_property(value_obj, "path", context);
            if !get_property.is_empty() {
                current_value.path = get_property.to_string();
                return true;
            }
            false
        },
    )?)
}

pub fn find_composed_functions(context: &mut Context) -> Result<Vec<FunctionForExecute>, String> {
    Ok(filter_functions_inside(
        context,
        |value_obj, context, current_value| {
            let get_property = has_property(value_obj, "compose", context);
            if get_property {
                current_value.path = get_property.to_string();
                return true;
            }
            false
        },
    )?)
}
