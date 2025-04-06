// traits5.rs
//
// Your task is to replace the '??' sections so the code compiles.
//
// Don't change any line other than the marked one.
//
// Execute `rustlings hint traits5` or use the `hint` watch subcommand for a
// hint.

/*
这道题目的关键是要实现一个函数 some_func，它接受一个同时实现了 SomeTrait 和 OtherTrait 的参数。
从代码中我们可以看到，SomeStruct 和 OtherStruct 都实现了这两个 trait。
我们需要使用 trait bounds 来指定参数类型。
在这种情况下，我们可以使用 impl trait 语法或者泛型语法。让我修改代码：
我使用了 impl SomeTrait + OtherTrait 作为参数类型，这表示：
参数 item 必须实现 SomeTrait trait
同时也必须实现 OtherTrait trait
+ 符号用于组合多个 trait bounds
这样修改后，函数就可以接受任何同时实现了这两个 trait 的类型作为参数。在这个例子中，SomeStruct 和 OtherStruct 都满足这个要求，所以 main 函数中的两个调用都是合法的。
现在代码应该可以编译通过了。你可以运行测试来验证这个解决方案。
*/

pub trait SomeTrait {
    fn some_function(&self) -> bool {
        true
    }
}

pub trait OtherTrait {
    fn other_function(&self) -> bool {
        true
    }
}

struct SomeStruct {}
struct OtherStruct {}

impl SomeTrait for SomeStruct {}
impl OtherTrait for SomeStruct {}
impl SomeTrait for OtherStruct {}
impl OtherTrait for OtherStruct {}

// YOU MAY ONLY CHANGE THE NEXT LINE
fn some_func(item: impl SomeTrait + OtherTrait) -> bool {
    item.some_function() && item.other_function()
}

fn main() {
    some_func(SomeStruct {});
    some_func(OtherStruct {});
}
