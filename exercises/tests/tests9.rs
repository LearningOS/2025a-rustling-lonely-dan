extern "Rust" {
    // 为 my_demo_function_alias 添加 link_name 属性，指向 my_demo_function 的符号
    #[link_name = "my_demo_function"]
    fn my_demo_function_alias(a: u32) -> u32;
    fn my_demo_function(a: u32) -> u32;
}

mod Foo {
    // 为函数添加 no_mangle 抑制符号混淆，pub 提升可见性
    #[no_mangle]
    pub fn my_demo_function(a: u32) -> u32 {
        a
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        // SAFETY: We know those functions are aliases of a safe
        // Rust function.
        unsafe {
            my_demo_function(123);
            my_demo_function_alias(456);
        }
    }
}