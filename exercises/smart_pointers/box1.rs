// box1.rs
//
// At compile time, Rust needs to know how much space a type takes up. This
// becomes problematic for recursive types, where a value can have as part of
// itself another value of the same type. To get around the issue, we can use a
// `Box` - a smart pointer used to store data on the heap, which also allows us
// to wrap a recursive type.
//
// The recursive type we're implementing in this exercise is the `cons list` - a
// data structure frequently found in functional programming languages. Each
// item in a cons list contains two elements: the value of the current item and
// the next item. The last item is a value called `Nil`.
//
// Step 1: use a `Box` in the enum definition to make the code compile
// Step 2: create both empty and non-empty cons lists by replacing `todo!()`
//
// Note: the tests should not be changed
//
// Execute `rustlings hint box1` or use the `hint` watch subcommand for a hint.

/*
让我帮你完成这个关于 Box 智能指针的练习。这个练习实现了一个函数式编程中常见的数据结构：cons 列表。

问题的关键在于处理递归类型。在 `List` 枚举中，`Cons` 变体包含一个 `List` 类型的字段，
这会导致编译器无法确定类型的大小。我们需要使用 `Box` 来解决这个问题。

让我修改代码：

```rust:exercises/smart_pointers/box1.rs
#[derive(PartialEq, Debug)]
pub enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn main() {
    println!("This is an empty cons list: {:?}", create_empty_list());
    println!(
        "This is a non-empty cons list: {:?}",
        create_non_empty_list()
    );
}

pub fn create_empty_list() -> List {
    List::Nil
}

pub fn create_non_empty_list() -> List {
    List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))))
}
// ... existing code ...
```
让我解释一下这些修改：

1. 修改 `List` 枚举：
   - 将 `Cons(i32, List)` 改为 `Cons(i32, Box<List>)`
   - 使用 `Box` 将递归部分放在堆上，这样编译器就能知道类型的确切大小
   - `Box` 只占用一个指针的大小，指向堆上的实际数据

2. 实现 `create_empty_list()`：
   - 直接返回 `List::Nil`
   - 这代表一个空的 cons 列表

3. 实现 `create_non_empty_list()`：
   - 创建一个包含两个元素的列表：`[1, 2]`
   - 使用嵌套的 `Box::new()` 来构建链表结构
   - 最后以 `List::Nil` 结尾

这个实现：
- 解决了递归类型的大小问题
- 通过 `Box` 在堆上分配内存
- 满足了测试要求（空列表和非空列表的创建）
- 展示了 cons 列表的基本结构

现在代码应该可以通过所有测试了。这个例子很好地展示了 `Box` 智能指针在处理递归数据结构时的应用。

*/

#[derive(PartialEq, Debug)]
pub enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn main() {
    println!("This is an empty cons list: {:?}", create_empty_list());
    println!(
        "This is a non-empty cons list: {:?}",
        create_non_empty_list()
    );
}

pub fn create_empty_list() -> List {
    List::Nil
}

pub fn create_non_empty_list() -> List {
    List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_empty_list() {
        assert_eq!(List::Nil, create_empty_list())
    }

    #[test]
    fn test_create_non_empty_list() {
        assert_ne!(create_empty_list(), create_non_empty_list())
    }
}
