// 通用排序函数（归并排序实现）
fn sort<T: Ord + Clone>(array: &mut [T]) {
    // 基线条件：空切片或单元素切片无需排序
    if array.len() <= 1 {
        return;
    }

    // 分割切片为左右两部分
    let mid = array.len() / 2;
    let (left, right) = array.split_at_mut(mid);

    // 递归排序左右部分
    sort(left);
    sort(right);

    // 合并两个有序的子切片
    let mut merged = Vec::with_capacity(array.len());
    let mut i = 0;
    let mut j = 0;

    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            merged.push(left[i].clone());
            i += 1;
        } else {
            merged.push(right[j].clone());
            j += 1;
        }
    }

    // 追加剩余元素
    merged.extend_from_slice(&left[i..]);
    merged.extend_from_slice(&right[j..]);

    // 将合并后的结果复制回原切片
    array.clone_from_slice(&merged);
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