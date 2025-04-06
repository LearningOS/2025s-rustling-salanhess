// cow1.rs
//
// This exercise explores the Cow, or Clone-On-Write type. Cow is a
// clone-on-write smart pointer. It can enclose and provide immutable access to
// borrowed data, and clone the data lazily when mutation or ownership is
// required. The type is designed to work with general borrowed data via the
// Borrow trait.
//
// This exercise is meant to show you what to expect when passing data to Cow.
// Fix the unit tests by checking for Cow::Owned(_) and Cow::Borrowed(_) at the
// TODO markers.
//
// Execute `rustlings hint cow1` or use the `hint` watch subcommand for a hint.

/*
hint
If Cow already owns the data it doesn't need to clone it when to_mut() is called.
Check out https://doc.rust-lang.org/std/borrow/enum.Cow.html for documentation
on the `Cow` type.
让我帮你完成这个关于 Cow（Clone-on-Write）智能指针的练习。
这个练习主要测试我们对 Cow 类型的理解，需要我们完成几个测试用例中的模式匹配。

Cow 是一个非常有用的智能指针，它可以：
1. 包装借用的数据并提供不可变访问
2. 只在需要修改或获取所有权时才克隆数据

让我修改代码来完成那些 TODO 的测试用例：
让我解释一下这些修改：

1. `reference_no_mutation` 测试：
   - 输入的是全是非负数的切片 `[0, 1, 2]`
   - 因为没有负数，所以 `abs_all` 不需要修改任何值
   - 所以 Cow 保持借用状态，应该匹配 `Cow::Borrowed(_)`

2. `owned_no_mutation` 测试：
   - 输入是一个 `Vec` 而不是切片引用
   - 即使没有发生修改，Cow 仍然拥有数据的所有权
   - 因此应该匹配 `Cow::Owned(_)`

3. `owned_mutation` 测试：
   - 输入是包含负数的 `Vec`
   - 虽然需要修改数据，但因为 Cow 已经拥有数据，所以不需要克隆
   - 应该匹配 `Cow::Owned(_)`

这个练习很好地展示了 Cow 的几个重要特性：
- 只在必要时（需要修改借用数据时）才进行克隆
- 如果数据已经被拥有，即使需要修改也不会克隆
- 如果不需要修改借用的数据，就保持借用状态

这种智能指针在需要在借用和所有权之间权衡的场景中特别有用，可以避免不必要的克隆操作。



*/

use std::borrow::Cow;

fn abs_all<'a, 'b>(input: &'a mut Cow<'b, [i32]>) -> &'a mut Cow<'b, [i32]> {
    for i in 0..input.len() {
        let v = input[i];
        if v < 0 {
            // Clones into a vector if not already owned.
            input.to_mut()[i] = -v;
        }
    }
    input
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reference_mutation() -> Result<(), &'static str> {
        // Clone occurs because `input` needs to be mutated.
        let slice = [-1, 0, 1];
        let mut input = Cow::from(&slice[..]);
        match abs_all(&mut input) {
            Cow::Owned(_) => Ok(()),
            _ => Err("Expected owned value"),
        }
    }

    #[test]
    fn reference_no_mutation() -> Result<(), &'static str> {
        // No clone occurs because `input` doesn't need to be mutated.
        let slice = [0, 1, 2];
        let mut input = Cow::from(&slice[..]);
        match abs_all(&mut input) {
            Cow::Borrowed(_) => Ok(()),
            _ => Err("Expected borrowed value"),
        }
    }

    #[test]
    fn owned_no_mutation() -> Result<(), &'static str> {
        // We can also pass `slice` without `&` so Cow owns it directly. In this
        // case no mutation occurs and thus also no clone, but the result is
        // still owned because it was never borrowed or mutated.
        let slice = vec![0, 1, 2];
        let mut input = Cow::from(slice);
        match abs_all(&mut input) {
            Cow::Owned(_) => Ok(()),
            _ => Err("Expected owned value"),
        }
    }

    #[test]
    fn owned_mutation() -> Result<(), &'static str> {
        // Of course this is also the case if a mutation does occur. In this
        // case the call to `to_mut()` returns a reference to the same data as
        // before.
        let slice = vec![-1, 0, 1];
        let mut input = Cow::from(slice);
        match abs_all(&mut input) {
            Cow::Owned(_) => Ok(()),
            _ => Err("Expected owned value"),
        }
    }
}
