use base64::{engine::general_purpose::STANDARD, Engine};
use std::path::Path;
/// 解析密钥来源
/// - "@file.txt" → 从文件读取
/// - "secret" → 直接使用字符串
pub fn parse_key(source: &str) -> Result<Vec<u8>, String> {
    if let Some(path) = source.strip_prefix('@') {
        if !Path::new(path).exists() {
            return Err(format!("密钥文件不存在: {}", path));
        }
        std::fs::read(path).map_err(|e| format!("读取密钥文件失败: {}", e))
    } else {
        Ok(source.as_bytes().to_vec())
    }
}

/// 解析签名（自动检测hex/base64/文件）
pub fn parse_signature(source: &str) -> Result<Vec<u8>, String> {
    // 尝试文件路径
    let content = if let Some(path) = source.strip_prefix('@') {
        if !Path::new(path).exists() {
            return Err(format!("签名文件不存在: {}", path));
        }
        std::fs::read_to_string(path).map_err(|e| format!("读取签名文件失败: {}", e))?
    } else {
        source.to_string()
    };

    let trimmed = content.trim();

    // 尝试hex（64字符，全十六进制）
    if trimmed.len() == 64 && trimmed.chars().all(|c| c.is_ascii_hexdigit()) {
        return hex::decode(trimmed).map_err(|e| format!("hex解码失败: {}", e));
    }

    // 尝试base64
    match STANDARD.decode(trimmed) {
        Ok(bytes) if bytes.len() == 32 => Ok(bytes),
        Ok(_) => Err("base64解码后长度不是32字节（不是SHA256签名）".to_string()),
        Err(e) => Err(format!("base64解码失败: {}", e)),
    }
}
