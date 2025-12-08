use crate::core::data_hold::data_struct::FunctionReturnType;
use boa_engine::{Context, JsString, JsValue};
use std::collections::HashMap;
use std::path::PathBuf;
use crate::core::boa_js_specific::js_convert_judge::{extract_string, is_array, js_value_to_string};

pub fn call_function_typed(
    context: &mut Context,
    func_name: &str,
    content: &str,
    path_alias: &HashMap<String, String>,
    current_path: &PathBuf,
) -> Result<FunctionReturnType, String> {
    let global = context.global_object();
    let func_key = JsString::from(func_name);

    // Get the function
    let func = global
        .get(func_key, context)
        .map_err(|e| format!("Failed to get function: {}", e))?;

    // Call the function
    let content_value = JsValue::from(JsString::from(content));
    let args = [content_value];

    let result = match func.as_callable() {
        Some(callable) => callable
            .call(&JsValue::undefined(), &args, context)
            .map_err(|e| format!("{}", e))?,
        None => return Err(format!("{} is not callable", func_name)),
    };

    // Determine return type
    classify_result(context, &result, path_alias, current_path)
}

/// Classify the JavaScript return value
fn classify_result(
    context: &mut Context,
    result: &JsValue,
    path_alias: &HashMap<String, String>,
    current_path: &PathBuf,
) -> Result<FunctionReturnType, String> {
    // Check if it's an array using Array.isArray()
    if is_array(context, result) {
        // Parse as 2D array
        parse_2d_array(context, result)
    } else if result.is_object() && !result.is_null() {
        // Parse as object
        parse_object(context, result)
    } else {
        // Convert everything else to string (JavaScript way)
        let value = js_value_to_string(context, result);
        Ok(FunctionReturnType::String(value))
    }
}

/// Parse 2D array: [[string, string], [string, string], ...] OR [{k:v}, {k:v}, ...] OR Mixed
fn parse_2d_array(context: &mut Context, array: &JsValue) -> Result<FunctionReturnType, String> {
    let array_obj = array.as_object().ok_or("Not an array object")?;

    // Get array length
    let length = array_obj
        .get(JsString::from("length"), context)
        .map_err(|e| format!("Failed to get length: {}", e))?
        .as_number()
        .ok_or("Length is not a number")? as usize;

    let mut result = Vec::new();

    for i in 0..length {
        let item = array_obj
            .get(JsString::from(i.to_string()), context)
            .map_err(|e| format!("Failed to get item {}: {}", i, e))?;

        // Check if item is an array with 2 elements
        if is_array(context, &item) {
            let item_obj = item.as_object().ok_or("Item is not an object")?;

            // Get [0] and [1]
            let key = item_obj
                .get(JsString::from("0"), context)
                .map_err(|e| format!("Failed to get key at index {}: {}", i, e))?;
            let value = item_obj
                .get(JsString::from("1"), context)
                .map_err(|e| format!("Failed to get value at index {}: {}", i, e))?;

            let key_str = extract_string(&key);
            let value_str = extract_string(&value);

            result.push((key_str, value_str));
        }
        // Check if item is an object
        else if item.is_object() && !item.is_null() {
            let item_obj = item.as_object().ok_or("Item is not an object")?;

            // Get all keys from the object
            let keys = item_obj
                .own_property_keys(context)
                .map_err(|e| format!("Failed to get keys at index {}: {}", i, e))?;

            // Process each key-value pair in the object
            for key in keys {
                let key_name = match &key {
                    boa_engine::property::PropertyKey::String(s) => s.to_std_string_escaped(),
                    boa_engine::property::PropertyKey::Index(idx) => idx.get().to_string(),
                    boa_engine::property::PropertyKey::Symbol(_) => continue,
                };

                let value = item_obj
                    .get(key, context)
                    .map_err(|e| format!("Failed to get value for key {}: {}", key_name, e))?;

                let value_str = extract_string(&value);
                result.push((key_name, value_str));
            }
        } else {
            return Err(format!("Item {} is neither an array nor an object", i));
        }
    }

    Ok(FunctionReturnType::Array2D(result))
}

/// Parse object into key-value pairs
fn parse_object(context: &mut Context, obj: &JsValue) -> Result<FunctionReturnType, String> {
    let obj_ref = obj.as_object().ok_or("Not an object")?;

    let keys = obj_ref
        .own_property_keys(context)
        .map_err(|e| format!("Failed to get keys: {}", e))?;

    let mut result = Vec::new();

    for key in keys {
        let key_name = match &key {
            boa_engine::property::PropertyKey::String(s) => s.to_std_string_escaped(),
            boa_engine::property::PropertyKey::Index(idx) => idx.get().to_string(),
            boa_engine::property::PropertyKey::Symbol(_) => continue,
        };

        let value = obj_ref
            .get(key, context)
            .map_err(|e| format!("Failed to get value: {}", e))?;

        let value_str = extract_string(&value);
        result.push((key_name, value_str));
    }

    Ok(FunctionReturnType::Object(result))
}

