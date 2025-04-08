// as_ref_mut.rs
//
// AsRef and AsMut allow for cheap reference-to-reference conversions. Read more
// about them at https://doc.rust-lang.org/std/convert/trait.AsRef.html and
// https://doc.rust-lang.org/std/convert/trait.AsMut.html, respectively.
//
// Execute `rustlings hint as_ref_mut` or use the `hint` watch subcommand for a
// hint：Add AsRef<str> or AsMut<u32> as a trait bound to the functions.

// Obtain the number of bytes (not characters) in the given argument.
// TODO: Add the AsRef trait appropriately as a trait bound.
/*
我来帮你完成 `as_ref_mut.rs` 的练习。这个练习需要实现 `AsRef` 和 `AsMut` trait 的转换：
让我解释一下这个实现：

1. **`byte_counter` 函数**：
   ```rust
   fn byte_counter<T: AsRef<str>>(arg: T) -> usize
   ```
   - 使用 `AsRef<str>` trait bound
   - 允许任何可以转换为 `&str` 的类型作为参数
   - 计算字符串的字节数

2. **`char_counter` 函数**：
   ```rust
   fn char_counter<T: AsRef<str>>(arg: T) -> usize
   ```
   - 同样使用 `AsRef<str>` trait bound
   - 计算字符串的字符数（考虑 Unicode）

3. **`num_sq` 函数**：
   ```rust
   fn num_sq<T: AsMut<u32>>(arg: &mut T)
   ```
   - 使用 `AsMut<u32>` trait bound
   - 允许任何可以转换为 `&mut u32` 的类型
   - 对数字进行平方运算

这个实现满足了所有测试用例：
- `different_counts`：测试包含 Unicode 字符的字符串
- `same_counts`：测试 ASCII 字符串
- `different_counts_using_string`：测试 String 类型
- `same_counts_using_string`：测试 String 类型
- `mult_box`：测试 Box 类型的数字平方

`AsRef` 和 `AsMut` 的优点：
1. 提供廉价的引用到引用的转换
2. 允许函数接受多种类型作为参数
3. 保持类型安全
4. 避免不必要的克隆

这样的实现使得函数更加灵活，可以处理多种输入类型，同时保持代码的简洁性和类型安全性。

@as_ref_mut.rs  keep original comment and finish
*/
fn byte_counter<T: AsRef<str>>(arg: T) -> usize {
    arg.as_ref().as_bytes().len()
}

// Obtain the number of characters (not bytes) in the given argument.
// TODO: Add the AsRef trait appropriately as a trait bound.
fn char_counter<T: AsRef<str>>(arg: T) -> usize {
    arg.as_ref().chars().count()
}

// Squares a number using as_mut().
// TODO: Add the appropriate trait bound.
fn num_sq<T: AsMut<u32>>(arg: &mut T) {
    // TODO: Implement the function body.
    let value = arg.as_mut();
    *value = (*value).pow(2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn different_counts() {
        let s = "Café au lait";
        assert_ne!(char_counter(s), byte_counter(s));
    }

    #[test]
    fn same_counts() {
        let s = "Cafe au lait";
        assert_eq!(char_counter(s), byte_counter(s));
    }

    #[test]
    fn different_counts_using_string() {
        let s = String::from("Café au lait");
        assert_ne!(char_counter(s.clone()), byte_counter(s));
    }

    #[test]
    fn same_counts_using_string() {
        let s = String::from("Cafe au lait");
        assert_eq!(char_counter(s.clone()), byte_counter(s));
    }

    #[test]
    fn mult_box() {
        let mut num: Box<u32> = Box::new(3);
        num_sq(&mut num);
        assert_eq!(*num, 9);
    }
}
