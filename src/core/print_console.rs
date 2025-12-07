pub fn print_js_required() {
    eprintln!("Error: JavaScript file path required");
    eprintln!("Usage: js_evaluator --file <JS_FILE> --content <CONTENT_FILE>");
    eprintln!("   or: js_evaluator <JS_FILE> <CONTENT_FILE>");
}

pub fn print_content_required() {
    eprintln!("Error: Content file path required");
    eprintln!("Usage: js_evaluator --file <JS_FILE> --content <CONTENT_FILE>");
    eprintln!("   or: js_evaluator <JS_FILE> <CONTENT_FILE>");
}
