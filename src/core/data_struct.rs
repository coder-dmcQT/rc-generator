use clap::Parser;

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
pub struct FunctionResult {
    pub name: String,
    pub result: Result<String, String>,
    pub path: String,
}

#[derive(Debug)]
pub struct FunctionForExecute {
    pub function_name: String,
    pub path: String,
    pub lang: String,
}