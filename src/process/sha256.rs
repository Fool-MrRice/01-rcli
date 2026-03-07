use crate::cli::SignFormat;
use crate::util::{parse_key, read, write};
use anyhow::{Context, Result};
use base64::prelude::*;
use base64::{engine::general_purpose::STANDARD, Engine};
use hmac::{Hmac, Mac};
use sha2::Sha256;
use std::io::Write;

type HmacSha256 = Hmac<Sha256>;

pub fn process_sign(input: &str, key: &str, output: &str, format: SignFormat) -> Result<()> {
    // 1. 读取密钥
    let key_bytes = parse_key(key).map_err(|e| anyhow::anyhow!("密钥解析失败: {}", e))?;

    // 2. 读取输入数据
    let data = read(input).with_context(|| format!("读取输入失败: {}", input))?;

    // 3. 计算 HMAC-SHA256
    let mut mac = HmacSha256::new_from_slice(&key_bytes)
        .map_err(|e| anyhow::anyhow!("HMAC初始化失败: {}", e))?;
    mac.update(&data);
    let result = mac.finalize().into_bytes();

    // 4. 格式化为字符串
    let output_str = match format {
        SignFormat::Hex => hex::encode(result),
        SignFormat::Base64 => STANDARD.encode(result),
    };

    // 5. 输出结果
    write(output, output_str.as_bytes()).with_context(|| format!("写入输出失败: {}", output))?;

    // 6. 添加换行符（如果是stdout，方便查看）
    if output == "-" {
        write(output, b"\n")?;
    }

    Ok(())
}

pub fn process_verify(input: &str, key: &str, signature: &str, format: SignFormat) -> Result<()> {
    // panic!("!!! VERIFY BRANCH ENTERED !!!");

    // 1. 读取密钥
    let key_bytes = parse_key(key).map_err(|e| anyhow::anyhow!("密钥解析失败: {}", e))?;

    // 2. 读取输入数据
    let data = read(input).with_context(|| format!("读取输入失败: {}", input))?;

    // 3. 根据格式解析签名
    let sig_bytes = match format {
        SignFormat::Hex => {
            hex::decode(signature.trim()).map_err(|e| anyhow::anyhow!("hex解码失败: {}", e))?
        }
        SignFormat::Base64 => BASE64_STANDARD
            .decode(signature.trim())
            .map_err(|e| anyhow::anyhow!("base64解码失败: {}", e))?,
    };

    // 4. 计算并验证
    let mut mac = HmacSha256::new_from_slice(&key_bytes)
        .map_err(|e| anyhow::anyhow!("HMAC初始化失败: {}", e))?;
    mac.update(&data);

    // 5. 安全比较（防时序攻击）
    match mac.verify_slice(&sig_bytes) {
        Ok(_) => {
            println!("✅ 签名验证通过");
            std::io::stdout().flush().unwrap();
            Ok(())
        }
        Err(_) => {
            anyhow::bail!("❌ 签名验证失败")
        }
    }
}
