/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
我来帮你实现冒泡排序算法。冒泡排序是一种简单的排序算法，通过重复地遍历要排序的数组，比较相邻元素并交换它们的位置（如果顺序错误）。

让我编辑文件来实现这个功能：
让我详细解释一下实现的每一部分：

1. `fn sort<T: PartialOrd>(array: &mut [T])`
   - 函数签名中添加了 `PartialOrd` trait 约束，因为我们需要比较元素的大小
   - 参数 `array` 是一个可变切片，允许我们修改数组内容

2. `let len = array.len();`
   - 获取数组的长度，用于控制循环次数

3. `if len <= 1 { return; }`
   - 优化：如果数组长度为0或1，不需要排序，直接返回

4. 外层循环 `for i in 0..len`
   - 控制排序的轮数
   - 每轮排序会将当前未排序部分的最大元素移动到正确位置

5. 内层循环 `for j in 0..len - i - 1`
   - 比较相邻元素
   - `len - i - 1` 是因为每轮排序后，最后 i 个元素已经是有序的

6. `if array[j] > array[j + 1]`
   - 比较相邻元素的大小
   - 如果前一个元素大于后一个元素，需要交换它们的位置

7. `array.swap(j, j + 1);`
   - 使用 Rust 的 `swap` 方法交换两个元素的位置

这个实现的特点：
- 时间复杂度：O(n²)，最坏和平均情况下都是 n²
- 空间复杂度：O(1)，原地排序，不需要额外的存储空间
- 稳定性：是稳定的排序算法，相等元素的相对位置不会改变
- 优化：添加了长度检查，避免不必要的排序操作

这个实现应该能够通过所有的测试用例：
- test_sort_1：测试一般情况下的排序
- test_sort_2：测试单个元素的数组
- test_sort_3：测试降序数组的排序


    */


fn sort<T: PartialOrd>(array: &mut [T]) {
    // 获取数组长度
    let len = array.len();
    
    // 如果数组长度小于等于1，直接返回
    if len <= 1 {
        return;
    }

    // 外层循环：控制排序轮数
    for i in 0..len {
        // 内层循环：比较相邻元素
        // 每轮排序后，最大的元素会被移动到末尾，所以可以减少比较次数
        for j in 0..len - i - 1 {
            // 如果前一个元素大于后一个元素，则交换它们的位置
            if array[j] > array[j + 1] {
                array.swap(j, j + 1);
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