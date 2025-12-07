use boa_engine::{Context, Source};
use crate::core::call_func::call_function;
use crate::core::data_struct::FunctionResult;
use crate::core::find_exported_functions::find_exported_functions;

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
    for func_name in exported_functions {
        let result = call_function(&mut context, &func_name, content);
        results.push(FunctionResult {
            name: func_name,
            result,
        });
    }

    Ok(results)
}