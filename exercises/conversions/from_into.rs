// from_into.rs
//
// The From trait is used for value-to-value conversions. If From is implemented
// correctly for a type, the Into trait should work conversely. You can read
// more about it at https://doc.rust-lang.org/std/convert/trait.From.html
//
// Execute `rustlings hint from_into` or use the `hint` watch subcommand for a

/*
hint
Follow the steps provided right before the `From` implementation
我来帮你完成这个练习并解释代码。这是一个关于 Rust 中 `From` trait 实现的练习，
需要将字符串转换为 `Person` 结构体。

首先，让我完成 `From` trait 的实现：


```rust:exercises/conversions/from_into.rs
impl From<&str> for Person {
    fn from(s: &str) -> Person {
   xxxx实现
    }
}
// ... existing code ...
```
让我解释一下这个实现：

1. **输入验证**：
   - 首先检查输入字符串是否为空，如果为空则返回默认的 `Person` 对象
   - 使用 `split(',')` 将字符串按逗号分割成两部分
   - 检查分割后的结果是否正好有两部分，如果不是则返回默认值

2. **名字处理**：
   - 获取第一部分作为名字，并使用 `trim()` 去除可能的空白字符
   - 如果名字为空，返回默认的 `Person` 对象

3. **年龄处理**：
   - 尝试将第二部分解析为 `usize` 类型的年龄
   - 使用 `match` 表达式处理解析结果：
     - 如果解析成功，创建新的 `Person` 对象
     - 如果解析失败，返回默认的 `Person` 对象

这个实现满足了所有测试用例的要求：
- 处理空字符串
- 处理格式不正确的输入（如缺少逗号）
- 处理无效的年龄值
- 处理名字为空的情况
- 处理额外的逗号或字符串

通过实现 `From` trait，我们同时也自动获得了 `Into` trait 的实现，
这就是为什么在 `main` 函数中可以使用 `into()` 方法将字符串转换为 `Person` 对象。

*/

#[derive(Debug)]
struct Person {
    name: String,
    age: usize,
}

// We implement the Default trait to use it as a fallback
// when the provided string is not convertible into a Person object
impl Default for Person {
    fn default() -> Person {
        Person {
            name: String::from("John"),
            age: 30,
        }
    }
}

// Your task is to complete this implementation in order for the line `let p =
// Person::from("Mark,20")` to compile Please note that you'll need to parse the
// age component into a `usize` with something like `"4".parse::<usize>()`. The
// outcome of this needs to be handled appropriately.
//
// Steps:
// 1. If the length of the provided string is 0, then return the default of
//    Person.
// 2. Split the given string on the commas present in it.
// 3. Extract the first element from the split operation and use it as the name.
// 4. If the name is empty, then return the default of Person.
// 5. Extract the other element from the split operation and parse it into a
//    `usize` as the age.
// If while parsing the age, something goes wrong, then return the default of
// Person Otherwise, then return an instantiated Person object with the results


impl From<&str> for Person {
    fn from(s: &str) -> Person {
        if s.is_empty() {
            return Person::default();
        }

        let parts: Vec<&str> = s.split(',').collect();
        if parts.len() != 2 {
            return Person::default();
        }

        let name = parts[0].trim();
        if name.is_empty() {
            return Person::default();
        }

        match parts[1].trim().parse::<usize>() {
            Ok(age) => Person {
                name: name.to_string(),
                age,
            },
            Err(_) => Person::default(),
        }
    }
}

fn main() {
    // Use the `from` function
    let p1 = Person::from("Mark,20");
    // Since From is implemented for Person, we should be able to use Into
    let p2: Person = "Gerald,70".into();
    println!("{:?}", p1);
    println!("{:?}", p2);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_default() {
        // Test that the default person is 30 year old John
        let dp = Person::default();
        assert_eq!(dp.name, "John");
        assert_eq!(dp.age, 30);
    }
    #[test]
    fn test_bad_convert() {
        // Test that John is returned when bad string is provided
        let p = Person::from("");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }
    #[test]
    fn test_good_convert() {
        // Test that "Mark,20" works
        let p = Person::from("Mark,20");
        assert_eq!(p.name, "Mark");
        assert_eq!(p.age, 20);
    }
    #[test]
    fn test_bad_age() {
        // Test that "Mark,twenty" will return the default person due to an
        // error in parsing age
        let p = Person::from("Mark,twenty");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_comma_and_age() {
        let p: Person = Person::from("Mark");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_age() {
        let p: Person = Person::from("Mark,");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_name() {
        let p: Person = Person::from(",1");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_name_and_age() {
        let p: Person = Person::from(",");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_name_and_invalid_age() {
        let p: Person = Person::from(",one");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_trailing_comma() {
        let p: Person = Person::from("Mike,32,");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_trailing_comma_and_some_string() {
        let p: Person = Person::from("Mike,32,man");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }
}
