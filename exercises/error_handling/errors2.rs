// errors2.rs
//
// Say we're writing a game where you can buy items with tokens. All items cost
// 5 tokens, and whenever you purchase items there is a processing fee of 1
// token. A player of the game will type in how many items they want to buy, and
// the `total_cost` function will calculate the total cost of the tokens. Since
// the player typed in the quantity, though, we get it as a string-- and they
// might have typed anything, not just numbers!
//
// Right now, this function isn't handling the error case at all (and isn't
// handling the success case properly either). What we want to do is: if we call
// the `parse` function on a string that is not a number, that function will
// return a `ParseIntError`, and in that case, we want to immediately return
// that error from our function and not try to multiply and add.
//
// There are at least two ways to implement this that are both correct-- but one
// is a lot shorter!
//
// Execute `rustlings hint errors2` or use the `hint` watch subcommand for a
// hint.

/*
使用 Rust 的 ? 操作符可以简化错误传播。
在解析字符串时，如果发生错误，会立即返回该错误。否则，使用解析后的值来计算总成本，然后将其作为 Ok 结果返回。
解释
解析输入：parse::<i32>()方法尝试将输入的字符串转换为整数。?操作符用于将解析错误传播到调用栈，如果解析失败，则提前返回Err。
计算总成本：如果解析成功，使用解析后的整数乘以每个物品的成本（5个代币）并加上手续费（1个代币）来计算总成本。结果被包装在Ok中表示成功。
这种方法有效地处理了有效和无效的输入，确保错误被正确传播而不崩溃，同时有效输入被处理以计算总成本。
*/

use std::num::ParseIntError;

pub fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;
    let qty = item_quantity.parse::<i32>()?;
    Ok(qty * cost_per_item + processing_fee)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn item_quantity_is_a_valid_number() {
        assert_eq!(total_cost("34"), Ok(171));
    }

    #[test]
    fn item_quantity_is_an_invalid_number() {
        assert_eq!(
            total_cost("beep boop").unwrap_err().to_string(),
            "invalid digit found in string"
        );
    }
}
