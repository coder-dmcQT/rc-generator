use std::{fs, io};

/// 处理文件：存在则删除，然后新建并写入内容
pub fn handle_file(file_path: &str, content: &str) -> io::Result<()> {
    // 尝试删除文件，若文件不存在则忽略错误
    if let Err(e) = fs::remove_file(file_path) {
        if e.kind() != io::ErrorKind::NotFound {
            // 若错误不是“文件不存在”，则返回错误
            return Err(e);
        }
    }

    // 写入内容（自动创建文件）
    fs::write(file_path, content)?;

    Ok(())
}