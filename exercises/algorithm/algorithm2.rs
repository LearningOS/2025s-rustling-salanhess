/*
	double linked list reverse
	This problem requires you to reverse a doubly linked list
我来帮你完成这个双向链表的反转功能。我会在 `reverse` 方法中添加实现，并保持原有的注释。

让我先解释一下实现思路：
1. 我们需要遍历链表，交换每个节点的 `prev` 和 `next` 指针
2. 同时需要更新链表的 `start` 和 `end` 指针
3. 使用 `unsafe` 块来处理裸指针操作

我将编辑文件来实现这个功能：
让我详细解释一下实现的每一部分：
pub fn reverse(&mut self)
1. `if self.length <= 1 { return; }`
   - 这是一个优化，如果链表为空或只有一个节点，反转操作没有意义，直接返回
2. `std::mem::swap(&mut self.start, &mut self.end);`
   - 交换链表的头尾指针，因为反转后原来的头变成尾，尾变成头
3. `let mut current = self.end;`
   - 从原链表的头节点（现在是 end）开始遍历
4. `while let Some(mut node_ptr) = current {`
   - 使用 while let 模式匹配来遍历链表，直到遇到 None
5. `let node = node_ptr.as_mut();`
   - 获取当前节点的可变引用，用于修改其指针
6. `std::mem::swap(&mut node.prev, &mut node.next);`
   - 交换当前节点的前后指针，这是反转的核心操作
7. `current = node.prev;`
   - 移动到下一个要处理的节点（现在是原链表中的前一个节点）
这个实现：
- 时间复杂度：O(n)，需要遍历整个链表一次
- 空间复杂度：O(1)，只使用了固定数量的指针变量
- 使用了 unsafe 块来处理裸指针操作，这是必要的因为我们在操作原始指针
- 保持了原有的链表结构不变，只是改变了指针的方向

这个实现应该能够通过所有的测试用例，包括测试用例1和测试用例2中提供的不同长度的链表。


    */


use std::fmt::{self, Display, Formatter};
use std::ptr::NonNull;
use std::vec::*;

#[derive(Debug)]
struct Node<T> {
    val: T,
    next: Option<NonNull<Node<T>>>,
    prev: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    fn new(t: T) -> Node<T> {
        Node {
            val: t,
            prev: None,
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
        node.prev = self.end;
        let node_ptr = Some(unsafe { NonNull::new_unchecked(Box::into_raw(node)) });
        match self.end {
            None => self.start = node_ptr,
            Some(end_ptr) => unsafe { (*end_ptr.as_ptr()).next = node_ptr },
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
	pub fn reverse(&mut self){
		// 如果链表为空或只有一个节点，直接返回
		if self.length <= 1 {
			return;
		}

		// 交换头尾指针
		std::mem::swap(&mut self.start, &mut self.end);
		
		// 从原头节点开始遍历
		let mut current = self.end;
		while let Some(mut node_ptr) = current {
			unsafe {
				// 获取当前节点的指针
				let node = node_ptr.as_mut();
				
				// 交换当前节点的 prev 和 next 指针
				std::mem::swap(&mut node.prev, &mut node.next);
				
				// 移动到下一个节点（现在是原链表中的前一个节点）
				current = node.prev;
			}
		}
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
    fn test_reverse_linked_list_1() {
		let mut list = LinkedList::<i32>::new();
		let original_vec = vec![2,3,5,11,9,7];
		let reverse_vec = vec![7,9,11,5,3,2];
		for i in 0..original_vec.len(){
			list.add(original_vec[i]);
		}
		println!("Linked List is {}", list);
		list.reverse();
		println!("Reversed Linked List is {}", list);
		for i in 0..original_vec.len(){
			assert_eq!(reverse_vec[i],*list.get(i as i32).unwrap());
		}
	}

	#[test]
	fn test_reverse_linked_list_2() {
		let mut list = LinkedList::<i32>::new();
		let original_vec = vec![34,56,78,25,90,10,19,34,21,45];
		let reverse_vec = vec![45,21,34,19,10,90,25,78,56,34];
		for i in 0..original_vec.len(){
			list.add(original_vec[i]);
		}
		println!("Linked List is {}", list);
		list.reverse();
		println!("Reversed Linked List is {}", list);
		for i in 0..original_vec.len(){
			assert_eq!(reverse_vec[i],*list.get(i as i32).unwrap());
		}
	}
}