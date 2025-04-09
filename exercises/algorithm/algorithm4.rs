/*
	binary_search tree
	This problem requires you to implement a basic interface for a binary tree
*/

/*
我来帮你实现二叉搜索树（BST）的插入和搜索功能。让我先解释一下实现思路：

1. 插入操作需要遵循BST的性质：左子树的值小于根节点，右子树的值大于根节点
2. 搜索操作需要递归地比较值，直到找到目标或到达叶子节点

让我编辑文件来实现这些功能：
    // Insert a value into the BST
    // Search for a value in the BST
让我详细解释一下实现的每一部分：

1. `BinarySearchTree` 的 `insert` 方法：
   - 首先检查树是否为空
   - 如果为空，创建新的根节点
   - 否则调用根节点的 `insert` 方法进行递归插入

2. `BinarySearchTree` 的 `search` 方法：
   - 使用迭代方式实现搜索
   - 从根节点开始，根据比较结果决定搜索方向
   - 使用 `Ordering` 枚举来处理三种比较结果
   - 时间复杂度：O(h)，其中 h 是树的高度

3. `TreeNode` 的 `insert` 方法：
   - 使用递归方式实现插入
   - 根据比较结果决定插入位置
   - 如果值相等，直接返回（不允许重复值）
   - 如果值小于当前节点，插入左子树
   - 如果值大于当前节点，插入右子树

实现的特点：
- 插入操作：
  - 时间复杂度：平均 O(log n)，最坏 O(n)（当树退化为链表时）
  - 空间复杂度：平均 O(log n)，最坏 O(n)
  - 保持BST的性质：左子树 < 根节点 < 右子树

- 搜索操作：
  - 时间复杂度：平均 O(log n)，最坏 O(n)
  - 空间复杂度：O(1)（使用迭代实现）
  - 使用 `Ordering` 枚举进行优雅的比较

这个实现应该能够通过所有的测试用例：
- test_insert_and_search：测试基本的插入和搜索功能
- test_insert_duplicate：测试重复值的处理

注意事项：
1. 不允许插入重复值
2. 保持BST的性质
3. 使用迭代实现搜索，避免递归带来的栈溢出风险
4. 使用 `Ordering` 枚举进行清晰的比较逻辑

*/
use std::cmp::Ordering;
use std::fmt::Debug;


#[derive(Debug)]
struct TreeNode<T>
where
    T: Ord,
{
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

#[derive(Debug)]
struct BinarySearchTree<T>
where
    T: Ord,
{
    root: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }
}

impl<T> BinarySearchTree<T>
where
    T: Ord,
{

    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    // Insert a value into the BST
    fn insert(&mut self, value: T) {
        // 如果树为空，创建新的根节点
        if self.root.is_none() {
            self.root = Some(Box::new(TreeNode::new(value)));
            return;
        }
        
        // 否则从根节点开始递归插入
        if let Some(ref mut root) = self.root {
            root.insert(value);
        }
    }

    // Search for a value in the BST
    fn search(&self, value: T) -> bool {
        // 如果树为空，直接返回false
        if self.root.is_none() {
            return false;
        }
        
        // 从根节点开始搜索
        let mut current = &self.root;
        while let Some(node) = current {
            match value.cmp(&node.value) {
                // 如果找到相等的值，返回true
                Ordering::Equal => return true,
                // 如果目标值小于当前节点值，搜索左子树
                Ordering::Less => current = &node.left,
                // 如果目标值大于当前节点值，搜索右子树
                Ordering::Greater => current = &node.right,
            }
        }
        // 如果遍历完整个树都没找到，返回false
        false
    }
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    // Insert a node into the tree
    fn insert(&mut self, value: T) {
        match value.cmp(&self.value) {
            // 如果值相等，不插入（BST不允许重复值）
            Ordering::Equal => return,
            // 如果值小于当前节点值，插入左子树
            Ordering::Less => {
                if let Some(ref mut left) = self.left {
                    left.insert(value);
                } else {
                    self.left = Some(Box::new(TreeNode::new(value)));
                }
            },
            // 如果值大于当前节点值，插入右子树
            Ordering::Greater => {
                if let Some(ref mut right) = self.right {
                    right.insert(value);
                } else {
                    self.right = Some(Box::new(TreeNode::new(value)));
                }
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_search() {
        let mut bst = BinarySearchTree::new();

        
        assert_eq!(bst.search(1), false);

        
        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(2);
        bst.insert(4);

        
        assert_eq!(bst.search(5), true);
        assert_eq!(bst.search(3), true);
        assert_eq!(bst.search(7), true);
        assert_eq!(bst.search(2), true);
        assert_eq!(bst.search(4), true);

        
        assert_eq!(bst.search(1), false);
        assert_eq!(bst.search(6), false);
    }

    #[test]
    fn test_insert_duplicate() {
        let mut bst = BinarySearchTree::new();

        
        bst.insert(1);
        bst.insert(1);

        
        assert_eq!(bst.search(1), true);

        
        match bst.root {
            Some(ref node) => {
                assert!(node.left.is_none());
                assert!(node.right.is_none());
            },
            None => panic!("Root should not be None after insertion"),
        }
    }
}    


