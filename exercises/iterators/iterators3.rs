// iterators3.rs
//
// This is a bigger exercise than most of the others! You can do it! Here is
// your mission, should you choose to accept it:
// 1. Complete the divide function to get the first four tests to pass.
// 2. Get the remaining tests to pass by completing the result_with_list and
//    list_of_results functions.
//
// Execute `rustlings hint iterators3` or use the `hint` watch subcommand for a
// hint.
// The divide function needs to return the correct error when even division is not
// possible.

// The division_results variable needs to be collected into a collection type.

// The result_with_list function needs to return a single Result where the success
// case is a vector of integers and the failure case is a DivisionError.

// The list_of_results function needs to return a vector of results.

// See https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.collect for how
// the `FromIterator` trait is used in `collect()`. This trait is REALLY powerful! It
// can make the solution to this exercise infinitely easier.

/*
让我帮你完成这个关于迭代器的练习。这个练习需要我们实现三个函数：divide、result_with_list 和 list_of_results。让我们一步步来解决。
首先，让我实现 divide 函数：现在让我们实现 result_with_list 和 list_of_results 函数：

让我帮你完成这个关于迭代器的练习。
这个练习需要我们实现三个函数：`divide`、`result_with_list` 和 `list_of_results`。让我们一步步来解决。
让我解释这些实现：

1. **`divide` 函数**：
   - 首先检查除数是否为 0，如果是则返回 `DivisionError::DivideByZero`
   - 然后检查是否能整除（使用取模运算 `%`）
   - 如果能整除，返回除法结果
   - 如果不能整除，返回 `DivisionError::NotDivisible` 错误

2. **`result_with_list` 函数**：
   - 返回类型是 `Result<Vec<i32>, DivisionError>`
   - 使用 `collect()` 将迭代器的结果收集到一个 `Result`
   - 如果所有除法都成功，返回包含所有结果的 vector
   - 如果有任何错误，返回第一个遇到的错误

3. **`list_of_results` 函数**：
   - 返回类型是 `Vec<Result<i32, DivisionError>>`
   - 同样使用 `collect()`，但这次收集到一个包含所有 `Result` 的 vector
   - 每个除法操作的结果都被保留在 vector 中，无论成功还是失败

这个练习展示了 Rust 迭代器的强大功能：
- 使用 `map` 转换数据
- 使用 `collect` 收集结果
- 处理错误传播
- 不同的结果类型收集策略（单个 `Result` vs `Vec` 的 `Result`）


*/
#[derive(Debug, PartialEq, Eq)]
pub enum DivisionError {
    NotDivisible(NotDivisibleError),
    DivideByZero,
}

#[derive(Debug, PartialEq, Eq)]
pub struct NotDivisibleError {
    dividend: i32,
    divisor: i32,
}

// Calculate `a` divided by `b` if `a` is evenly divisible by `b`.
// Otherwise, return a suitable error.
pub fn divide(a: i32, b: i32) -> Result<i32, DivisionError> {
    if b == 0 {
        return Err(DivisionError::DivideByZero);
    }
    if a % b == 0 {
        Ok(a / b)
    } else {
        Err(DivisionError::NotDivisible(NotDivisibleError {
            dividend: a,
            divisor: b,
        }))
    }
}

// Complete the function and return a value of the correct type so the test
// passes.
// Desired output: Ok([1, 11, 1426, 3])
fn result_with_list() -> Result<Vec<i32>, DivisionError> {
    let numbers = vec![27, 297, 38502, 81];
    let division_results = numbers.into_iter().map(|n| divide(n, 27));
    division_results.collect()
}

// Complete the function and return a value of the correct type so the test
// passes.
// Desired output: [Ok(1), Ok(11), Ok(1426), Ok(3)]
fn list_of_results() -> Vec<Result<i32, DivisionError>> {
    let numbers = vec![27, 297, 38502, 81];
    let division_results = numbers.into_iter().map(|n| divide(n, 27));
    division_results.collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(divide(81, 9), Ok(9));
    }

    #[test]
    fn test_not_divisible() {
        assert_eq!(
            divide(81, 6),
            Err(DivisionError::NotDivisible(NotDivisibleError {
                dividend: 81,
                divisor: 6
            }))
        );
    }

    #[test]
    fn test_divide_by_0() {
        assert_eq!(divide(81, 0), Err(DivisionError::DivideByZero));
    }

    #[test]
    fn test_divide_0_by_something() {
        assert_eq!(divide(0, 81), Ok(0));
    }

    #[test]
    fn test_result_with_list() {
        assert_eq!(format!("{:?}", result_with_list()), "Ok([1, 11, 1426, 3])");
    }

    #[test]
    fn test_list_of_results() {
        assert_eq!(
            format!("{:?}", list_of_results()),
            "[Ok(1), Ok(11), Ok(1426), Ok(3)]"
        );
    }
}
