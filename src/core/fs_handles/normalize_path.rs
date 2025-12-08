use std::path::{Path, PathBuf, Component};

// Add this helper function once at the top of your file
pub fn normalize_path(path: &Path) -> PathBuf {
    let mut components = Vec::new();

    for component in path.components() {
        match component {
            Component::CurDir => {}
            Component::ParentDir => {
                if !components.is_empty() && components.last() != Some(&Component::RootDir) {
                    components.pop();
                }
            }
            comp => components.push(comp),
        }
    }

    let result: PathBuf = components.iter().collect();

    // Strip Windows \\?\ prefix
    strip_windows_prefix(&result)
}

fn strip_windows_prefix(path: &Path) -> PathBuf {
    let path_str = path.to_string_lossy();

    if cfg!(windows) && path_str.starts_with(r"\\?\") {
        PathBuf::from(&path_str[4..])
    } else {
        path.to_path_buf()
    }
}