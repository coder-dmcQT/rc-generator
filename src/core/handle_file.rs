use std::{fs, io};
use std::path::Path;

pub fn handle_file(file_path: &str, content: &str) -> io::Result<()> {

    if let Err(e) = fs::remove_file(file_path) {
        if e.kind() != io::ErrorKind::NotFound {
            return Err(e);
        }
    }

    if let Some(parent) = Path::new(file_path).parent() {
        fs::create_dir_all(parent)?;
    }

    fs::write(file_path, content)?;

    Ok(())
}