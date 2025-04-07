// clippy2.rs
// 
// Execute `rustlings hint clippy2` or use the `hint` watch subcommand for a
// hint.

/*
hint
`for` loops over Option values are more clearly expressed as an `if let`
我来帮你完成这个 clippy 练习。这个练习的关键是要将 `for` 循环改为更清晰的 `if let` 语法。让我先查看完整的文件内容：





Read file: exercises/clippy/clippy2.rs
现在我将修改文件，将 `for` 循环改为 `if let` 语法，并添加注释来解释代码：
这个修改解决了问题。让我解释一下：

1. 原来的代码使用 `for` 循环来遍历 `Option`，这在 Rust 中虽然可以工作，但不是最清晰的表达方式
2. 我们将其改为使用 `if let` 语法，这样可以：
   - 更清晰地表达我们只关心 `Some` 的情况
   - 更符合 Rust 的惯用写法
   - 让代码的意图更加明确
3. 我添加了注释来解释：
   - 变量的初始化
   - Option 的创建
   - if let 的使用目的
   - 最终结果的输出

这个练习展示了 Rust 的一个重要特性：在处理 `Option` 类型时，使用 `if let` 通常比使用 `for` 循环更清晰和惯用。这是 Rust 的 clippy 工具建议的改进之一，它帮助我们写出更符合 Rust 惯用写法的代码。

现在代码应该可以通过 clippy 的检查了。这个练习虽然简单，但它展示了 Rust 中处理 `Option` 类型的最佳实践。

*/

fn main() {
    // 初始化结果变量
    let mut res = 42;
    // 创建一个包含值 12 的 Option
    let option = Some(12);
    
    // 使用 if let 替代 for 循环来处理 Option
    // 这样更清晰地表达了我们只关心 Some 的情况
    if let Some(x) = option {
        res += x;
    }
    
    // 打印最终结果
    println!("{}", res);
}


// fn main() {
//     let mut res = 42;
//     let option = Some(12);
//     for x in option {
//         res += x;
//     }
//     println!("{}", res);
// }
