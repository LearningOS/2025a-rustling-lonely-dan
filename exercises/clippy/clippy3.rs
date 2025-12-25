use std::mem;

#[allow(unused_variables)]
fn main() {
    // 1. 修复：None 调用 unwrap() 会 panic，is_none() 判断后无需额外操作
    let my_option: Option<()> = None;
    if my_option.is_none() {
        println!("my_option is None (as expected)");
    }

    // 2. 修复：数组元素间补充逗号，修正语法错误
    let my_arr = &[
        -1, -2, -3,  // 每行末尾保留逗号，Rust 支持尾逗号（更易维护）
        -4, -5, -6,
    ];
    println!("My array! Here it is: {:?}", my_arr);

    // 3. 修复：正确创建空 Vec（两种方式选其一，推荐第二种更直观）
    // 方式1：从有值 Vec 调整为空（保留你的原有逻辑）
    // let mut my_empty_vec = vec![1, 2, 3, 4, 5];
    // my_empty_vec.resize(0, 5);
    
    // 方式2：直接创建空 Vec（更简洁、符合直觉，推荐）
    let my_empty_vec: Vec<i32> = Vec::new();
    println!("This Vec is empty, see? {:?}", my_empty_vec);

    // 4. 修复：正确交换两个变量（两种方式都可行，mem::swap 更规范）
    let mut value_a = 45;
    let mut value_b = 66;
    mem::swap(&mut value_a, &mut value_b); // 标准库 swap 函数（推荐）
    // 备选：元组解构交换 (value_a, value_b) = (value_b, value_a);
    println!("value a: {}; value b: {}", value_a, value_b);
}