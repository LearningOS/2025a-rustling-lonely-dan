#[derive(PartialEq, Debug)]
pub enum List {
    // 用 Box 包装递归的 List，解决大小计算问题
    Cons(i32, Box<List>),
    Nil,
}

fn main() {
    println!("This is an empty cons list: {:?}", create_empty_list());
    println!(
        "This is a non-empty cons list: {:?}",
        create_non_empty_list()
    );
}

pub fn create_empty_list() -> List {
    // 空列表直接返回 Nil
    List::Nil
}

pub fn create_non_empty_list() -> List {
    // 构造非空列表，例如 Cons(1, Box::new(Nil)) 或更复杂的嵌套
    List::Cons(1, Box::new(List::Nil))
    // 也可以构造嵌套的列表，比如：
    // List::Cons(5, Box::new(List::Cons(3, Box::new(List::Nil))))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_empty_list() {
        assert_eq!(List::Nil, create_empty_list())
    }

    #[test]
    fn test_create_non_empty_list() {
        assert_ne!(create_empty_list(), create_non_empty_list())
    }
}