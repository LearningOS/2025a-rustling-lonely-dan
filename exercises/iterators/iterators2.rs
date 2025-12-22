// Step 1. 实现首字符大写的函数
pub fn capitalize_first(input: &str) -> String {
    let mut c = input.chars();
    match c.next() {
        None => String::new(),
        Some(first) => {
            // 首字符大写，拼接剩余字符
            first.to_uppercase().chain(c).collect()
        },
    }
}

// Step 2. 对切片中的每个字符串应用 capitalize_first，返回向量
pub fn capitalize_words_vector(words: &[&str]) -> Vec<String> {
    // 迭代每个单词，应用函数，收集为向量
    words.iter().map(|&word| capitalize_first(word)).collect()
}

// Step 3. 对切片中的每个字符串应用 capitalize_first，拼接为单个字符串
pub fn capitalize_words_string(words: &[&str]) -> String {
    // 迭代每个单词，应用函数，拼接为字符串
    words.iter().map(|&word| capitalize_first(word)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(capitalize_first("hello"), "Hello");
    }

    #[test]
    fn test_empty() {
        assert_eq!(capitalize_first(""), "");
    }

    #[test]
    fn test_iterate_string_vec() {
        let words = vec!["hello", "world"];
        assert_eq!(capitalize_words_vector(&words), ["Hello", "World"]);
    }

    #[test]
    fn test_iterate_into_string() {
        let words = vec!["hello", " ", "world"];
        assert_eq!(capitalize_words_string(&words), "Hello World");
    }
}