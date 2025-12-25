// 正确定义无参数声明式宏
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

fn main() {
    // 关键修复：宏调用必须加 ! 符号
    my_macro!();
}