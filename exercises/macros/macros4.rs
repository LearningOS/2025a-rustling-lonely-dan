#[rustfmt::skip]
macro_rules! my_macro {
    // 分支1：无参数匹配
    () => {
        println!("Check out my macro!");
    }; // 多分支之间必须用 ; 分隔（不是 ,）
    // 分支2：匹配单个表达式参数
    ($val:expr) => {
        println!("Look at this other macro: {}", $val);
    }; // 最后一个分支的 ; 可选，但推荐加
}

fn main() {
    // 调用无参分支
    my_macro!();
    // 调用带参分支（传入数字 7777）
    my_macro!(7777);
}