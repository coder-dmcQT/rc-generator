use crate::core::data_hold::data_struct::{FailureResult, SuccessResult};
use crate::core::fs_handles::handle_file::handle_file;
use std::io::Error;

pub(crate) fn write_fs_path_content_paris(pairs: &Vec<(String, String)>) -> Result<bool, Error> {
    for (path, content) in pairs {
        match handle_file(path, content) {
            Ok(_) => {}
            Err(e) => {
                return Err(e);
            }
        }
    }
    Ok(true)
}

pub fn write_pairs(
    pairs: &Vec<(String, String)>,
    successes: &mut Vec<SuccessResult>,
    failures: &mut Vec<FailureResult>,
    function_name: String,
) {
    match write_fs_path_content_paris(&pairs) {
        Ok(_) => {
            successes.push(SuccessResult {
                name: function_name,
                path: "".to_string(),
            });
        }
        Err(e) => {
            failures.push(FailureResult {
                error: e.to_string(),
                name: function_name,
                path: "".to_string(),
            });
        }
    }
}
