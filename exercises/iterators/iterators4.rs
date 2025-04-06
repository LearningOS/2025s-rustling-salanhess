// iterators4.rs
//
// Execute `rustlings hint iterators4` or use the `hint` watch subcommand for a
// hint.

/*
hint
In an imperative language, you might write a for loop that updates
a mutable variable. Or, you might write code utilizing recursion
and a match clause. In Rust you can take another functional
approach, computing the factorial elegantly with ranges and iterators.

Hint 2: Check out the `fold` and `rfold` methods!
分解说明：

1..=num：生成从1到num的闭区间（包含num）。
.fold(1, |acc, n| acc * n)：从初始值1开始，依次将每个数乘到累积值上。
初始值设为1，正确处理num=0的情况（0! = 1）。
每次迭代将当前数值n乘到累积值acc。
示例：
factorial(5)执行路径：
1*1 → 1*2 → 2*3 → 6*4 → 24*5 = 120
此方法简洁高效，利用Rust迭代器特性满足所有约束条件。
*/

pub fn factorial(num: u64) -> u64 {
    // Complete this function to return the factorial of num
    // Do not use:
    // - return
    // Try not to use:
    // - imperative style loops (for, while)
    // - additional variables
    // For an extra challenge, don't use:
    // - recursion
    // Execute `rustlings hint iterators4` for hints.
    (1..=num).fold(1, |acc, n| acc * n)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_0() {
        assert_eq!(1, factorial(0));
    }

    #[test]
    fn factorial_of_1() {
        assert_eq!(1, factorial(1));
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(2, factorial(2));
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(24, factorial(4));
    }
}
