// traits3.rs
//
// Your task is to implement the Licensed trait for both structures and have
// them return the same information without writing the same function twice.
//
// Consider what you can add to the Licensed trait.
//
// Execute `rustlings hint traits3` or use the `hint` watch subcommand for a
// hint.

/*
让我帮您完成这个特质练习。这个练习的要求是为两个结构体实现 `Licensed` 特质，\
并且让它们返回相同的信息，但不能重复编写相同的函数。
关键是要在 `Licensed` 特质中添加一个默认实现，这样两个结构体就可以共享相同的实现。

```rust
pub trait Licensed {
    fn licensing_info(&self) -> String {
        String::from("Some information")
    }
}

```
我做了以下修改：

1. 在 `Licensed` 特质中为 `licensing_info` 方法提供了默认实现
2. 默认实现返回字符串 `"Some information"`，这正是测试用例中期望的值
3. 由于两个结构体 `SomeSoftware` 和 `OtherSoftware` 都实现了 `Licensed` 特质，但没有提供自己的实现，它们会自动使用特质中的默认实现

这样做的好处是：
1. 避免了代码重复
2. 两个结构体都能返回相同的许可信息
3. 如果将来需要修改许可信息，只需要修改特质中的默认实现即可

这是 Rust 特质系统的一个重要特性 - 默认实现（default implementation）允许特质为方法提供默认行为，实现该特质的类型可以选择使用默认实现或提供自己的实现。

*/

pub trait Licensed {
    fn licensing_info(&self) -> String {
        String::from("Some information")
    }
}

struct SomeSoftware {
    version_number: i32,
}

struct OtherSoftware {
    version_number: String,
}

impl Licensed for SomeSoftware {} // Don't edit this line
impl Licensed for OtherSoftware {} // Don't edit this line

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_licensing_info_the_same() {
        let licensing_info = String::from("Some information");
        let some_software = SomeSoftware { version_number: 1 };
        let other_software = OtherSoftware {
            version_number: "v2.0.0".to_string(),
        };
        assert_eq!(some_software.licensing_info(), licensing_info);
        assert_eq!(other_software.licensing_info(), licensing_info);
    }
}
