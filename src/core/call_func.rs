use boa_engine::{Context, JsValue};

pub fn call_function(context: &mut Context, func_name: &str, content: &str) -> Result<String, String> {
    use boa_engine::JsString;

    let global = context.global_object();

    // Convert function name to JsString for property key
    let func_key = JsString::from(func_name);

    // Get the function
    let func = global
        .get(func_key, context)
        .map_err(|e| format!("Failed to get function: {}", e))?;

    // Convert content to JsString and then to JsValue
    let content_value = JsValue::from(JsString::from(content));
    let args = [content_value];

    // Call the function
    match func.as_callable() {
        Some(callable) => match callable.call(&JsValue::undefined(), &args, context) {
            Ok(result) => Ok(result.display().to_string()),
            Err(e) => Err(format!("{}", e)),
        },
        None => Err(format!("{} is not callable", func_name)),
    }
}