fn main() {
    let mut res = 42;
    let option = Some(12);

    // 方案1：使用 if let 模式匹配（Clippy 最推荐的方式，直观且高效）
    if let Some(x) = option {
        res += x;
    }

    // 方案2：使用 map 方法（函数式风格，适合简单操作）
    // option.map(|x| res += x);

    // 方案3：使用 unwrap_or_default（若 Option 为 None 则加 0，效果一致）
    // res += option.unwrap_or_default();

    println!("{}", res);
}