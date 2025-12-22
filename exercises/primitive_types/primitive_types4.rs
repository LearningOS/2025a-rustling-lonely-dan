#[test]
fn slice_out_of_array() {
    let a = [1, 2, 3, 4, 5];

    // 截取从索引 1 到 4 的切片（包含 1，不包含 4），对应元素 2、3、4
    let nice_slice = &a[1..4];

    assert_eq!([2, 3, 4], nice_slice)
}