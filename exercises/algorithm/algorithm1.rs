use std::fmt::{self, Display, Formatter};
use std::ptr::NonNull;

#[derive(Debug)]
struct Node<T> {
    val: T,
    next: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    fn new(t: T) -> Node<T> {
        Node {
            val: t,
            next: None,
        }
    }
}

#[derive(Debug)]
struct LinkedList<T> {
    length: u32,
    start: Option<NonNull<Node<T>>>,
    end: Option<NonNull<Node<T>>>,
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            length: 0,
            start: None,
            end: None,
        }
    }

    pub fn add(&mut self, obj: T) {
        let mut node = Box::new(Node::new(obj));
        node.next = None;
        // 安全：Box::into_raw 返回非空指针
        let node_ptr = Some(unsafe { NonNull::new_unchecked(Box::into_raw(node)) });
        match self.end {
            None => self.start = node_ptr,
            Some(end_ptr) => unsafe {
                (*end_ptr.as_ptr()).next = node_ptr;
            },
        }
        self.end = node_ptr;
        self.length += 1;
    }

    pub fn get(&mut self, index: i32) -> Option<&T> {
        self.get_ith_node(self.start, index)
    }

    fn get_ith_node(&mut self, node: Option<NonNull<Node<T>>>, index: i32) -> Option<&T> {
        match node {
            None => None,
            Some(next_ptr) => match index {
                0 => Some(unsafe { &(*next_ptr.as_ptr()).val }),
                _ => self.get_ith_node(unsafe { (*next_ptr.as_ptr()).next }, index - 1),
            },
        }
    }

    // 合并两个有序单链表为一个新的有序单链表
    pub fn merge(mut list_a: LinkedList<T>, mut list_b: LinkedList<T>) -> Self
    where
        T: Ord, // 要求T实现比较特性
    {
        let mut merged = LinkedList::new();
        // 初始化两个指针，指向各自链表的头节点
        let mut ptr_a = list_a.start;
        let mut ptr_b = list_b.start;

        // 循环比较两个链表的当前节点
        while let (Some(a), Some(b)) = (ptr_a, ptr_b) {
            let a_val = unsafe { &(*a.as_ptr()).val };
            let b_val = unsafe { &(*b.as_ptr()).val };

            if a_val <= b_val {
                // 取a节点，添加到新链表
                let next_a = unsafe { (*a.as_ptr()).next };
                merged.append_node(a);
                ptr_a = next_a;
            } else {
                // 取b节点，添加到新链表
                let next_b = unsafe { (*b.as_ptr()).next };
                merged.append_node(b);
                ptr_b = next_b;
            }
        }

        // 处理list_a的剩余节点
        while let Some(a) = ptr_a {
            let next_a = unsafe { (*a.as_ptr()).next };
            merged.append_node(a);
            ptr_a = next_a;
        }

        // 处理list_b的剩余节点
        while let Some(b) = ptr_b {
            let next_b = unsafe { (*b.as_ptr()).next };
            merged.append_node(b);
            ptr_b = next_b;
        }

        // 清空原链表的指针（避免悬垂指针）
        list_a.start = None;
        list_a.end = None;
        list_b.start = None;
        list_b.end = None;

        merged
    }

    // 辅助方法：将一个节点追加到链表尾部（直接复用节点指针，避免复制值）
    fn append_node(&mut self, node: NonNull<Node<T>>) {
        // 先将节点的next置为None（避免携带原链表的后续节点）
        unsafe {
            (*node.as_ptr()).next = None;
        }

        match self.end {
            None => {
                self.start = Some(node);
            }
            Some(end_ptr) => {
                unsafe {
                    (*end_ptr.as_ptr()).next = Some(node);
                }
            }
        }

        self.end = Some(node);
        self.length += 1;
    }
}

impl<T> Display for LinkedList<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.start {
            Some(node) => write!(f, "{}", unsafe { node.as_ref() }),
            None => Ok(()),
        }
    }
}

impl<T> Display for Node<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.next {
            Some(node) => write!(f, "{}, {}", self.val, unsafe { node.as_ref() }),
            None => write!(f, "{}", self.val),
        }
    }
}

// 实现Drop trait，避免内存泄漏
impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        let mut current = self.start;
        while let Some(node) = current {
            current = unsafe { (*node.as_ptr()).next };
            // 安全：将NonNull转换回Box，自动释放内存
            let _ = unsafe { Box::from_raw(node.as_ptr()) };
        }
    }
}

#[cfg(test)]
mod tests {
    use super::LinkedList;

    #[test]
    fn create_numeric_list() {
        let mut list = LinkedList::<i32>::new();
        list.add(1);
        list.add(2);
        list.add(3);
        println!("Linked List is {}", list);
        assert_eq!(3, list.length);
    }

    #[test]
    fn create_string_list() {
        let mut list_str = LinkedList::<String>::new();
        list_str.add("A".to_string());
        list_str.add("B".to_string());
        list_str.add("C".to_string());
        println!("Linked List is {}", list_str);
        assert_eq!(3, list_str.length);
    }

    #[test]
    fn test_merge_linked_list_1() {
        let mut list_a = LinkedList::<i32>::new();
        let mut list_b = LinkedList::<i32>::new();
        let vec_a = vec![1, 3, 5, 7];
        let vec_b = vec![2, 4, 6, 8];
        let target_vec = vec![1, 2, 3, 4, 5, 6, 7, 8];

        for &i in &vec_a {
            list_a.add(i);
        }
        for &i in &vec_b {
            list_b.add(i);
        }

        println!("list a {} list b {}", list_a, list_b);
        let mut list_c = LinkedList::<i32>::merge(list_a, list_b);
        println!("merged List is {}", list_c);

        for i in 0..target_vec.len() {
            assert_eq!(target_vec[i], *list_c.get(i as i32).unwrap());
        }
    }

    #[test]
    fn test_merge_linked_list_2() {
        let mut list_a = LinkedList::<i32>::new();
        let mut list_b = LinkedList::<i32>::new();
        let vec_a = vec![11, 33, 44, 88, 89, 90, 100];
        let vec_b = vec![1, 22, 30, 45];
        let target_vec = vec![1, 11, 22, 30, 33, 44, 45, 88, 89, 90, 100];

        for &i in &vec_a {
            list_a.add(i);
        }
        for &i in &vec_b {
            list_b.add(i);
        }

        println!("list a {} list b {}", list_a, list_b);
        let mut list_c = LinkedList::<i32>::merge(list_a, list_b);
        println!("merged List is {}", list_c);

        for i in 0..target_vec.len() {
            assert_eq!(target_vec[i], *list_c.get(i as i32).unwrap());
        }
    }
}