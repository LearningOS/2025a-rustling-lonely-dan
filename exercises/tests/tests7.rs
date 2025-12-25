// exercises/tests/build.rs
fn main() {
    // 获取当前 Unix 时间戳（秒数）
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    // 输出合法的 Cargo 指令：cargo:rustc-env=KEY=VALUE（必须包含 =）
    // 作用：将 TEST_FOO 环境变量注入到编译过程中
    println!("cargo:rustc-env=TEST_FOO={}", timestamp);

    // 普通日志输出（无 cargo: 前缀，仅用于调试，不会触发错误）
    println!("Build script running, timestamp: {}", timestamp);
}