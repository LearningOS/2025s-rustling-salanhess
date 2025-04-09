/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
我来帮你将排序算法改为插入排序。插入排序是一种简单直观的排序算法，它的工作原理是通过构建有序序列，对于未排序数据，在已排序序列中从后向前扫描，找到相应位置并插入。
让我详细解释一下插入排序的实现：

1. `fn sort<T: PartialOrd>(array: &mut [T])`
   - 函数签名保持不变，仍然需要 `PartialOrd` trait 来比较元素大小

2. `let len = array.len();`
   - 获取数组长度，用于控制循环

3. `if len <= 1 { return; }`
   - 优化：对于空数组或单元素数组，直接返回

4. 外层循环 `for i in 1..len`
   - 从第二个元素开始遍历（索引1）
   - 因为第一个元素默认是有序的

5. 内层循环 `for j in (0..i).rev()`
   - 从当前元素的前一个位置开始向前比较
   - 使用 `rev()` 实现从后向前的遍历
   - 范围是 `0..i` 因为 `i` 之前的元素都是已排序的

6. `if array[j] > array[j + 1]`
   - 比较当前元素和前一个元素
   - 如果前一个元素更大，需要交换位置

7. `else { break; }`
   - 优化：如果前一个元素小于等于当前元素
   - 说明已经找到正确位置，可以结束内层循环

插入排序的特点：
- 时间复杂度：
  - 最坏情况：O(n²)（数组完全逆序）
  - 最好情况：O(n)（数组已经有序）
  - 平均情况：O(n²)
- 空间复杂度：O(1)，原地排序
- 稳定性：是稳定的排序算法
- 适合场景：
  - 小规模数据
  - 基本有序的数据
  - 需要稳定排序的场景

这个实现应该能够通过所有的测试用例，并且相比冒泡排序，插入排序在某些情况下（特别是数据基本有序时）会有更好的性能。


    */


fn sort<T: PartialOrd>(array: &mut [T]) {
    // 获取数组长度
    let len = array.len();
    
    // 如果数组长度小于等于1，直接返回
    if len <= 1 {
        return;
    }

    // 从第二个元素开始遍历（索引1）
    for i in 1..len {
        // 将当前元素插入到已排序部分的适当位置
        // 从当前元素的前一个位置开始向前比较
        for j in (0..i).rev() {
            // 如果前一个元素大于当前元素，则交换它们的位置
            if array[j] > array[j + 1] {
                array.swap(j, j + 1);
            } else {
                // 如果前一个元素小于等于当前元素，说明已经找到正确位置
                break;
            }
        }
    }
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
}