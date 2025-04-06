// lifetimes2.rs
//
// So if the compiler is just validating the references passed to the annotated
// parameters and the return type, what do we need to change?
//
// Execute `rustlings hint lifetimes2` or use the `hint` watch subcommand for a
// hint.

/*
让我来分析并解决这个生命周期问题。

这个练习展示了一个典型的生命周期错误场景。让我们看看代码中的问题：

```rust
fn main() {
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }  // string2 在这里被释放
    println!("The longest string is '{}'", result);  // 试图使用可能已经无效的引用
}
```

问题在于：
1. `string2` 的生命周期被限制在内部作用域 `{}` 中
2. `result` 可能会引用 `string2`（如果 `string2` 更长）
3. 当我们在外部作用域使用 `result` 时，`string2` 已经被释放了

要修复这个问题，我们需要确保所有引用在使用时都是有效的。让我修改代码：

```rust:exercises/lifetimes/lifetimes2.rs
// ... existing code ...
fn main() {
    let string1 = String::from("long string is long");
    let string2 = String::from("xyz");
    let result = longest(string1.as_str(), string2.as_str());
    println!("The longest string is '{}'", result);
}
// ... existing code ...
```
让我解释这个修改：

1. **原始代码的问题**：
   - `string2` 被创建在一个内部作用域中
   - 当这个作用域结束时，`string2` 被释放
   - 如果 `result` 引用了 `string2`，那么在打印时就会使用一个已经无效的引用

2. **修改后的解决方案**：
   - 将 `string2` 移到与 `string1` 相同的作用域中
   - 这样确保 `string2` 的生命周期至少持续到 `println!` 语句执行完毕
   - `result` 的引用现在总是有效的，因为它引用的字符串（无论是 `string1` 还是 `string2`）都在整个使用期间保持有效

3. **为什么这样修改有效**：
   - 生命周期标注 `'a` 要求所有引用在使用时必须有效
   - 通过将所有字符串移到同一作用域，我们确保了它们的生命周期足够长
   - 这满足了 `longest` 函数的生命周期约束，即返回的引用不会比输入参数活得更久

这个练习很好地展示了 Rust 的借用检查器如何在编译时防止悬垂引用，这是 Rust 内存安全保证的重要组成部分。

*/

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("long string is long");
    let string2 = String::from("xyz");
    let result = longest(string1.as_str(), string2.as_str());
    println!("The longest string is '{}'", result);
}
