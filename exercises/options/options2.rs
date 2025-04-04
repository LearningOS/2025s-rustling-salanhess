// options2.rs
//
// Execute `rustlings hint options2` or use the `hint` watch subcommand for a
// hint.

/*
让我帮你完成这个关于 Rust `Option` 类型的练习。这个文件中有两个需要修复的测试用例。

我会修改文件来解决语法错误并完成 TODO 中要求的任务：
1. 第一个测试需要使用 `if let` 语句来处理 `Some` 类型
2. 第二个测试需要使用 `while let` 语句来处理嵌套的 `Option` 类型

让我来编辑这个文件：
我已经修复了两处代码：

1. 第一个测试中，我添加了 `if let` 语句来正确处理 `Some` 类型：
   ```rust
   if let Some(word) = optional_target {
       assert_eq!(word, target);
   }
   ```
   这样可以安全地解构 `Some` 值并将其内容绑定到 `word` 变量。

2. 第二个测试中，我添加了 `while let` 语句来处理双层 `Option`：
   ```rust
   while let Some(Some(integer)) = optional_integers.pop() {
       assert_eq!(integer, cursor);
       cursor -= 1;
   }
   ```
   这里使用了模式匹配来处理两层 `Option`：
   - 外层 `Some` 来自 `Vec::pop()` 的返回值（当 vector 非空时）
   - 内层 `Some` 来自 vector 中存储的 `Option<i8>` 值

现在代码应该可以通过所有测试了。这个练习展示了 Rust 中 `Option` 类型的模式匹配功能，
以及如何使用 `if let` 和 `while let` 语句来优雅地处理可选值。

*/

#[cfg(test)]
mod tests {
    #[test]
    fn simple_option() {
        let target = "rustlings";
        let optional_target = Some(target);

        // TODO: Make this an if let statement whose value is "Some" type
        if let Some(word) = optional_target {
            assert_eq!(word, target);
        }
    }

    #[test]
    fn layered_option() {
        let range = 10;
        let mut optional_integers: Vec<Option<i8>> = vec![None];

        for i in 1..(range + 1) {
            optional_integers.push(Some(i));
        }

        let mut cursor = range;

        // TODO: make this a while let statement - remember that vector.pop also
        // adds another layer of Option<T>. You can stack `Option<T>`s into
        // while let and if let.
        while let Some(Some(integer)) = optional_integers.pop() {
            assert_eq!(integer, cursor);
            cursor -= 1;
        }

        assert_eq!(cursor, 0);
    }
}
