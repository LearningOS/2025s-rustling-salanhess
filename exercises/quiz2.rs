// quiz2.rs
//
// This is a quiz for the following sections:
// - Strings
// - Vecs
// - Move semantics
// - Modules
// - Enums
//
// Let's build a little machine in the form of a function. As input, we're going
// to give a list of strings and commands. These commands determine what action
// is going to be applied to the string. It can either be:
// - Uppercase the string
// - Trim the string
// - Append "bar" to the string a specified amount of times
// The exact form of this will be:
// - The input is going to be a Vector of a 2-length tuple,
//   the first element is the string, the second one is the command.
// - The output element is going to be a Vector of strings.
//
// No hints this time!
/*
我看到现在的代码有一些需要完成的部分。让我帮你完成这个练习。根据测试用例和题目要求，我们需要：

1. 完成函数签名
2. 定义正确的输出类型
3. 实现函数体
4. 添加正确的导入语句

让我来修改代码：
让我解释一下我做的修改：

1. 函数签名：
   - 输入参数 `input: Vec<(String, Command)>`：一个包含字符串和命令元组的向量
   - 返回类型 `-> Vec<String>`：一个字符串向量作为处理结果

2. 输出声明：
   - `let mut output: Vec<String> = vec![];`：创建一个可变的空字符串向量

3. 函数体实现：
   - 使用 `match` 语句处理不同的命令类型
   - `Uppercase`：将字符串转换为大写
   - `Trim`：去除字符串两端的空白
   - `Append(n)`：将 "bar" 追加 n 次到字符串末尾

4. 导入语句：
   - `use crate::my_module::transformer;`：从当前 crate 的 my_module 模块中导入 transformer 函数

这个实现满足了所有测试用例的要求：
- 可以将字符串转换为大写
- 可以去除字符串两端的空白
- 可以将 "bar" 追加指定次数到字符串末尾

现在你可以删除文件开头的 `// I AM NOT DONE` 注释，代码应该可以通过所有测试了。

*/

pub enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

mod my_module {
    use super::Command;

    // TODO: Complete the function signature!
    pub fn transformer(input: Vec<(String, Command)>) -> Vec<String> {
        // TODO: Complete the output declaration!
        let mut output: Vec<String> = vec![];
        for (string, command) in input.iter() {
            // TODO: Complete the function body. You can do it!
            match command {
                Command::Uppercase => output.push(string.to_uppercase()),
                Command::Trim => output.push(string.trim().to_string()),
                Command::Append(n) => {
                    let mut result = string.clone();
                    for _ in 0..*n {
                        result.push_str("bar");
                    }
                    output.push(result);
                }
            }
        }
        output
    }
}

#[cfg(test)]
mod tests {
    // TODO: What do we need to import to have `transformer` in scope?
    use crate::my_module::transformer;
    use super::Command;

    #[test]
    fn it_works() {
        let output = transformer(vec![
            ("hello".into(), Command::Uppercase),
            (" all roads lead to rome! ".into(), Command::Trim),
            ("foo".into(), Command::Append(1)),
            ("bar".into(), Command::Append(5)),
        ]);
        assert_eq!(output[0], "HELLO");
        assert_eq!(output[1], "all roads lead to rome!");
        assert_eq!(output[2], "foobar");
        assert_eq!(output[3], "barbarbarbarbarbar");
    }
}
