/*
	stack
	This question requires you to use a stack to achieve a bracket match
*/

/*
我来帮你实现栈的弹出操作和括号匹配功能。让我先完成栈的 `pop` 方法，然后实现括号匹配算法。

让我编辑文件来实现这些功能：
rust:exercises/algorithm/algorithm7.rs

fn bracket_match(bracket: &str) -> bool {

让我详细解释一下实现：

1. 栈的 `pop` 方法：
   fn pop(&mut self) -> Option<T>
   - 检查栈是否为空
   - 更新栈大小
   - 使用 `Vec` 的 `pop` 方法移除并返回最后一个元素

2. 括号匹配算法：
   fn bracket_match(bracket: &str) -> bool
       let mut stack = Stack::new();
   - 使用栈来存储左括号
   - 遍历字符串中的每个字符
   - 使用 `match` 语句处理不同类型的括号

3. 括号匹配逻辑：
   - 左括号 `(`, `{`, `[`：直接入栈
   - 右括号 `)`, `}`, `]`：检查栈顶元素是否匹配
   - 其他字符：忽略

算法特点：
- 时间复杂度：O(n)，其中 n 是字符串长度
- 空间复杂度：O(n)，最坏情况下需要存储所有左括号
- 使用栈来保证括号的嵌套顺序
- 支持三种括号类型：圆括号、花括号和方括号

这个实现应该能够通过所有的测试用例：
1. `bracket_matching_1`：测试混合括号的正确匹配
2. `bracket_matching_2`：测试缺少右括号的情况
3. `bracket_matching_3`：测试嵌套括号的正确匹配
4. `bracket_matching_4`：测试括号类型不匹配的情况
5. `bracket_matching_5`：测试右括号过多的情况
6. `bracket_matching_6`：测试空字符串的情况

注意事项：
1. 使用 `Option` 类型处理栈为空的情况
2. 使用 `match` 语句优雅地处理不同类型的括号
3. 最后检查栈是否为空，确保没有未匹配的左括号
4. 忽略非括号字符，只处理括号匹配

*/
#[derive(Debug)]
struct Stack<T> {
	size: usize,
	data: Vec<T>,
}
impl<T> Stack<T> {
	fn new() -> Self {
		Self {
			size: 0,
			data: Vec::new(),
		}
	}
	fn is_empty(&self) -> bool {
		0 == self.size
	}
	fn len(&self) -> usize {
		self.size
	}
	fn clear(&mut self) {
		self.size = 0;
		self.data.clear();
	}
	fn push(&mut self, val: T) {
		self.data.push(val);
		self.size += 1;
	}
	fn pop(&mut self) -> Option<T> {
		// 如果栈为空，返回None
		if self.is_empty() {
			return None;
		}
		// 栈大小减1
		self.size -= 1;
		// 弹出最后一个元素
		self.data.pop()
	}
	fn peek(&self) -> Option<&T> {
		if 0 == self.size {
			return None;
		}
		self.data.get(self.size - 1)
	}
	fn peek_mut(&mut self) -> Option<&mut T> {
		if 0 == self.size {
			return None;
		}
		self.data.get_mut(self.size - 1)
	}
	fn into_iter(self) -> IntoIter<T> {
		IntoIter(self)
	}
	fn iter(&self) -> Iter<T> {
		let mut iterator = Iter { 
			stack: Vec::new() 
		};
		for item in self.data.iter() {
			iterator.stack.push(item);
		}
		iterator
	}
	fn iter_mut(&mut self) -> IterMut<T> {
		let mut iterator = IterMut { 
			stack: Vec::new() 
		};
		for item in self.data.iter_mut() {
			iterator.stack.push(item);
		}
		iterator
	}
}
struct IntoIter<T>(Stack<T>);
impl<T: Clone> Iterator for IntoIter<T> {
	type Item = T;
	fn next(&mut self) -> Option<Self::Item> {
		if !self.0.is_empty() {
			self.0.size -= 1;self.0.data.pop()
		} 
		else {
			None
		}
	}
}
struct Iter<'a, T: 'a> {
	stack: Vec<&'a T>,
}
impl<'a, T> Iterator for Iter<'a, T> {
	type Item = &'a T;
	fn next(&mut self) -> Option<Self::Item> {
		self.stack.pop()
	}
}
struct IterMut<'a, T: 'a> {
	stack: Vec<&'a mut T>,
}
impl<'a, T> Iterator for IterMut<'a, T> {
	type Item = &'a mut T;
	fn next(&mut self) -> Option<Self::Item> {
		self.stack.pop()
	}
}

fn bracket_match(bracket: &str) -> bool {
	// 创建一个栈来存储左括号
	let mut stack = Stack::new();
	
	// 遍历字符串中的每个字符
	for c in bracket.chars() {
		match c {
			// 遇到左括号，入栈
			'(' | '{' | '[' => stack.push(c),
			// 遇到右括号，检查是否匹配
			')' => {
				if stack.pop() != Some('(') {
					return false;
				}
			},
			'}' => {
				if stack.pop() != Some('{') {
					return false;
				}
			},
			']' => {
				if stack.pop() != Some('[') {
					return false;
				}
			},
			// 其他字符忽略
			_ => continue,
		}
	}
	
	// 如果栈为空，说明所有括号都匹配
	stack.is_empty()
}

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn bracket_matching_1(){
		let s = "(2+3){func}[abc]";
		assert_eq!(bracket_match(s),true);
	}
	#[test]
	fn bracket_matching_2(){
		let s = "(2+3)*(3-1";
		assert_eq!(bracket_match(s),false);
	}
	#[test]
	fn bracket_matching_3(){
		let s = "{{([])}}";
		assert_eq!(bracket_match(s),true);
	}
	#[test]
	fn bracket_matching_4(){
		let s = "{{(}[)]}";
		assert_eq!(bracket_match(s),false);
	}
	#[test]
	fn bracket_matching_5(){
		let s = "[[[]]]]]]]]]";
		assert_eq!(bracket_match(s),false);
	}
	#[test]
	fn bracket_matching_6(){
		let s = "";
		assert_eq!(bracket_match(s),true);
	}
}