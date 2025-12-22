use std::mem;

#[allow(unused_variables)]
fn main() {
    let my_option: Option<()> = None;
    // 修复：移除无意义的 unwrap()（None 调用 unwrap 会 panic，且 is_none() 后无需处理）
    if my_option.is_none() {
        // 空逻辑，或根据需求改为有意义的操作
    }

    // 修复：数组元素间添加逗号，修正语法错误
    let my_arr = &[
        -1, -2, -3,
        -4, -5, -6,
    ];
    println!("My array! Here it is: {:?}", my_arr);

    // 修复：正确创建空 Vec（resize 返回 ()，应先创建 Vec 再调用 resize，或直接用 Vec::new()）
    let mut my_empty_vec = vec![1, 2, 3, 4, 5];
    my_empty_vec.resize(0, 5); // 调整为空
    // 或更简单：let my_empty_vec: Vec<i32> = Vec::new();
    println!("This Vec is empty, see? {:?}", my_empty_vec);

    let mut value_a = 45;
    let mut value_b = 66;
    // 修复：使用 std::mem::swap 正确交换变量（Clippy 推荐）
    mem::swap(&mut value_a, &mut value_b);
    // 或用元组解构：(value_a, value_b) = (value_b, value_a);
    println!("value a: {}; value b: {}", value_a, value_b);
}