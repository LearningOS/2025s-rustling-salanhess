// errors3.rs
//
// This is a program that is trying to use a completed version of the
// `total_cost` function from the previous exercise. It's not working though!
// Why not? What should we do to fix it?
//
// Execute `rustlings hint errors3` or use the `hint` watch subcommand for a
// hint.

/*
我看到问题出在 main 函数中使用了 ? 运算符。? 运算符只能在返回 Result 或 Option 类型的函数中使用，但是 main 函数并不返回 Result 类型。
我们有两种方法可以解决这个问题：
修改 main 函数的返回类型为 Result<(), ParseIntError>
使用 match 或 unwrap 等方法直接处理 Result
我会选择第一种方案，因为这是处理错误的更优雅的方式。
让我来修改这个文件：
我已经修复了这个问题。主要的改动是：
将 main 函数的签名改为 fn main() -> Result<(), ParseIntError>，这样它就可以返回一个 Result 类型
在函数末尾添加了 Ok(()) 作为返回值
现在代码应该可以正常工作了。这个程序会：
尝试将用户输入解析为数字
计算总成本（每个物品 5 个代币加上 1 个处理费）
检查用户是否有足够的代币
如果有足够的代币，会扣除相应的数量并显示剩余代币数量
如果在任何步骤中发生解析错误（比如用户输入了非数字字符串），错误会被正确地传播出去。

*/

use std::num::ParseIntError;

fn main() -> Result<(), ParseIntError> {
    let mut tokens = 100;
    let pretend_user_input = "8";

    let cost = total_cost(pretend_user_input)?;

    if cost > tokens {
        println!("You can't afford that many!");
    } else {
        tokens -= cost;
        println!("You now have {} tokens.", tokens);
    }
    Ok(())
}

pub fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;
    let qty = item_quantity.parse::<i32>()?;

    Ok(qty * cost_per_item + processing_fee)
}
