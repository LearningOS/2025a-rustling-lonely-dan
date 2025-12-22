pub fn is_even(num: i32) -> bool {
    num % 2 == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_true_when_even() {
        // 传入偶数（如4），断言返回值为true
        assert!(is_even(4));
        // 也可以用assert_eq!更明确：assert_eq!(is_even(2), true);
    }

    #[test]
    fn is_false_when_odd() {
        // 按题目要求测试is_even(5)，断言返回值为false
        assert!(!is_even(5));
        // 也可以用assert_eq!：assert_eq!(is_even(5), false);
    }
}