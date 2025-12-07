pub(crate) mod core;

use crate::core::data_struct::{Args, ExecutionOutcome};
use crate::core::execute_exported_function::execute_exported_functions;
use crate::core::print_console::{print_content_required, print_js_required};
use clap::Parser;
use std::fs;

/// JavaScript Function Evaluator - Execute exported functions with content from fil

fn main() {
    let args = Args::parse();

    // Get file paths from either flags or positional arguments
    let js_file = args.file.or_else(|| args.positional.get(0).cloned());
    let content_file = args.content.or_else(|| args.positional.get(1).cloned());

    // Validate arguments
    let js_file = match js_file {
        Some(path) => path,
        None => {
            print_js_required();
            std::process::exit(1);
        }
    };

    // Convert to absolute path
    let js_file_absolute = match fs::canonicalize(&js_file) {
        Ok(abs_path) => abs_path,
        Err(e) => {
            eprintln!("Error: Cannot find JavaScript file '{}': {}", js_file, e);
            std::process::exit(1);
        }
    };

    let js_file_str = js_file_absolute.to_string_lossy().to_string();

    let content_file = match content_file {
        Some(path) => path,
        None => {
            print_content_required();
            std::process::exit(1);
        }
    };

    // Read JavaScript file
    let js_code = match fs::read_to_string(&js_file) {
        Ok(code) => code,
        Err(e) => {
            eprintln!("Error reading JavaScript file '{}': {} File path is {}", js_file, e, js_file_str);
            std::process::exit(1);
        }
    };

    // Read content file
    let content = match fs::read_to_string(&content_file) {
        Ok(text) => text,
        Err(e) => {
            eprintln!("Error reading content file '{}': {}", content_file, e);
            std::process::exit(1);
        }
    };

    // Execute exported functions
    match execute_exported_functions(&js_code, &content, &js_file_str) {
        Ok(results) => {
            if results.is_empty() {
                println!("⚠️  No functions for execution found");
            } else {
                println!("✅ Executed {} function(s):\n", results.len());
                for func_result in results {
                    match func_result.result {
                        ExecutionOutcome::Success(_result) => {
                            println!("  • {} path is {}", func_result.name, func_result.path);
                        }
                        ExecutionOutcome::Failure(_result) => {
                            println!("  • {} => Error: {}", func_result.name, func_result.path);
                        }
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("❌ Error: {}", e);
            std::process::exit(1);
        }
    }
}


