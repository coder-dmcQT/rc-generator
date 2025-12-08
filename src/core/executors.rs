use crate::core::boa_js_specific::call_func::call_function;
use crate::core::data_hold::data_struct::{
    Config, FailureResult, FunctionReturnType, SuccessResult,
};
use crate::core::fs_handles::handle_file::handle_file;
use crate::core::fs_handles::normalize_path::normalize_path;
use crate::core::fs_handles::replace_prefix_or_not::replace_prefix_longest;
use crate::core::functions_handles::call_in_compose::call_function_typed;
use crate::core::functions_handles::find_exported_functions::{
    find_composed_functions, find_exported_functions,
};
use crate::core::functions_handles::make_uuid_for_path_not::make_uuid_for_path_not;
use crate::core::functions_handles::write_fs_kv_pairs::write_pairs;
use boa_engine::{Context, Source};
use std::path::PathBuf;

pub fn execute_exported_functions(
    js_code: &str,
    content: &str,
    js_file_str: &str,
    generation_config: &Config,
) -> Result<(Vec<SuccessResult>, Vec<FailureResult>), String> {
    // Create JavaScript context
    let mut context = Context::default();

    // Evaluate the JavaScript code
    context
        .eval(Source::from_bytes(js_code))
        .map_err(|e| format!("Failed to evaluate JavaScript: {}", e))?;

    // Find all functions with "path" property
    let exported_functions = find_exported_functions(&mut context)?;

    // Find all compose functions with "compose" property
    let composed_functions = find_composed_functions(&mut context)?;

    let mut successes = Vec::new();
    let mut failures = Vec::new();

    if exported_functions.is_empty() && composed_functions.is_empty() {
        return Ok((successes, failures));
    }

    // Store the temporary parent dir
    let current_path = PathBuf::from(js_file_str);
    let current_path_dir = current_path.parent().unwrap();

    let path_alias = generation_config.alias.clone().unwrap();

    println!("config here {:?}", path_alias);

    // Call each exported function with the content
    for func_item in exported_functions {
        let function_name = func_item.function_name;
        let target_path = {
            let path = match replace_prefix_longest(func_item.path.as_str(), &path_alias) {
                Ok(path) => PathBuf::from(path),
                Err(_) => current_path_dir.join(func_item.path),
            };
            normalize_path(&*path).to_string_lossy().to_string()
        };
        let result = call_function(&mut context, &function_name, content);
        match handle_file(&*target_path, result?.as_str()) {
            Ok(_result) => {
                successes.push(SuccessResult {
                    name: function_name,
                    path: target_path.to_string(),
                });
            }
            Err(e) => {
                failures.push(FailureResult {
                    name: function_name,
                    error: e.to_string(),
                    path: target_path.to_string(),
                });
            }
        };
    }

    // Handle the compose function calls
    for composed_function in composed_functions {
        let result = call_function_typed(
            &mut context,
            &*composed_function.function_name,
            content,
            &path_alias,
            current_path_dir,
        );
        match result {
            Ok(result) => match result {
                FunctionReturnType::Array2D(path_content_pairs) => {
                    write_pairs(
                        &path_content_pairs,
                        &mut successes,
                        &mut failures,
                        composed_function.function_name,
                    );
                }
                FunctionReturnType::Object(path_content_pairs) => {
                    write_pairs(
                        &path_content_pairs,
                        &mut successes,
                        &mut failures,
                        composed_function.function_name,
                    );
                }
                FunctionReturnType::String(string_content) => {
                    let newly_path = make_uuid_for_path_not(&composed_function.function_name);
                    match handle_file(&*newly_path, &*string_content) {
                        Ok(_) => {
                            successes.push(SuccessResult {
                                name: composed_function.function_name,
                                path: newly_path.to_string(),
                            });
                        }
                        Err(e) => {
                            failures.push(FailureResult {
                                error: e.to_string(),
                                name: composed_function.function_name,
                                path: newly_path.to_string(),
                            });
                        }
                    }
                }
            },
            Err(e) => {
                failures.push(FailureResult {
                    name: composed_function.function_name,
                    error: e.to_string(),
                    path: "".to_string(),
                });
            }
        };
    }

    Ok((successes, failures))
}
