fn main() {
    // Characters (`char`)

    // Note the _single_ quotes, these are different from the double quotes
    // you've been seeing around.
    let my_first_initial = 'C';
    if my_first_initial.is_alphabetic() {
        println!("Alphabetical!");
    } else if my_first_initial.is_numeric() {
        println!("Numerical!");
    } else {
        println!("Neither alphabetic nor numeric!");
    }

    // 可替换为任意合法的 char：字母、数字、特殊符号、其他语言字符、emoji 等
    let your_character = '😜'; // 示例：emoji 字符
    // let your_character = '5'; // 数字
    // let your_character = '中'; // 中文汉字
    // let your_character = '!'; // 特殊符号
    // let your_character = 'A'; // 字母

    if your_character.is_alphabetic() {
        println!("Alphabetical!");
    } else if your_character.is_numeric() {
        println!("Numerical!");
    } else {
        println!("Neither alphabetic nor numeric!");
    }
}