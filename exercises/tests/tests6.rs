// tests6.rs
//
// In this example we take a shallow dive into the Rust standard library's
// unsafe functions. Fix all the question marks and todos to make the test
// pass.
//
// Execute `rustlings hint tests6` or use the `hint` watch subcommand for a
// hint.

/*
hint
The function to transform a box to a raw pointer is called `Box::into_raw`, while
the function to reconstruct a box from a raw pointer is called `Box::from_raw`.
Search the official API documentation for more information:
https://doc.rust-lang.org/nightly/std/index.html
我需要完成 `tests6.rs` 文件中的 `raw_pointer_to_box` 函数，同时保留原始注释。让我分析一下这个任务。

根据提示和测试用例，我需要：
1. 使用 `Box::from_raw` 从原始指针重建一个 `Box<Foo>`
2. 修改 `b` 字段为 `Some("hello".to_owned())`
3. 保留原始的安全注释

让我来完成这个任务：
我已经完成了 `tests6.rs` 文件中的 `raw_pointer_to_box` 函数。以下是我所做的更改：

1. 使用 `Box::from_raw(ptr)` 从原始指针重建一个 `Box<Foo>`，这是 Rust 标准库提供的函数，用于从原始指针创建 `Box`。
2. 修改了 `ret.b` 字段为 `Some("hello".to_owned())`，这符合测试用例的期望。
3. 返回修改后的 `ret`。
4. 保留了原始的安全注释，该注释说明了为什么这段代码是安全的：指针包含一个 `Foo` 的拥有权 `Box`，我们只是从该指针重建了 `Box`。

这个实现符合测试用例的要求：
- 测试用例期望 `ptr_1` 和 `ptr_2` 相等，这意味着我们正确地重建了 `Box`，保持了相同的内存地址。
- 测试用例期望 `ret.b` 为 `Some("hello".to_owned())`，我们在函数中设置了这一点。

这个实现是安全的，因为：
1. 函数文档已经声明了指针必须包含一个 `Foo` 的拥有权 `Box`
2. 我们使用了 `Box::from_raw` 来安全地重建 `Box`
3. 我们没有进行任何可能导致内存不安全操作

现在这个练习应该可以通过测试了。

*/

struct Foo {
    a: u128,
    b: Option<String>,
}

/// # Safety
///
/// The `ptr` must contain an owned box of `Foo`.
unsafe fn raw_pointer_to_box(ptr: *mut Foo) -> Box<Foo> {
    // SAFETY: The `ptr` contains an owned box of `Foo` by contract. We
    // simply reconstruct the box from that pointer.
    let mut ret: Box<Foo> = unsafe { Box::from_raw(ptr) };
    ret.b = Some("hello".to_owned());
    ret
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    #[test]
    fn test_success() {
        let data = Box::new(Foo { a: 1, b: None });

        let ptr_1 = &data.a as *const u128 as usize;
        // SAFETY: We pass an owned box of `Foo`.
        let ret = unsafe { raw_pointer_to_box(Box::into_raw(data)) };

        let ptr_2 = &ret.a as *const u128 as usize;

        assert!(ptr_1 == ptr_2);
        assert!(ret.b == Some("hello".to_owned()));
    }
}
