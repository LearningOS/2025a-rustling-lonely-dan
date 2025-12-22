trait AppendBar {
    fn append_bar(self) -> Self;
}

impl AppendBar for String {
    // 实现 append_bar 方法，追加 "Bar" 并返回自身
    fn append_bar(self) -> Self {
        // 创建可变的字符串副本（或直接修改 self，因为 self 是值传递）
        let mut s = self;
        s.push_str("Bar");
        s
    }
}

fn main() {
    let s = String::from("Foo");
    let s = s.append_bar();
    println!("s: {}", s);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_foo_bar() {
        assert_eq!(String::from("Foo").append_bar(), String::from("FooBar"));
    }

    #[test]
    fn is_bar_bar() {
        assert_eq!(
            String::from("").append_bar().append_bar(),
            String::from("BarBar")
        );
    }
}