#[cfg(test)]
mod tests {
    #[test]
    fn you_can_assert_eq() {
        // 传入两个相等的值，断言成功，测试通过
        assert_eq!(1 + 1, 2);
        // 也可以传入字符串、其他数值类型等
        // assert_eq!("hello", "hello");
        // assert_eq!(5, 5);
    }
}