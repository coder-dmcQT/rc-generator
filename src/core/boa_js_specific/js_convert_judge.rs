use boa_engine::{Context, JsString, JsValue};

/// Check if a JsValue is an array using Array.isArray()
pub(crate) fn is_array(context: &mut Context, value: &JsValue) -> bool {
    // Use JavaScript's Array.isArray() function
    let global = context.global_object();

    // Get Array constructor
    let array_constructor = match global.get(JsString::from("Array"), context) {
        Ok(arr) => arr,
        Err(_) => return false,
    };

    // Get Array.isArray function
    let is_array_fn = match array_constructor.as_object() {
        Some(obj) => match obj.get(JsString::from("isArray"), context) {
            Ok(func) => func,
            Err(_) => return false,
        },
        None => return false,
    };

    // Call Array.isArray(value)
    match is_array_fn.as_callable() {
        Some(callable) => {
            let result = callable.call(&JsValue::undefined(), &[value.clone()], context);
            match result {
                Ok(res) => res.as_boolean().unwrap_or(false),
                Err(_) => false,
            }
        }
        None => false,
    }
}

/// Convert JsValue to String the JavaScript way (using toString())
pub(crate) fn js_value_to_string(value: &JsValue) -> String {
    // For strings, extract without quotes
    if value.is_string() {
        return value
            .as_string()
            .map(|s| s.to_std_string_escaped())
            .unwrap_or_default();
    }

    // For null
    if value.is_null() {
        return "null".to_string();
    }

    // For undefined
    if value.is_undefined() {
        return "undefined".to_string();
    }

    // For booleans: "true" or "false"
    if value.is_boolean() {
        return value.as_boolean().unwrap().to_string();
    }

    // For numbers: convert directly
    if value.is_number() {
        let num = value.as_number().unwrap();
        // Handle special cases
        if num.is_nan() {
            return "NaN".to_string();
        }
        if num.is_infinite() {
            return if num.is_sign_positive() {
                "Infinity".to_string()
            } else {
                "-Infinity".to_string()
            };
        }
        return num.to_string();
    }

    // Fallback for anything else
    value.display().to_string()
}


/// Helper to extract string from JsValue
pub(crate) fn extract_string(value: &JsValue) -> String {
    if value.is_string() {
        value
            .as_string()
            .map(|s| s.to_std_string_escaped())
            .unwrap_or_default()
    } else {
        value.display().to_string()
    }
}