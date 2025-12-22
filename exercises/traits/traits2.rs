trait AppendBar {
    fn append_bar(self) -> Self;
}

// 为 Vec<String> 实现 AppendBar 特质
impl AppendBar for Vec<String> {
    fn append_bar(self) -> Self {
        // 将 self 绑定为可变变量，以便修改向量
        let mut vec = self;
        // 向向量中追加 "Bar" 字符串
        vec.push(String::from("Bar"));
        // 返回修改后的向量
        vec
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_vec_pop_eq_bar() {
        let mut foo = vec![String::from("Foo")].append_bar();
        assert_eq!(foo.pop().unwrap(), String::from("Bar"));
        assert_eq!(foo.pop().unwrap(), String::from("Foo"));
    }
}