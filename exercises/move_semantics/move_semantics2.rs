fn main() {
    // 初始为空向量
    let vec0 = Vec::new();

    // 调用 fill_vec，获取修改后的向量并赋值给 vec0（所有权转移）
    let vec0 = fill_vec(vec0);

    // 克隆 vec0 得到 vec1，避免所有权移动，以便后续使用 vec0
    let mut vec1 = vec0.clone();

    // 打印 vec0 的信息（预期长度 3，内容 [22,44,66]）
    println!("{} has length {}, with contents: `{:?}`", "vec0", vec0.len(), vec0);

    // 给 vec1 追加 88
    vec1.push(88);

    // 打印 vec1 的信息（预期长度 4，内容 [22,44,66,88]）
    println!("{} has length {}, with contents `{:?}`", "vec1", vec1.len(), vec1);
}

fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}