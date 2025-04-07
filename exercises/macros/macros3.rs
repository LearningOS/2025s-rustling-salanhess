// macros3.rs
//
// Make me compile, without taking the macro out of the module!
//
// Execute `rustlings hint macros3` or use the `hint` watch subcommand for a
/*
hint
In order to use a macro outside of its module, you need to do something
special to the module to lift the macro out into its parent.

The same trick also works on "extern crate" statements for crates that have
exported macros, if you've seen any of those around.

我来帮你完成这个宏的练习。这个练习的关键是要让宏在模块外部可用。在 Rust 中，我们需要使用 `#[macro_export]` 属性来导出宏。

让我先查看一下完整的文件内容：


Read file: exercises/macros/macros3.rs
现在我将修改文件，添加 `#[macro_export]` 属性来导出宏：
这个修改解决了问题。让我解释一下：

1. `#[macro_export]` 属性告诉 Rust 编译器将这个宏导出到父作用域中
2. 这样在 `main` 函数中就可以直接使用 `my_macro!()` 了
3. 宏会被提升到模块的父作用域中，所以不需要通过 `macros::my_macro!()` 的方式来调用

现在代码应该可以正常编译和运行了。当你运行这个程序时，它会输出 "Check out my macro!"。

这个练习展示了 Rust 中宏的一个重要特性：通过 `#[macro_export]` 属性，我们可以让宏在模块外部可用。这在创建库时特别有用，因为这样可以让库的用户也能使用你定义的宏。


*/
mod macros {
    #[macro_export]
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }
}

fn main() {
    my_macro!();
}
