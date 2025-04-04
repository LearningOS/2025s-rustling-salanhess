// errors1.rs
//
// This function refuses to generate text to be printed on a nametag if you pass
// it an empty string. It'd be nicer if it explained what the problem was,
// instead of just sometimes returning `None`. Thankfully, Rust has a similar
// construct to `Result` that can be used to express error conditions. Let's use
// it!
//
// Execute `rustlings hint errors1` or use the `hint` watch subcommand for a
// hint.

/*
让我来帮你完成这个错误处理的练习。这个练习要求我们将 `Option<String>` 改为 `Result<String, String>` 来更好地处理错误情况。
我对代码做了以下修改：

1. 将函数返回类型从 `Option<String>` 改为 `Result<String, String>`
2. 将 `None` 改为返回一个带有错误信息的 `Err`
3. 将 `Some` 改为 `Ok`

这些改动的好处是：
- 现在函数可以返回具体的错误信息，而不是简单的 `None`
- 使用 `Result` 更符合 Rust 的错误处理惯例
- 错误信息清晰地说明了问题所在：名字不能为空

测试用例已经预期这些改动，它们检查：
1. 当提供有效名字时，返回 `Ok` 包含正确的欢迎文本
2. 当提供空名字时，返回 `Err` 包含指定的错误信息

这个改动展示了 Rust 中 `Result` 类型的基本用法，它比 `Option` 提供了更丰富的错误处理能力。


*/
pub fn generate_nametag_text(name: String) -> Result<String, String> {
    if name.is_empty() {
        // Empty names aren't allowed.
        Err("`name` was empty; it must be nonempty.".into())
    } else {
        Ok(format!("Hi! My name is {}", name))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_nametag_text_for_a_nonempty_name() {
        assert_eq!(
            generate_nametag_text("Beyoncé".into()),
            Ok("Hi! My name is Beyoncé".into())
        );
    }

    #[test]
    fn explains_why_generating_nametag_text_fails() {
        assert_eq!(
            generate_nametag_text("".into()),
            // Don't change this line
            Err("`name` was empty; it must be nonempty.".into())
        );
    }
}
