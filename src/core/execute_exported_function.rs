use std::path::{Path, PathBuf};
use crate::core::call_func::call_function;
use crate::core::data_struct::FunctionResult;
use crate::core::find_exported_functions::find_exported_functions;
use boa_engine::{Context, Source};

/// Execute all functions prefixed with "export_" using content as parameter
pub fn execute_exported_functions(
    js_code: &str,
    content: &str,
    js_file_str: &str,
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

    // Store the temporary parent dir
    let current_path = PathBuf::from(js_file_str);
    let current_path_dir = current_path.parent().unwrap();

    // Call each exported function with the content
    let mut results = Vec::new();
    for func_item in exported_functions {
        let function_name = func_item.function_name;
        let target_path = if func_item.path.is_empty() {
            current_path_dir
                .join(format!("{}.{}", function_name, func_item.lang))
                .canonicalize()
                .unwrap()
                .to_string_lossy()
                .to_string()
        } else {
            current_path_dir
                .join(func_item.path)
                .canonicalize()
                .unwrap()
                .to_string_lossy()
                .to_string()
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
