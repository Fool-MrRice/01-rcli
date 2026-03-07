use std::io::{self, Write};

use anyhow::Result;
use base64::prelude::*;

pub fn process_decode(input: &str, output: &str) -> Result<()> {
    // 如果input是-，则从stdin读取，如果不是-，则从文件读取
    // 如果output是-，则写入stdout，如果不是-，则写入文件
    match (input, output) {
        ("-", "-") => {
            let stdin = io::stdin();
            let mut buffer = String::new();
            // 要有提示输入“input：”
            print!("Please input base64 encoded string:");
            io::stdout().flush()?; // 刷新缓冲区，确保提示显示
            stdin.read_line(&mut buffer)?;
            let decoded = BASE64_STANDARD.decode(buffer.trim())?; //trim()去掉首尾空格                                                      // 输出到屏幕，要能够正确显示二进制文本

            print!("Decoded: ");
            for (i, byte) in decoded.iter().enumerate() {
                print!("0x{:02x}", byte);
                if i < decoded.len() - 1 {
                    print!(" "); // 字节间空格
                }
            }

            // 打印 ASCII 字符
            print!("  (");
            for byte in &decoded {
                // 可打印字符显示，不可打印显示 .
                if *byte >= 0x20 && *byte <= 0x7e {
                    print!("{}", *byte as char);
                } else {
                    print!(".");
                }
            }
            println!(")");
        }
        ("-", _) => {
            let stdin = io::stdin();
            let mut buffer = String::new();
            stdin.read_line(&mut buffer)?;
            // 要有提示输入“input：”
            print!("Please input base64 encoded string:");
            io::stdout().flush()?; // 刷新缓冲区，确保提示显示
            let decoded = BASE64_STANDARD.decode(buffer.trim())?; //trim()去掉首尾空格

            std::fs::write(output, decoded)?;
        }
        (_, "-") => {
            let decoded = BASE64_STANDARD.decode(std::fs::read_to_string(input)?.as_bytes())?;
            print!("Decoded: ");
            for (i, byte) in decoded.iter().enumerate() {
                print!("0x{:02x}", byte);
                if i < decoded.len() - 1 {
                    print!(" "); // 字节间空格
                }
            }

            // 打印 ASCII 字符
            print!("  (");
            for byte in &decoded {
                // 可打印字符显示，不可打印显示 .
                if *byte >= 0x20 && *byte <= 0x7e {
                    print!("{}", *byte as char);
                } else {
                    print!(".");
                }
            }
            println!(")");
        }
        (_, _) => {
            let decoded = BASE64_STANDARD.decode(std::fs::read_to_string(input)?.as_bytes())?;
            std::fs::write(output, decoded)?;
        }
    }

    Ok(())
}
pub fn process_encode(input: &str, output: &str) -> Result<()> {
    // 如果input是-，则从stdin读取，如果不是-，则从文件读取
    // 如果output是-，则写入stdout，如果不是-，则写入文件
    match (input, output) {
        ("-", "-") => {
            let stdin = io::stdin();
            let mut buffer = String::new();
            // 要有提示输入“input：”
            print!("Please input base64 encoded string:");
            io::stdout().flush()?; // 刷新缓冲区，确保提示显示
            stdin.read_line(&mut buffer)?;
            let encoded = BASE64_STANDARD.encode(buffer.trim()); //trim()去掉首尾空格
            print!("Encoded: ");
            io::stdout().write_all(encoded.as_bytes())?;
            println!(); // ← 关键！最后换行
        }
        ("-", _) => {
            let stdin = io::stdin();
            let mut buffer = String::new();
            // 要有提示输入“input：”
            print!("Please input base64 encoded string:");
            io::stdout().flush()?; // 刷新缓冲区，确保提示显示
            stdin.read_line(&mut buffer)?;
            let encoded = BASE64_STANDARD.encode(buffer.trim()); //trim()去掉首尾空格
            std::fs::write(output, encoded)?;
        }
        (_, "-") => {
            let encoded = BASE64_STANDARD.encode(std::fs::read_to_string(input)?.as_bytes());
            print!("Encoded: ");
            io::stdout().write_all(encoded.as_bytes())?;
            println!(); // ← 关键！最后换行
        }
        (_, _) => {
            let encoded = BASE64_STANDARD.encode(std::fs::read_to_string(input)?.as_bytes());
            std::fs::write(output, encoded)?;
        }
    }

    Ok(())
}
