// tests5.rs
//
// An `unsafe` in Rust serves as a contract.
//
// When `unsafe` is marked on an item declaration, such as a function,
// a trait or so on, it declares a contract alongside it. However,
// the content of the contract cannot be expressed only by a single keyword.
// Hence, its your responsibility to manually state it in the `# Safety`
// section of your documentation comment on the item.
//
// When `unsafe` is marked on a code block enclosed by curly braces,
// it declares an observance of some contract, such as the validity of some
// pointer parameter, the ownership of some memory address. However, like
// the text above, you still need to state how the contract is observed in
// the comment on the code block.
//
// NOTE: All the comments are for the readability and the maintainability of
// your code, while the Rust compiler hands its trust of soundness of your
// code to yourself! If you cannot prove the memory safety and soundness of
// your own code, take a step back and use safe code instead!
//
// Execute `rustlings hint tests5` or use the `hint` watch subcommand for a
/*
hint
For more information about `unsafe` and soundness, see
https://doc.rust-lang.org/nomicon/safe-unsafe-meaning.html

我需要完成 `tests5.rs` 文件中的 `modify_by_address` 函数，同时保留原始注释。让我先分析一下这个任务。

这个函数需要修改一个通过地址传递的 `u32` 值。根据测试用例，我们需要将值从 `0x12345678` 修改为 `0xAABBCCDD`。同时，我们需要在 `unsafe` 代码块中添加安全注释，说明为什么这段代码是安全的。

让我来完成这个任务：

我已经完成了 `tests5.rs` 文件中的 `modify_by_address` 函数。以下是我所做的更改：

1. 添加了安全注释，说明为什么这段代码是安全的：
   ```
   // SAFETY: The address is guaranteed to be valid and contains
   // a unique reference to a `u32` value as per the function's contract.
   ```

2. 实现了函数的功能：
   - 将传入的 `address` 转换为 `*mut u32` 类型的指针
   - 通过解引用指针将值修改为 `0xAABBCCDD`

这个实现符合测试用例的要求，测试用例期望将值从 `0x12345678` 修改为 `0xAABBCCDD`。同时，我保留了原始的函数文档注释，该注释说明了函数的契约：地址必须包含对有效 `u32` 值的可变引用。

这个实现是安全的，因为：
1. 函数文档已经声明了地址必须包含对有效 `u32` 值的引用
2. 测试用例中提供了有效的地址（本地变量的地址）
3. 我们只是修改了指针指向的值，没有进行任何可能导致内存不安全操作

现在这个练习应该可以通过测试了。

*/


/// # Safety
///
/// The `address` must contain a mutable reference to a valid `u32` value.
unsafe fn modify_by_address(address: usize) {
    // SAFETY: The address is guaranteed to be valid and contains
    // a unique reference to a `u32` value as per the function's contract.
    unsafe {
        let ptr = address as *mut u32;
        *ptr = 0xAABBCCDD;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        let mut t: u32 = 0x12345678;
        // SAFETY: The address is guaranteed to be valid and contains
        // a unique reference to a `u32` local variable.
        unsafe { modify_by_address(&mut t as *mut u32 as usize) };
        assert!(t == 0xAABBCCDD);
    }
}
