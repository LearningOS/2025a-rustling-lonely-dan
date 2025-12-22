fn main() {
    // 获取当前时间戳（从 Unix 纪元到现在的秒数）
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    // 向 Cargo 发送指令，设置环境变量 TEST_FOO 为当前时间戳
    println!("cargo:rustc-env=TEST_FOO={}", timestamp);
}