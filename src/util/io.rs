// use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;

/// 统一读取输入（文件或stdin）
pub fn read(source: &str) -> io::Result<Vec<u8>> {
    if source == "-" {
        let mut buffer = Vec::new();
        io::stdin().read_to_end(&mut buffer)?;
        Ok(buffer)
    } else {
        std::fs::read(source)
    }
}

/// 统一写入输出（文件或stdout）
pub fn write(dest: &str, data: &[u8]) -> io::Result<()> {
    if dest == "-" {
        let stdout = io::stdout();
        let mut handle = stdout.lock();
        handle.write_all(data)?;
        handle.flush()?;
    } else {
        // 确保父目录存在
        if let Some(parent) = Path::new(dest).parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(dest, data)?;
    }
    Ok(())
}
