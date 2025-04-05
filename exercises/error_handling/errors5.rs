// errors5.rs
//
// This program uses an altered version of the code from errors4.
//
// This exercise uses some concepts that we won't get to until later in the
// course, like `Box` and the `From` trait. It's not important to understand
// them in detail right now, but you can read ahead if you like. For now, think
// of the `Box<dyn ???>` type as an "I want anything that does ???" type, which,
// given Rust's usual standards for runtime safety, should strike you as
// somewhat lenient!
//
// In short, this particular use case for boxes is for when you want to own a
// value and you care only that it is a type which implements a particular
// trait. To do so, The Box is declared as of type Box<dyn Trait> where Trait is
// the trait the compiler looks for on any value used in that context. For this
// exercise, that context is the potential errors which can be returned in a
// Result.
//
// What can we use to describe both errors? In other words, is there a trait
// which both errors implement?
//
// Execute `rustlings hint errors5` or use the `hint` watch subcommand for a
// hint.

/*
让我帮你完成这个练习。这是一个关于错误处理的练习，主要涉及使用 Box<dyn Trait> 来处理不同类型的错误。
通过分析代码，我们可以看到：
代码中有两种可能的错误类型：ParseIntError（来自字符串解析）和 CreationError（来自 PositiveNonzeroInteger 创建）
我们需要找到一个这两种错误都实现的 trait 来替换 ???
从代码中可以看到 CreationError 实现了 error::Error trait
由于 ParseIntError 也实现了 std::error::Error trait，所以这就是我们要找的共同 trait。
现在代码应该可以编译了。让我解释一下这个修改：
我们将 ??? 替换为 error::Error，这是一个 trait，代表任何可以作为错误处理的类型。
Box<dyn error::Error> 表示一个指向实现了 error::Error trait 的类型的智能指针。
这样的返回类型允许我们返回任何实现了 error::Error trait 的错误，包括 ParseIntError 和 CreationError。
这个修改使得 main 函数可以处理代码中可能出现的两种错误类型，因为它们都实现了 error::Error trait。? 运算符会自动将这些错误转换为 Box<dyn error::Error>。
*/

use std::error;
use std::fmt;
use std::num::ParseIntError;

// TODO: update the return type of `main()` to make this compile.
fn main() -> Result<(), Box<dyn error::Error>> {
    let pretend_user_input = "42";
    let x: i64 = pretend_user_input.parse()?;
    println!("output={:?}", PositiveNonzeroInteger::new(x)?);
    Ok(())
}

// Don't change anything below this line.

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        match value {
            x if x < 0 => Err(CreationError::Negative),
            x if x == 0 => Err(CreationError::Zero),
            x => Ok(PositiveNonzeroInteger(x as u64)),
        }
    }
}

// This is required so that `CreationError` can implement `error::Error`.
impl fmt::Display for CreationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let description = match *self {
            CreationError::Negative => "number is negative",
            CreationError::Zero => "number is zero",
        };
        f.write_str(description)
    }
}

impl error::Error for CreationError {}
