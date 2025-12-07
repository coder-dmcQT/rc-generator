use crate::core::call_func::call_function;
use crate::core::data_struct::{SuccessResult, FailureResult};
use crate::core::find_exported_functions::find_exported_functions;
use boa_engine::{Context, Source};
use std::path::PathBuf;
use crate::core::normalize_path::normalize_path;
use crate::core::handle_file::handle_file;

/// Execute all functions prefixed with "export_" using content as parameter
pub fn execute_exported_functions(
    js_code: &str,
    content: &str,
    js_file_str: &str,
) -> Result<(Vec<SuccessResult>, Vec<FailureResult>), String> {
    // Create JavaScript context
    let mut context = Context::default();

    // Evaluate the JavaScript code
    context
        .eval(Source::from_bytes(js_code))
        .map_err(|e| format!("Failed to evaluate JavaScript: {}", e))?;

    // Find all functions with "export_" prefix
    let exported_functions = find_exported_functions(&mut context)?;

    let mut successes = Vec::new();
    let mut failures = Vec::new();

    if exported_functions.is_empty() {
        return Ok((successes, failures));
    }

    // Store the temporary parent dir
    let current_path = PathBuf::from(js_file_str);
    let current_path_dir = current_path.parent().unwrap();

    // Call each exported function with the content
    for func_item in exported_functions {
        let function_name = func_item.function_name;
        let target_path =  {
            normalize_path(&current_path_dir.join(func_item.path))
                .to_string_lossy()
                .to_string()
        };
        let result = call_function(&mut context, &function_name, content);
        match handle_file(&*target_path, result?.as_str()) {
            Ok(_result) => {
                successes.push(SuccessResult {
                    name: function_name,
                    path: target_path.to_string(),
                });
            },
            Err(e) => {
                failures.push(FailureResult {
                    name: function_name,
                    error: e.to_string(),
                    path: target_path.to_string(),
                });
            }
        };
    }

    Ok((successes, failures))
}
