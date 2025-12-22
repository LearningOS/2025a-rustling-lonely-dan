fn main() {
    let data = "Rust is great!".to_string();

    get_char(&data); // 传递引用，不转移所有权

    string_uppercase(data); // 传递所有权，符合函数要求
}

// 改为接收不可变引用，不获取所有权
fn get_char(data: &String) -> char {
    data.chars().last().unwrap()
}

// 保持接收所有权，但修正为使用 to_uppercase() 的返回值
fn string_uppercase(mut data: String) {
    data = data.to_uppercase(); // 重新赋值为大写版本

    println!("{}", data);
}