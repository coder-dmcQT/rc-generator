use crate::core::call_func::call_function;
use crate::core::data_struct::FunctionResult;
use crate::core::find_exported_functions::find_exported_functions;
use boa_engine::{Context, Source};

/// Execute all functions prefixed with "export_" using content as parameter
pub fn execute_exported_functions(
    js_code: &str,
    content: &str,
) -> Result<Vec<FunctionResult>, String> {
    // Create JavaScript context
    let mut context = Context::default();

    // Evaluate the JavaScript code
    context
        .eval(Source::from_bytes(js_code))
        .map_err(|e| format!("Failed to evaluate JavaScript: {}", e))?;

    // Find all functions with "export_" prefix
    let exported_functions = find_exported_functions(&mut context)?;

    if exported_functions.is_empty() {
        return Ok(Vec::new());
    }

    // Call each exported function with the content
    let mut results = Vec::new();
    for func_item in exported_functions {
        let function_name = func_item.function_name;
        let target_path = if func_item.path.is_empty() {
            "."
        } else {
            func_item.path.as_str()
        };
        let result = call_function(&mut context, &function_name, content);
        results.push(FunctionResult {
            name: function_name,
            result,
            path: target_path.to_string(),
        });
    }

    Ok(results)
}
