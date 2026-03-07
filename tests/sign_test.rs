use std::fs;
use std::process::Command;
use tempfile::TempDir;

#[test]
fn test_sign_and_verify_hex() {
    let temp = TempDir::new().unwrap();
    let data_path = temp.path().join("data.txt");

    fs::write(&data_path, "hello world").unwrap();

    // 1. 签名
    let sign_output = Command::new("cargo")
        .args(&[
            "run",
            "--quiet",
            "--",
            "sign",
            "sign",
            "--key",
            "testsecret",
            "--input",
            data_path.to_str().unwrap(),
            "--format",
            "hex",
        ])
        .output()
        .expect("Failed to execute sign command");

    assert!(sign_output.status.success(), "Sign failed");

    let sign_stdout = String::from_utf8_lossy(&sign_output.stdout);
    let signature = sign_stdout
        .lines()
        .last()
        .unwrap_or("")
        .trim()
        .trim_matches('"')
        .to_string();

    assert_eq!(signature.len(), 64, "Bad signature length: {}", signature);

    // 2. 验证 - 使用编译好的二进制，避免 cargo 干扰
    let verify_output = Command::new("target/debug/rcli") // ✅ 直接用二进制
        .args(&[
            "sign",
            "verify",
            "--key",
            "testsecret",
            "--input",
            data_path.to_str().unwrap(),
            "--signature",
            &signature,
            "--format",
            "hex",
        ])
        .output()
        .expect("Failed to execute verify command");

    let verify_stdout = String::from_utf8_lossy(&verify_output.stdout); // ✅ 用 verify_output
    let verify_stderr = String::from_utf8_lossy(&verify_output.stderr); // ✅ 用 verify_output

    println!("VERIFY STDOUT: '{}'", verify_stdout);
    println!("VERIFY STDERR: '{}'", verify_stderr);

    assert!(verify_output.status.success(), "Verify failed");

    let combined = format!("{}{}", verify_stdout, verify_stderr);
    assert!(
        combined.contains("✅") || combined.contains("通过"),
        "Expected success. Got stdout: '{}', stderr: '{}'",
        verify_stdout,
        verify_stderr
    );
}

#[test]
fn test_sign_and_verify_base64() {
    let temp = TempDir::new().unwrap();
    let data_path = temp.path().join("data.txt");
    let sig_path = temp.path().join("sig.b64");

    fs::write(&data_path, "test data").unwrap();

    // Base64签名
    let output = Command::new("cargo")
        .args(&[
            "run",
            "--",
            "sign",
            "sign",
            "--key",
            "mykey",
            "--input",
            data_path.to_str().unwrap(),
            "--output",
            sig_path.to_str().unwrap(),
            "--format",
            "base64",
        ])
        .output()
        .expect("Failed to execute sign command");

    assert!(output.status.success());

    let signature = fs::read_to_string(&sig_path)
        .unwrap()
        .trim()
        .trim_matches('"')
        .to_string();

    // 验证 - 添加 --format base64
    let verify = Command::new("cargo")
        .args(&[
            "run",
            "--",
            "sign",
            "verify",
            "--key",
            "mykey",
            "--input",
            data_path.to_str().unwrap(),
            "--signature",
            &signature,
            "--format", // ✅ 添加格式参数
            "base64",
        ])
        .output()
        .expect("Verify failed");

    let stdout = String::from_utf8_lossy(&verify.stdout);
    assert!(
        stdout.contains("✅") || stdout.contains("通过"),
        "Base64 verify should pass"
    );
}

#[test]
fn test_verify_fails_with_wrong_key() {
    let temp = TempDir::new().unwrap();
    let data_path = temp.path().join("data.txt");

    fs::write(&data_path, "secret message").unwrap();

    // 用 key1 签名
    let sign = Command::new("cargo")
        .args(&[
            "run",
            "--",
            "sign",
            "sign",
            "--key",
            "key1",
            "--input",
            data_path.to_str().unwrap(),
            "--format",
            "hex",
        ])
        .output()
        .unwrap();

    let signature = String::from_utf8_lossy(&sign.stdout)
        .trim()
        .trim_matches('"')
        .to_string();

    // 用 key2 验证（应该失败）- 添加 --format hex
    let verify = Command::new("cargo")
        .args(&[
            "run",
            "--",
            "sign",
            "verify",
            "--key",
            "key2", // 错误的密钥
            "--input",
            data_path.to_str().unwrap(),
            "--signature",
            &signature,
            "--format", // ✅ 添加格式参数
            "hex",
        ])
        .output()
        .unwrap();

    let stderr = String::from_utf8_lossy(&verify.stderr);
    assert!(
        !verify.status.success() || stderr.contains("❌") || stderr.contains("失败"),
        "Verify with wrong key should fail"
    );
}

#[test]
fn test_verify_fails_with_tampered_data() {
    let temp = TempDir::new().unwrap();
    let data_path = temp.path().join("data.txt");
    let sig_path = temp.path().join("sig.hex");

    // 原始数据签名
    fs::write(&data_path, "original data").unwrap();

    Command::new("cargo")
        .args(&[
            "run",
            "--",
            "sign",
            "sign",
            "--key",
            "secret",
            "--input",
            data_path.to_str().unwrap(),
            "--output",
            sig_path.to_str().unwrap(),
            "--format", // ✅ 明确指定格式
            "hex",
        ])
        .output()
        .unwrap();

    let signature = fs::read_to_string(&sig_path)
        .unwrap()
        .trim()
        .trim_matches('"')
        .to_string();

    // 篡改数据
    fs::write(&data_path, "tampered data").unwrap();

    // 验证应该失败 - 添加 --format hex
    let verify = Command::new("cargo")
        .args(&[
            "run",
            "--",
            "sign",
            "verify",
            "--key",
            "secret",
            "--input",
            data_path.to_str().unwrap(),
            "--signature",
            &signature,
            "--format", // ✅ 添加格式参数
            "hex",
        ])
        .output()
        .unwrap();

    assert!(
        !verify.status.success(),
        "Verify should fail with tampered data"
    );
}

// test_key_from_file 和 test_stdin_input 不需要修改
