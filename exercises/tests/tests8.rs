fn main() {
    // ========== 兼容 tests7：设置 TEST_FOO 环境变量 ==========
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    println!("cargo:rustc-env=TEST_FOO={}", timestamp);

    // ========== 兼容 tests8：注入 feature = "pass" 的条件编译标志 ==========
    println!("cargo:rustc-cfg=feature=\"pass\"");
}