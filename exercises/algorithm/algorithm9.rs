/*
	heap
	This question requires you to implement a binary heap function
*/

/*
【prompt】  先分析应该如何处理，然后结合mod tests 进行验证无误后再修改
让我分析一下这个堆(Heap)的实现问题：

1. 首先，这是一个需要实现二叉堆的练习，主要需要完成以下几个关键方法：
   - `add()`: 添加元素到堆中
   - `smallest_child_idx()`: 找到最小子节点的索引
   - `next()`: 实现迭代器接口，用于取出堆顶元素

2. 从测试用例可以看出：
   - 需要支持最小堆(MinHeap)和最大堆(MaxHeap)
   - 测试用例验证了空堆、最小堆和最大堆的基本操作
   - 主要测试了添加元素和取出元素的功能

3. 实现思路：
   - `add()`方法需要：
     1. 将新元素添加到数组末尾
     2. 向上调整(上浮)以维持堆的性质
   - `smallest_child_idx()`需要：
     1. 比较左右子节点
     2. 返回较小的那个节点的索引
   - `next()`方法需要：
     1. 取出堆顶元素
     2. 将最后一个元素移到堆顶
     3. 向下调整(下沉)以维持堆的性质

让我们先实现这些方法：
现在让我解释一下实现的细节：

1. `add()`方法：
   - 将新元素添加到数组末尾
   - 从新元素位置开始向上调整
   - 使用`comparator`函数比较父子节点，如果子节点更优则交换
   - 直到到达根节点或不需要交换为止

2. `smallest_child_idx()`方法：
   - 获取左右子节点的索引
   - 如果右子节点不存在，返回左子节点
   - 否则比较两个子节点，返回更优的那个

3. `next()`方法：
   - 如果堆为空，返回None
   - 取出堆顶元素（索引1）
   - 将最后一个元素移到堆顶
   - 从堆顶开始向下调整
   - 使用`comparator`函数比较父子节点，如果子节点更优则交换
   - 直到到达叶子节点或不需要交换为止

这个实现应该能够通过所有测试用例：
- 空堆测试：`next()`会返回None
- 最小堆测试：使用`a < b`比较器，确保最小值在堆顶
- 最大堆测试：使用`a > b`比较器，确保最大值在堆顶

实现遵循了二叉堆的基本性质：
- 完全二叉树结构
- 堆序性质（父节点优于子节点）
- 高效的插入和删除操作（O(log n)）

您可以运行测试来验证实现是否正确。如果有任何问题，我们可以进一步调整。

*/

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
            items: vec![T::default()],
            comparator,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn add(&mut self, value: T) {
        self.items.push(value);
        self.count += 1;
        let mut idx = self.count;
        while idx > 1 {
            let parent_idx = self.parent_idx(idx);
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

    fn smallest_child_idx(&self, idx: usize) -> usize {
        let left_idx = self.left_child_idx(idx);
        let right_idx = self.right_child_idx(idx);
        
        if right_idx > self.count {
            left_idx
        } else {
            if (self.comparator)(&self.items[left_idx], &self.items[right_idx]) {
                left_idx
            } else {
                right_idx
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
    T: Default,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.count == 0 {
            return None;
        }
        
        let result = self.items.swap_remove(1);
        self.count -= 1;
        
        if self.count > 0 {
            let mut idx = 1;
            while self.children_present(idx) {
                let smallest_child = self.smallest_child_idx(idx);
                if (self.comparator)(&self.items[smallest_child], &self.items[idx]) {
                    self.items.swap(smallest_child, idx);
                    idx = smallest_child;
                } else {
                    break;
                }
            }
        }
        
        Some(result)
    }
}

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
    }
}