use boa_engine::{Context, JsValue};

pub fn call_function(context: &mut Context, func_name: &str, content: &str) -> Result<String, String> {
    use boa_engine::JsString;

    let global = context.global_object();
    let func_key = JsString::from(func_name);

    let func = global
        .get(func_key, context)
        .map_err(|e| format!("Failed to get function: {}", e))?;

    let content_value = JsValue::from(JsString::from(content));
    let args = [content_value];

    match func.as_callable() {
        Some(callable) => match callable.call(&JsValue::undefined(), &args, context) {
            Ok(result) => {
                // Properly convert result based on type
                let output = match result {
                    // String: extract without quotes
                    _ if result.is_string() => {
                        result.as_string()
                            .map(|s| s.to_std_string_escaped())
                            .unwrap_or_else(|| result.display().to_string())
                    }
                    // Number: convert to string
                    _ if result.is_number() => {
                        result.as_number().unwrap().to_string()
                    }
                    // Boolean: convert to string
                    _ if result.is_boolean() => {
                        result.as_boolean().unwrap().to_string()
                    }
                    // Null/Undefined: convert to string
                    _ if result.is_null() => "null".to_string(),
                    _ if result.is_undefined() => "undefined".to_string(),
                    // Objects/Arrays: use display
                    _ => result.display().to_string(),
                };
                Ok(output)
            },
            Err(e) => Err(format!("{}", e)),
        },
        None => Err(format!("{} is not callable", func_name)),
    }
}