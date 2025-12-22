// This function returns how much icecream there is left in the fridge.
// If it's before 10PM, there's 5 pieces left. At 10PM, someone eats them
// all, so there'll be no more left :(
fn maybe_icecream(time_of_day: u16) -> Option<u16> {
    // 先判断时间是否在合法范围（0-23）
    if time_of_day > 23 {
        None
    } else if time_of_day < 22 {
        // 10PM 前，返回 5 份
        Some(5)
    } else {
        // 10PM 及以后，返回 0 份
        Some(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_icecream() {
        assert_eq!(maybe_icecream(9), Some(5));
        assert_eq!(maybe_icecream(10), Some(5));
        assert_eq!(maybe_icecream(23), Some(0));
        assert_eq!(maybe_icecream(22), Some(0));
        assert_eq!(maybe_icecream(25), None);
    }

    #[test]
    fn raw_value() {
        // 提取 Option 中的值：使用 unwrap()（测试场景下安全，因为我们知道是 Some(5)）
        let icecreams = maybe_icecream(12).unwrap();
        assert_eq!(icecreams, 5);

        // 也可以使用 expect()（更友好的错误提示）
        // let icecreams = maybe_icecream(12).expect("Expected icecream!");

        // 或模式匹配（更安全，适合生产代码）
        // let icecreams = match maybe_icecream(12) {
        //     Some(v) => v,
        //     None => panic!("No icecream!"),
        // };
    }
}