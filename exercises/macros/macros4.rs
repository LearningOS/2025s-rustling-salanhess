// macros4.rs
//
// Execute `rustlings hint macros4` or use the `hint` watch subcommand for a
// hint.

/*
hint
You only need to add a single character to make this compile.
The way macros are written, it wants to see something between each
"macro arm", so it can separate them.

That's all the macro exercises we have in here, but it's barely even
scratching the surface of what you can do with Rust's macros. For a more
thorough introduction, you can have a read through the little book of Rust
macros: https://veykril.github.io/tlborm/

我来帮你完成这个宏的练习。这个练习的关键是在宏的多个分支之间添加分隔符。让我先查看完整的文件内容：




Read file: exercises/macros/macros4.rs
现在我将修改文件，在宏的分支之间添加分号，并添加注释来解释代码：
这个修改解决了问题。让我解释一下：

1. 在 Rust 的宏定义中，每个分支（arm）之间需要用分号 `;` 分隔
2. 我添加了注释来解释每个宏分支的用途：
   - 第一个分支是无参数版本
   - 第二个分支接受一个表达式参数
3. 在 `main` 函数中，我添加了注释来说明两种不同的宏调用方式

这个练习展示了 Rust 宏的一个重要特性：宏可以有多个分支，每个分支可以匹配不同的模式。在这个例子中：
- `my_macro!()` 会匹配第一个分支，输出 "Check out my macro!"
- `my_macro!(7777)` 会匹配第二个分支，输出 "Look at this other macro: 7777"

现在代码应该可以正常编译和运行了。这个练习虽然简单，但它展示了 Rust 宏系统的基本模式匹配功能。


*/

#[rustfmt::skip]
macro_rules! my_macro {
    // 无参数版本
    () => {
        println!("Check out my macro!");
    };
    // 带表达式参数的版本
    ($val:expr) => {
        println!("Look at this other macro: {}", $val);
    };
}

fn main() {
    // 调用无参数版本的宏
    my_macro!();
    // 调用带参数版本的宏
    my_macro!(7777);
}
