use std::cmp::Ord;
use std::default::Default;

pub struct Heap<T>
where
    T: Default,
{
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T>
where
    T: Default,
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            items: vec![T::default()], // 索引从1开始，0位置占位
            comparator,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    // 插入元素并上浮调整堆
    pub fn add(&mut self, value: T) {
        self.count += 1;
        self.items.push(value);
        // 从最后一个元素的索引开始上浮
        let mut idx = self.count;
        while idx > 1 {
            let parent_idx = self.parent_idx(idx);
            // 若当前元素与父节点不满足堆的特性，则交换
            if (self.comparator)(&self.items[idx], &self.items[parent_idx]) {
                self.items.swap(idx, parent_idx);
                idx = parent_idx;
            } else {
                break;
            }
        }
    }

    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
    }

    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) <= self.count
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }

    // 根据比较器选择符合条件的子节点索引（最小堆选更小的，最大堆选更大的）
    fn smallest_child_idx(&self, idx: usize) -> usize {
        let left_idx = self.left_child_idx(idx);
        let right_idx = self.right_child_idx(idx);

        // 若右子节点存在，比较左右子节点，返回符合条件的
        if right_idx <= self.count {
            if (self.comparator)(&self.items[right_idx], &self.items[left_idx]) {
                right_idx
            } else {
                left_idx
            }
        } else {
            // 只有左子节点，返回左子节点
            left_idx
        }
    }

    // 下沉调整堆：从idx开始，与子节点比较并交换
    fn bubble_down(&mut self, mut idx: usize) {
        while self.children_present(idx) {
            let child_idx = self.smallest_child_idx(idx);
            // 若当前元素与子节点不满足堆的特性，则交换
            if (self.comparator)(&self.items[child_idx], &self.items[idx]) {
                self.items.swap(idx, child_idx);
                idx = child_idx;
            } else {
                break;
            }
        }
    }
}

impl<T> Heap<T>
where
    T: Default + Ord,
{
    /// Create a new MinHeap
    pub fn new_min() -> Self {
        Self::new(|a, b| a < b)
    }

    /// Create a new MaxHeap
    pub fn new_max() -> Self {
        Self::new(|a, b| a > b)
    }
}

impl<T> Iterator for Heap<T>
where
    T: Default + Clone, // 若T不实现Copy，需Clone（或使用take，这里用Clone更通用）
{
    type Item = T;

    // 弹出堆顶元素并下沉调整堆
    fn next(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        // 取出堆顶元素（索引1）
        let top = self.items[1].clone();
        // 将最后一个元素移到堆顶
        self.items[1] = self.items[self.count].clone();
        self.count -= 1;
        self.items.pop(); // 移除最后一个元素（已移到堆顶）
        // 从堆顶开始下沉调整
        self.bubble_down(1);

        Some(top)
    }
}

// 为了兼容测试用例，保留MinHeap/MaxHeap的构造器
pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a > b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_heap() {
        let mut heap = MaxHeap::new::<i32>();
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_min_heap() {
        let mut heap = MinHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(4));
        assert_eq!(heap.next(), Some(9));
        heap.add(1);
        assert_eq!(heap.next(), Some(1));
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_max_heap() {
        let mut heap = MaxHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), Some(9));
        assert_eq!(heap.next(), Some(4));
        heap.add(1);
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(1));
        assert_eq!(heap.next(), None);
    }
}