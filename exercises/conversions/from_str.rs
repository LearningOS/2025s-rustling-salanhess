// from_str.rs
//
// This is similar to from_into.rs, but this time we'll implement `FromStr` and
// return errors instead of falling back to a default value. Additionally, upon
// implementing FromStr, you can use the `parse` method on strings to generate
// an object of the implementor type. You can read more about it at
// https://doc.rust-lang.org/std/str/trait.FromStr.html
//
// Execute `rustlings hint from_str` or use the `hint` watch subcommand for a
/*
hint
The implementation of FromStr should return an Ok with a Person object,
or an Err with an error if the string is not valid.

This is almost like the `from_into` exercise, but returning errors instead
of falling back to a default value.

Look at the test cases to see which error variants to return.

Another hint: You can use the `map_err` method of `Result` with a function
or a closure to wrap the error from `parse::<usize>`.

Yet another hint: If you would like to propagate errors by using the `?`
operator in your solution, you might want to look at
https://doc.rust-lang.org/stable/rust-by-example/error/multiple_error_types/reenter_question_mark.html

我来帮你完成 `from_str.rs` 的练习。这个练习与之前的 `from_into.rs` 类似，但需要返回错误而不是默认值。让我实现 `FromStr` trait：



```rust:exercises/conversions/from_str.rs
impl FromStr for Person {
    type Err = ParsePersonError;
    fn from_str(s: &str) -> Result<Person, Self::Err> {
具体实现。。。
    }
}
// ... existing code ...
```
让我解释一下这个实现：

1. **错误处理**：
   - 与 `from_into.rs` 不同，这里我们返回 `Result<Person, ParsePersonError>` 而不是默认值
   - 使用自定义的 `ParsePersonError` 枚举来表示不同类型的错误

2. **输入验证**：
   - 检查空字符串，返回 `ParsePersonError::Empty`
   - 使用 `split(',')` 分割字符串，如果分割结果不是两部分，返回 `ParsePersonError::BadLen`

3. **名字处理**：
   - 获取并清理名字部分
   - 如果名字为空，返回 `ParsePersonError::NoName`

4. **年龄处理**：
   - 使用 `parse::<usize>()` 尝试解析年龄
   - 使用 `map_err` 将 `ParseIntError` 转换为 `ParsePersonError::ParseInt`
   - 使用 `?` 操作符来传播错误

这个实现满足了所有测试用例的要求：
- `empty_input()`: 处理空字符串
- `good_input()`: 正确处理有效输入
- `missing_age()`: 处理缺少年龄的情况
- `invalid_age()`: 处理无效的年龄值
- `missing_comma_and_age()`: 处理缺少逗号和年龄的情况
- `missing_name()`: 处理缺少名字的情况
- `trailing_comma()`: 处理额外的逗号

通过实现 `FromStr` trait，我们可以使用 `parse()` 方法将字符串转换为 `Person` 对象，
这在 Rust 中是一个常见的模式。

错误类型转换：
- 使用 map_err 可以将底层错误类型转换为我们的自定义错误类型
- ? 操作符会自动处理这个转换后的错误

提前返回：
-如果解析失败，? 会立即从函数返回错误
-这避免了深层嵌套的 match 语句

  在我们的 Person 实现中，? 操作符帮助我们：
-优雅地处理年龄解析错误
-保持代码简洁易读
-确保错误被正确传播到调用者
这就是为什么在实现 FromStr trait 时，使用 ? 操作符是一个很好的选择，它让错误处理变得更加优雅和直观。

*/

use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Person {
    name: String,
    age: usize,
}

// We will use this error type for the `FromStr` implementation.
#[derive(Debug, PartialEq)]
enum ParsePersonError {
    // Empty input string
    Empty,
    // Incorrect number of fields
    BadLen,
    // Empty name field
    NoName,
    // Wrapped error from parse::<usize>()
    ParseInt(ParseIntError),
}


// Steps:
// 1. If the length of the provided string is 0, an error should be returned
// 2. Split the given string on the commas present in it
// 3. Only 2 elements should be returned from the split, otherwise return an
//    error
// 4. Extract the first element from the split operation and use it as the name
// 5. Extract the other element from the split operation and parse it into a
//    `usize` as the age with something like `"4".parse::<usize>()`
// 6. If while extracting the name and the age something goes wrong, an error
//    should be returned
// If everything goes well, then return a Result of a Person object
//
// As an aside: `Box<dyn Error>` implements `From<&'_ str>`. This means that if
// you want to return a string error message, you can do so via just using
// return `Err("my error message".into())`.

impl FromStr for Person {
    type Err = ParsePersonError;
    fn from_str(s: &str) -> Result<Person, Self::Err> {
        if s.is_empty() {
            return Err(ParsePersonError::Empty);
        }

        let parts: Vec<&str> = s.split(',').collect();
        if parts.len() != 2 {
            return Err(ParsePersonError::BadLen);
        }

        let name = parts[0].trim();
        if name.is_empty() {
            return Err(ParsePersonError::NoName);
        }

        let age = parts[1]
            .trim()
            .parse::<usize>()
            .map_err(ParsePersonError::ParseInt)?;

        Ok(Person {
            name: name.to_string(),
            age,
        })
    }
}

fn main() {
    let p = "Mark,20".parse::<Person>().unwrap();
    println!("{:?}", p);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_input() {
        assert_eq!("".parse::<Person>(), Err(ParsePersonError::Empty));
    }
    #[test]
    fn good_input() {
        let p = "John,32".parse::<Person>();
        assert!(p.is_ok());
        let p = p.unwrap();
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 32);
    }
    #[test]
    fn missing_age() {
        assert!(matches!(
            "John,".parse::<Person>(),
            Err(ParsePersonError::ParseInt(_))
        ));
    }

    #[test]
    fn invalid_age() {
        assert!(matches!(
            "John,twenty".parse::<Person>(),
            Err(ParsePersonError::ParseInt(_))
        ));
    }

    #[test]
    fn missing_comma_and_age() {
        assert_eq!("John".parse::<Person>(), Err(ParsePersonError::BadLen));
    }

    #[test]
    fn missing_name() {
        assert_eq!(",1".parse::<Person>(), Err(ParsePersonError::NoName));
    }

    #[test]
    fn missing_name_and_age() {
        assert!(matches!(
            ",".parse::<Person>(),
            Err(ParsePersonError::NoName | ParsePersonError::ParseInt(_))
        ));
    }

    #[test]
    fn missing_name_and_invalid_age() {
        assert!(matches!(
            ",one".parse::<Person>(),
            Err(ParsePersonError::NoName | ParsePersonError::ParseInt(_))
        ));
    }

    #[test]
    fn trailing_comma() {
        assert_eq!("John,32,".parse::<Person>(), Err(ParsePersonError::BadLen));
    }

    #[test]
    fn trailing_comma_and_some_string() {
        assert_eq!(
            "John,32,man".parse::<Person>(),
            Err(ParsePersonError::BadLen)
        );
    }
}
