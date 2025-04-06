// lifetimes3.rs
//
// Lifetimes are also needed when structs hold references.
//
// Execute `rustlings hint lifetimes3` or use the `hint` watch subcommand for a
// hint.

/*
让我来解决这个关于结构体中引用的生命周期问题。

这个练习展示了当结构体包含引用字段时，我们需要为这些引用指定生命周期标注。问题在于 `Book` 结构体包含了两个字符串引用（`&str`），但没有指定这些引用的生命周期。

让我修改代码：

```rust:exercises/lifetimes/lifetimes3.rs
// ... existing code ...
struct Book<'a> {
    author: &'a str,
    title: &'a str,
}
// ... existing code ...
```
让我解释这个修改：

1. **为什么需要生命周期标注**：
   - 当结构体包含引用时，我们需要告诉编译器这些引用的有效期
   - 这确保结构体实例不会比它引用的数据活得更久
   - 防止出现悬垂引用（dangling references）

2. **修改的内容**：
   - 添加了生命周期参数 `'a` 到结构体定义：`struct Book<'a>`
   - 为两个引用字段都标注了相同的生命周期 `'a`
   - 这表明 `author` 和 `title` 引用必须在结构体实例的整个生命周期内保持有效

3. **代码如何工作**：
   ```rust
   let name = String::from("Jill Smith");
   let title = String::from("Fish Flying");
   let book = Book { author: &name, title: &title };
   println!("{} by {}", book.title, book.author);
   ```
   - `name` 和 `title` 字符串在 `book` 使用期间保持有效
   - 生命周期标注确保了这些引用在 `println!` 语句执行时仍然有效
   - 如果我们试图使用超出了源数据生命周期的引用，编译器会报错

这个练习展示了 Rust 如何通过生命周期系统来确保引用的安全性，即使在更复杂的数据结构中也是如此。
生命周期标注是 Rust 的一个重要特性，它帮助我们在编译时就能发现潜在的内存安全问题。

*/

struct Book<'a> {
    author: &'a str,
    title: &'a str,
}

fn main() {
    let name = String::from("Jill Smith");
    let title = String::from("Fish Flying");
    let book = Book { author: &name, title: &title };

    println!("{} by {}", book.title, book.author);
}
