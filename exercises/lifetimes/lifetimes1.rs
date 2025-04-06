// lifetimes1.rs
//
// The Rust compiler needs to know how to check whether supplied references are
// valid, so that it can let the programmer know if a reference is at risk of
// going out of scope before it is used. Remember, references are borrows and do
// not own their own data. What if their owner goes out of scope?
//
// Execute `rustlings hint lifetimes1` or use the `hint` watch subcommand for a
// hint.

/*
让我帮你完成这个生命周期练习。这个练习主要是关于引用的生命周期标注。

问题在于 `longest` 函数返回一个字符串切片引用，但编译器无法确定返回的引用应该具有什么生命周期。它可能是 `x` 的引用，也可能是 `y` 的引用，这取决于运行时的条件。

让我来修改代码：

```rust:exercises/lifetimes/lifetimes1.rs
// ... existing code ...
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
// ... existing code ...
```
我添加了生命周期参数 `'a`，这里的修改说明：

1. `<'a>` 声明了一个生命周期参数
2. 两个输入参数 `x` 和 `y` 都被标注为具有相同的生命周期 `'a`
3. 返回值也被标注为具有相同的生命周期 `'a`

这个标注告诉编译器：
- 返回的引用的生命周期与输入参数的生命周期相关联
- 返回的引用的生命周期不会超过输入参数的生命周期
- 两个输入参数必须活得一样久（因为它们共享同一个生命周期参数）

这样修改后，编译器就能够正确地检查引用的有效性，确保不会出现悬垂引用（dangling reference）的问题。

/*
提示：
生命周期标注是 Rust 的一个重要特性，它帮助编译器确保所有的引用都是有效的。在这个例子中：
- 如果没有生命周期标注，编译器无法确定返回的引用是否安全
- 使用相同的生命周期 'a 表示所有参数和返回值都在同一个生命周期范围内
- 这确保了返回的引用在使用时，原始数据一定还是有效的
*/

*/

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is '{}'", result);
}
