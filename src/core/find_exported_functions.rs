use boa_engine::Context;

pub fn find_exported_functions(context: &mut Context) -> Result<Vec<String>, String> {
    let global = context.global_object();
    let mut exported = Vec::new();

    let keys = global
        .own_property_keys(context)
        .map_err(|e| format!("Failed to get property keys: {}", e))?;

    for key in keys {
        // Convert PropertyKey to string
        let name = match &key {
            boa_engine::property::PropertyKey::String(s) => s.to_std_string_escaped(),
            boa_engine::property::PropertyKey::Symbol(_) => continue,
            boa_engine::property::PropertyKey::Index(idx) => idx.get().to_string(),
        };

        // Skip built-in functions
        if is_builtin(&name) {
            continue;
        }

        // Check if it starts with "export_"
        if !name.starts_with("export_") {
            continue;
        }

        let value = global
            .get(key, context)
            .map_err(|e| format!("Failed to get value: {}", e))?;

        if value.is_callable() {
            exported.push(name);
        }
    }

    Ok(exported)
}

fn is_builtin(name: &str) -> bool {
    matches!(
        name,
        "Object" | "Array" | "String" | "Number" | "Boolean"
            | "Function" | "Math" | "Date" | "RegExp" | "JSON"
            | "console" | "parseInt" | "parseFloat" | "isNaN" | "isFinite"
            | "eval" | "undefined" | "NaN" | "Infinity" | "Error"
            | "TypeError" | "RangeError" | "SyntaxError" | "Symbol"
            | "Promise" | "Map" | "Set" | "WeakMap" | "WeakSet"
            | "Proxy" | "Reflect" | "ArrayBuffer" | "DataView"
            | "Int8Array" | "Uint8Array" | "Int16Array" | "Uint16Array"
            | "Int32Array" | "Uint32Array" | "Float32Array" | "Float64Array"
    )
}