#[rustfmt::skip]
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    }, // 添加分号或逗号分隔多个分支（推荐逗号）
    ($val:expr) => {
        println!("Look at this other macro: {}", $val);
    }; // 最后一个分支可以加分号（可选，推荐添加以保证语法完整）
}

fn main() {
    my_macro!();
    my_macro!(7777);
}