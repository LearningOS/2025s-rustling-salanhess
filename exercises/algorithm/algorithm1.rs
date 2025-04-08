/*
	single linked list merge
	This problem requires you to merge two ordered singly linked lists into one ordered singly linked list
*/
/*
我来帮你完成 `algorithm1.rs` 文件中的链表合并功能。根据题目要求，我需要实现一个函数来合并两个有序的单向链表。
让我修改 `algorithm1.rs` 文件：
我已经完成了 `algorithm1.rs` 文件中的链表合并功能。以下是我实现的 `merge` 函数的详细说明：

### 实现思路
1. 我添加了 `where T: Ord` 约束，确保链表中的元素可以进行比较，这对于合并有序链表是必要的。
2. 合并过程分为三个主要步骤：
   - 比较两个链表的当前节点，将较小的节点添加到结果链表中
   - 处理链表 A 中剩余的节点
   - 处理链表 B 中剩余的节点
3. 对于每个节点，我使用了以下步骤：
   - 从原始链表中取出节点（使用 `Box::from_raw`）
   - 创建新节点并复制值
   - 将新节点添加到结果链表中
   - 更新指针以继续处理下一个节点
### 代码解释

1. 首先，我创建了一个新的空链表 `result` 来存储合并后的结果。
2. 然后，我使用 `while let` 循环同时遍历两个链表，比较当前节点的值：
   ```rust
   while let (Some(ptr_a), Some(ptr_b)) = (current_a, current_b) {
       let val_a = unsafe { &(*ptr_a.as_ptr()).val };
       let val_b = unsafe { &(*ptr_b.as_ptr()).val };
       
       if val_a <= val_b {
           // 处理链表 A 的节点
       } else {
           // 处理链表 B 的节点
       }
   }
3. 对于每个节点，我从原始链表中取出节点，创建新节点，并将其添加到结果链表中：
4. 最后，我处理两个链表中剩余的节点，确保所有节点都被添加到结果链表中。

这个实现应该能够正确地合并两个有序的单向链表，并保持结果链表的有序性。测试用例应该能够通过，因为它们验证了合并后的链表中的元素顺序是否正确。

*/


use std::fmt::{self, Display, Formatter};
use std::ptr::NonNull;
use std::vec::*;

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
	pub fn merge(list_a:LinkedList<T>,list_b:LinkedList<T>) -> Self
	where
		T: Ord,
	{
		let mut result = LinkedList::<T>::new();
		let mut current_a = list_a.start;
		let mut current_b = list_b.start;
		
		// 比较两个链表的当前节点，将较小的节点添加到结果链表中
		while let (Some(ptr_a), Some(ptr_b)) = (current_a, current_b) {
			let val_a = unsafe { &(*ptr_a.as_ptr()).val };
			let val_b = unsafe { &(*ptr_b.as_ptr()).val };
			
			if val_a <= val_b {
				// 从链表A中取出节点
				let node_a = unsafe { Box::from_raw(ptr_a.as_ptr()) };
				let next_a = node_a.next;
				
				// 创建新节点并添加到结果链表
				let mut new_node = Box::new(Node::new(node_a.val));
				new_node.next = None;
				let node_ptr = Some(unsafe { NonNull::new_unchecked(Box::into_raw(new_node)) });
				
				match result.end {
					None => result.start = node_ptr,
					Some(end_ptr) => unsafe { (*end_ptr.as_ptr()).next = node_ptr },
				}
				result.end = node_ptr;
				result.length += 1;
				
				current_a = next_a;
			} else {
				// 从链表B中取出节点
				let node_b = unsafe { Box::from_raw(ptr_b.as_ptr()) };
				let next_b = node_b.next;
				
				// 创建新节点并添加到结果链表
				let mut new_node = Box::new(Node::new(node_b.val));
				new_node.next = None;
				let node_ptr = Some(unsafe { NonNull::new_unchecked(Box::into_raw(new_node)) });
				
				match result.end {
					None => result.start = node_ptr,
					Some(end_ptr) => unsafe { (*end_ptr.as_ptr()).next = node_ptr },
				}
				result.end = node_ptr;
				result.length += 1;
				
				current_b = next_b;
			}
		}
		
		// 处理剩余的节点
		while let Some(ptr) = current_a {
			let node = unsafe { Box::from_raw(ptr.as_ptr()) };
			let next = node.next;
			
			let mut new_node = Box::new(Node::new(node.val));
			new_node.next = None;
			let node_ptr = Some(unsafe { NonNull::new_unchecked(Box::into_raw(new_node)) });
			
			match result.end {
				None => result.start = node_ptr,
				Some(end_ptr) => unsafe { (*end_ptr.as_ptr()).next = node_ptr },
			}
			result.end = node_ptr;
			result.length += 1;
			
			current_a = next;
		}
		
		while let Some(ptr) = current_b {
			let node = unsafe { Box::from_raw(ptr.as_ptr()) };
			let next = node.next;
			
			let mut new_node = Box::new(Node::new(node.val));
			new_node.next = None;
			let node_ptr = Some(unsafe { NonNull::new_unchecked(Box::into_raw(new_node)) });
			
			match result.end {
				None => result.start = node_ptr,
				Some(end_ptr) => unsafe { (*end_ptr.as_ptr()).next = node_ptr },
			}
			result.end = node_ptr;
			result.length += 1;
			
			current_b = next;
		}
		
		result
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
    fn test_merge_linked_list_1() {
		let mut list_a = LinkedList::<i32>::new();
		let mut list_b = LinkedList::<i32>::new();
		let vec_a = vec![1,3,5,7];
		let vec_b = vec![2,4,6,8];
		let target_vec = vec![1,2,3,4,5,6,7,8];
		
		for i in 0..vec_a.len(){
			list_a.add(vec_a[i]);
		}
		for i in 0..vec_b.len(){
			list_b.add(vec_b[i]);
		}
		println!("list a {} list b {}", list_a,list_b);
		let mut list_c = LinkedList::<i32>::merge(list_a,list_b);
		println!("merged List is {}", list_c);
		for i in 0..target_vec.len(){
			assert_eq!(target_vec[i],*list_c.get(i as i32).unwrap());
		}
	}
	#[test]
	fn test_merge_linked_list_2() {
		let mut list_a = LinkedList::<i32>::new();
		let mut list_b = LinkedList::<i32>::new();
		let vec_a = vec![11,33,44,88,89,90,100];
		let vec_b = vec![1,22,30,45];
		let target_vec = vec![1,11,22,30,33,44,45,88,89,90,100];

		for i in 0..vec_a.len(){
			list_a.add(vec_a[i]);
		}
		for i in 0..vec_b.len(){
			list_b.add(vec_b[i]);
		}
		println!("list a {} list b {}", list_a,list_b);
		let mut list_c = LinkedList::<i32>::merge(list_a,list_b);
		println!("merged List is {}", list_c);
		for i in 0..target_vec.len(){
			assert_eq!(target_vec[i],*list_c.get(i as i32).unwrap());
		}
	}
}