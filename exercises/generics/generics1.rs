fn main() {
    // 明确指定向量的元素类型为 &str（或 &'static str）
    let mut shopping_list: Vec<&str> = Vec::new();
    shopping_list.push("milk");
}