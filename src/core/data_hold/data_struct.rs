use std::collections::HashMap;
use clap::Parser;
use serde::Deserialize;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Path to the JavaScript file
    #[arg(short, long, value_name = "FILE")]
    pub file: Option<String>,

    /// Path to the content/parameter file
    #[arg(short, long, value_name = "CONTENT")]
    pub content: Option<String>,

    /// Positional arguments (fallback if flags not used)
    #[arg(value_name = "ARGS")]
    pub positional: Vec<String>,
}
#[derive(Debug)]
pub struct SuccessResult {
    pub name: String,
    pub path: String,
}

/// Enhanced function result with type information
#[derive(Debug)]
pub enum FunctionReturnType {
    Array2D(Vec<(String, String)>),  // 2D array: [["k1","v1"], ["k2","v2"]]
    Object(Vec<(String, String)>),   // Object: {k1: "v1", k2: "v2"}
    String(String),                   // Everything else converted to string (JS way)
}

// 定义与JSON结构匹配的结构体
#[derive(Debug, Deserialize)]
pub struct Config {
    pub alias: Option<HashMap<String, String>>, 
}

#[derive(Debug)]
pub struct FailureResult {
    pub name: String,
    pub error: String,  // 仅存错误信息
    pub path: String,
}

#[derive(Debug)]
pub struct FunctionForExecute {
    pub function_name: String,
    pub path: String,
}