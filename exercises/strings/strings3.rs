fn trim_me(input: &str) -> String {
    // 去除首尾空白，再转换为 String
    input.trim().to_string()
}

fn compose_me(input: &str) -> String {
    // 方法1：使用 format! 宏（推荐）
    format!("{} world!", input)

    // 方法2：使用 + 运算符（也可实现）
    // input.to_string() + " world!"

    // 方法3：使用 push_str 方法（也可实现）
    // let mut s = input.to_string();
    // s.push_str(" world!");
    // s
}

fn replace_me(input: &str) -> String {
    // 替换所有 "cars" 为 "balloons"
    input.replace("cars", "balloons")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_a_string() {
        assert_eq!(trim_me("Hello!     "), "Hello!");
        assert_eq!(trim_me("  What's up!"), "What's up!");
        assert_eq!(trim_me("   Hola!  "), "Hola!");
    }

    #[test]
    fn compose_a_string() {
        assert_eq!(compose_me("Hello"), "Hello world!");
        assert_eq!(compose_me("Goodbye"), "Goodbye world!");
    }

    #[test]
    fn replace_a_string() {
        assert_eq!(replace_me("I think cars are cool"), "I think balloons are cool");
        assert_eq!(replace_me("I love to look at cars"), "I love to look at balloons");
    }
}