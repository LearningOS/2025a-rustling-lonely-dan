use std::collections::HashMap;

fn fruit_basket() -> HashMap<String, u32> {
    // 初始化空的哈希映射（可变，因为要插入元素）
    let mut basket = HashMap::new();

    // 已有两个香蕉
    basket.insert(String::from("banana"), 2);

    // 插入苹果（数量2）和芒果（数量2），满足：
    // - 水果种类：香蕉、苹果、芒果（3种，满足≥3）
    // - 总数：2+2+2=6（满足≥5）
    basket.insert(String::from("apple"), 2);
    basket.insert(String::from("mango"), 2);

    basket
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at_least_three_types_of_fruits() {
        let basket = fruit_basket();
        assert!(basket.len() >= 3);
    }

    #[test]
    fn at_least_five_fruits() {
        let basket = fruit_basket();
        assert!(basket.values().sum::<u32>() >= 5);
    }
}