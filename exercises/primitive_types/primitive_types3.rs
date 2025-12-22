fn main() {
    // 方式1：使用 [0; 100] 创建长度为 100 的数组（元素全为 0）
    let a = [0; 100];

    // 也可以用其他长度（如 101、200、1000 等），只要 >=100 即可
    // let a = [1; 150];
    // let a = ["hello"; 200];

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
    }
}