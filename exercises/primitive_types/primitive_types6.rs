#[test]
fn indexing_tuple() {
    let numbers = (1, 2, 3);
    // 使用元组索引语法访问第二个元素（索引为 1）
    let second = numbers.1;

    assert_eq!(2, second,
        "This is not the 2nd number in the tuple!")
}