// options1.rs
//
// Execute `rustlings hint options1` or use the `hint` watch subcommand for a
// hint.

// This function returns how much icecream there is left in the fridge.
// If it's before 10PM, there's 5 pieces left. At 10PM, someone eats them
// all, so there'll be no more left :(

/*
让我帮你完成这个关于 Option 类型的练习。这个练习是关于冰淇淋库存的判断，需要根据时间返回剩余的冰淇淋数量。

让我们分析一下要求：
1. 如果时间在 22 点（10PM）之前，有 5 个冰淇淋
2. 22 点及以后，冰淇淋数量为 0
3. 如果时间超过 23（非法时间），返回 None
4. 需要修复 raw_value 测试

让我来修改代码：
让我解释一下我做的修改：

1. `maybe_icecream` 函数实现：
   - 使用 if-else 结构来处理不同的时间情况
   - 当时间 > 23 时返回 `None`（表示无效时间）
   - 当时间 >= 22 时返回 `Some(0)`（表示没有冰淇淋了）
   - 其他时间返回 `Some(5)`（表示还有 5 个冰淇淋）

2. 修复 `raw_value` 测试：
   - 原来的测试直接比较 `Option<u16>` 和 `u16`，这是不正确的
   - 使用 `unwrap()` 方法来获取 `Option` 中的值
   - `unwrap()` 在这里是安全的，因为我们知道在时间为 12 时一定会返回 `Some(5)`

这个实现满足了所有测试用例：
- `maybe_icecream(9)` 返回 `Some(5)`
- `maybe_icecream(22)` 返回 `Some(0)`
- `maybe_icecream(25)` 返回 `None`
- `raw_value` 测试现在可以正确获取 Option 中的值

*/
fn maybe_icecream(time_of_day: u16) -> Option<u16> {
    // We use the 24-hour system here, so 10PM is a value of 22 and 12AM is a
    // value of 0 The Option output should gracefully handle cases where
    // time_of_day > 23.
    if time_of_day > 23 {
        None
    } else if time_of_day >= 22 {
        Some(0)
    } else {
        Some(5)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_icecream() {
        assert_eq!(maybe_icecream(9), Some(5));
        assert_eq!(maybe_icecream(10), Some(5));
        assert_eq!(maybe_icecream(23), Some(0));
        assert_eq!(maybe_icecream(22), Some(0));
        assert_eq!(maybe_icecream(25), None);
    }

    #[test]
    fn raw_value() {
        // TODO: Fix this test. How do you get at the value contained in the
        // Option?
        let icecreams = maybe_icecream(12);
        assert_eq!(icecreams.unwrap(), 5);
    }
}
