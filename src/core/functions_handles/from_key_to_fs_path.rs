use crate::core::boa_js_specific::js_convert_judge::extract_string;
use crate::core::fs_handles::normalize_path::normalize_path;
use crate::core::fs_handles::replace_prefix_or_not::replace_prefix_longest;
use boa_engine::JsValue;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

pub fn from_js_value_to_path_string(
    path_alias: &HashMap<String, String>,
    current_path: &Path,
    keyPath: JsValue,
) -> String {
    let key_extracted = extract_string(&keyPath);
    let key_str = match replace_prefix_longest(&*key_extracted, path_alias) {
        Ok(path) => PathBuf::from(path),
        Err(_) => current_path.join(key_extracted),
    };
    normalize_path(&key_str).to_string_lossy().to_string()
}

pub fn from_string_to_path_string(
    path_alias: &HashMap<String, String>,
    current_path: &Path,
    keyPath: String,
) -> String {
    let key_str = match replace_prefix_longest(&*keyPath, path_alias) {
        Ok(path) => PathBuf::from(path),
        Err(_) => current_path.join(keyPath),
    };
    normalize_path(&key_str).to_string_lossy().to_string()
}
