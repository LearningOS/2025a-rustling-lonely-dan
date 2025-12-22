// 通用排序函数（快速排序实现）
fn sort<T: Ord>(array: &mut [T]) {
    // 基线条件：空切片或单元素切片无需排序
    if array.len() <= 1 {
        return;
    }

    // 选择第一个元素作为基准（pivot）
    let pivot_idx = 0;
    let mut left = 1;
    let mut right = array.len() - 1;

    // 分区操作：将元素分为小于基准、等于基准、大于基准的三部分
    while left <= right {
        if array[left] <= array[pivot_idx] {
            // 左指针右移，找大于基准的元素
            left += 1;
        } else if array[right] > array[pivot_idx] {
            // 右指针左移，找小于等于基准的元素
            right -= 1;
        } else {
            // 交换左右指针指向的元素
            array.swap(left, right);
            left += 1;
            right -= 1;
        }
    }

    // 将基准元素交换到正确的位置（右指针的位置）
    array.swap(pivot_idx, right);

    // 递归排序左半部分（小于基准）和右半部分（大于基准）
    let (left_part, rest) = array.split_at_mut(right);
    sort(left_part);
    // rest 的第一个元素是基准，所以从第二个元素开始排序右半部分
    sort(&mut rest[1..]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }

    #[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }

    #[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }

    // 额外测试：空切片
    #[test]
    fn test_sort_empty() {
        let mut vec: Vec<i32> = vec![];
        sort(&mut vec);
        assert_eq!(vec, vec![]);
    }

    // 额外测试：字符串切片
    #[test]
    fn test_sort_strings() {
        let mut vec = vec!["banana", "apple", "cherry", "date"];
        sort(&mut vec);
        assert_eq!(vec, vec!["apple", "banana", "cherry", "date"]);
    }
}