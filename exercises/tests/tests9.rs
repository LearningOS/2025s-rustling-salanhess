// tests9.rs
//
// Rust is highly capable of sharing FFI interfaces with C/C++ and other statically compiled
// languages, and it can even link within the code itself! It makes it through the extern
// block, just like the code below.
//
// The short string after the `extern` keyword indicates which ABI the externally imported
// function would follow. In this exercise, "Rust" is used, while other variants exists like
// "C" for standard C ABI, "stdcall" for the Windows ABI.
//
// The externally imported functions are declared in the extern blocks, with a semicolon to
// mark the end of signature instead of curly braces. Some attributes can be applied to those
// function declarations to modify the linking behavior, such as #[link_name = ".."] to
// modify the actual symbol names.
//
// If you want to export your symbol to the linking environment, the `extern` keyword can
// also be marked before a function definition with the same ABI string note. The default ABI
// for Rust functions is literally "Rust", so if you want to link against pure Rust functions,
// the whole extern term can be omitted.
//
// Rust mangles symbols by default, just like C++ does. To suppress this behavior and make
// those functions addressable by name, the attribute #[no_mangle] can be applied.
//
// In this exercise, your task is to make the testcase able to call the `my_demo_function` in
// module Foo. the `my_demo_function_alias` is an alias for `my_demo_function`, so the two
// line of code in the testcase should call the same function.
//
// You should NOT modify any existing code except for adding two lines of attributes.
// Rust 非常擅长与 C/C++ 等静态编译语言共享 FFI 接口，甚至可以在代码内部进行链接！这是通过 extern 块实现的，就像下面的代码所示。
// `extern` 关键字后面的短字符串表示外部导入的函数将遵循的 ABI（应用程序二进制接口）。在这个练习中，使用的是 "Rust"，但还有其他变体，例如 "C" 表示标准 C ABI，"stdcall" 表示 Windows ABI。
// 外部导入的函数在 extern 块中声明，签名以分号 `;` 结束，而不是花括号 `{}`。可以对这些函数声明应用一些属性来修改链接行为，例如 #[link_name = ".."] 用于修改实际的符号名称。
// 如果你想将你的符号导出到链接环境，也可以在函数定义前标记 `extern` 关键字，并使用相同的 ABI 字符串。Rust 函数的默认 ABI 实际上是 "Rust"，所以如果你想链接纯 Rust 函数，整个 extern 项可以省略。
// 默认情况下，Rust 会对符号进行名称修饰（mangling），就像 C++ 那样。要抑制这种行为并使这些函数可以通过名称寻址，可以应用 #[no_mangle] 属性。
// 在这个练习中，你的任务是使测试用例能够调用 Foo 模块中的 `my_demo_function`。`my_demo_function_alias` 是 `my_demo_function` 的别名，因此测试用例中的两行代码应该调用同一个函数。
// 你不应修改任何现有代码，只能添加两行属性。


extern "Rust" {
    fn my_demo_function(a: u32) -> u32;
    #[link_name = "my_demo_function"]
    fn my_demo_function_alias(a: u32) -> u32;
}


mod Foo {
    // No `extern` equals `extern "Rust"`.
    #[no_mangle]
    fn my_demo_function(a: u32) -> u32 {
        a
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        // The externally imported functions are UNSAFE by default
        // because of untrusted source of other languages. You may
        // wrap them in safe Rust APIs to ease the burden of callers.
        //
        // SAFETY: We know those functions are aliases of a safe
        // Rust function.
        unsafe {
            my_demo_function(123);
            my_demo_function_alias(456);
        }
    }
}
