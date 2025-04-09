/*
	queue
	This question requires you to use queues to implement the functionality of the stac
*/
/*
我来帮你实现用队列模拟栈的功能。这是一个经典的面试题，我们需要使用两个队列来实现栈的先进后出特性。

让我编辑文件来实现这个功能：
让我详细解释一下实现：

1. 数据结构：
   ```rust
   pub struct myStack<T> {
       q1: Queue<T>,
       q2: Queue<T>
   }
   ```
   - 使用两个队列来模拟栈
   - 任何时候只有一个队列存储数据，另一个队列用于辅助操作

2. `push` 方法：
   pub fn push(&mut self, elem: T) {
   - 将新元素入队到非空队列
   - 如果两个队列都为空，默认入队到 q1

3. `pop` 方法：
   pub fn pop(&mut self) -> Result<T, &str> 
   - 找到非空队列和空队列
   - 将非空队列中的元素（除了最后一个）转移到空队列
   - 返回最后一个元素（栈顶元素）

4. `is_empty` 方法：
   ```rust
   pub fn is_empty(&self) -> bool {
       self.q1.is_empty() && self.q2.is_empty()
   }
   ```
   - 两个队列都为空时，栈为空

实现特点：
- 时间复杂度：
  - push: O(1)
  - pop: O(n)，需要转移 n-1 个元素
- 空间复杂度：O(n)，使用两个队列存储元素
- 使用两个队列实现栈的先进后出特性

这个实现应该能够通过测试用例：
1. 测试空栈的 pop 操作
2. 测试基本的 push 和 pop 操作
3. 测试栈的 isEmpty 功能
4. 测试连续 push 和 pop 操作

注意事项：
1. 使用两个队列来模拟栈的先进后出特性
2. 在 pop 操作时，需要将元素从一个队列转移到另一个队列
3. 保持队列的先进先出特性，通过转移元素来实现栈的先进后出
4. 使用 Result 类型处理错误情况

*/

#[derive(Debug)]
pub struct Queue<T> {
    elements: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }

    pub fn enqueue(&mut self, value: T) {
        self.elements.push(value)
    }

    pub fn dequeue(&mut self) -> Result<T, &str> {
        if !self.elements.is_empty() {
            Ok(self.elements.remove(0usize))
        } else {
            Err("Queue is empty")
        }
    }

    pub fn peek(&self) -> Result<&T, &str> {
        match self.elements.first() {
            Some(value) => Ok(value),
            None => Err("Queue is empty"),
        }
    }

    pub fn size(&self) -> usize {
        self.elements.len()
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}

impl<T> Default for Queue<T> {
    fn default() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }
}

pub struct myStack<T>
{
	// 使用两个队列来模拟栈
	q1: Queue<T>,
	q2: Queue<T>
}

impl<T> myStack<T> {
    pub fn new() -> Self {
        Self {
			q1: Queue::<T>::new(),
			q2: Queue::<T>::new()
        }
    }

    pub fn push(&mut self, elem: T) {
        // 将新元素入队到非空队列
        // 如果两个队列都为空，默认入队到q1
        if !self.q1.is_empty() {
            self.q1.enqueue(elem);
        } else {
            self.q2.enqueue(elem);
        }
    }

    pub fn pop(&mut self) -> Result<T, &str> {
        // 如果两个队列都为空，返回错误
        if self.is_empty() {
            return Err("Stack is empty");
        }

        // 找到非空队列和空队列
        let (full, empty) = if !self.q1.is_empty() {
            (&mut self.q1, &mut self.q2)
        } else {
            (&mut self.q2, &mut self.q1)
        };

        // 将非空队列中的元素（除了最后一个）转移到空队列
        while full.size() > 1 {
            if let Ok(elem) = full.dequeue() {
                empty.enqueue(elem);
            }
        }

        // 返回最后一个元素
        full.dequeue()
    }

    pub fn is_empty(&self) -> bool {
        // 两个队列都为空时，栈为空
        self.q1.is_empty() && self.q2.is_empty()
    }
}

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn test_queue(){
		let mut s = myStack::<i32>::new();
		assert_eq!(s.pop(), Err("Stack is empty"));
        s.push(1);
        s.push(2);
        s.push(3);
        assert_eq!(s.pop(), Ok(3));
        assert_eq!(s.pop(), Ok(2));
        s.push(4);
        s.push(5);
        assert_eq!(s.is_empty(), false);
        assert_eq!(s.pop(), Ok(5));
        assert_eq!(s.pop(), Ok(4));
        assert_eq!(s.pop(), Ok(1));
        assert_eq!(s.pop(), Err("Stack is empty"));
        assert_eq!(s.is_empty(), true);
	}
}