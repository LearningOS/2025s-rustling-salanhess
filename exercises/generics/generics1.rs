// generics1.rs
//
// This shopping list program isn't compiling! Use your knowledge of generics to
// fix it.
//
// Execute `rustlings hint generics1` or use the `hint` watch subcommand for a
// hint.

/*
让我帮您完成这个泛型练习。这是一个简单的购物清单程序，需要修复 `Vec` 的泛型类型。

由于我们在向 `shopping_list` 中推入字符串字面量 `"milk"`，所以我们需要使用 `&str` 作为 `Vec` 的泛型类型参数。

```rust
    let mut shopping_list: Vec<&str> = Vec::new();
    shopping_list.push("milk");
}
```
我已经将 `Vec<?>` 修改为 `Vec<&str>`，因为：
1. 我们存储的是字符串字面量 `"milk"`，它的类型是 `&str`
2. `Vec` 需要知道它要存储什么类型的元素，这样它才能正确分配内存和管理数据

现在代码应该可以正确编译了。这个例子展示了 Rust 中泛型的基本用法，通过在尖括号 `<>` 中指定具体类型来实例化泛型类型。

*/

fn main() {
    let mut shopping_list: Vec<&str> = Vec::new();
    shopping_list.push("milk");
}
