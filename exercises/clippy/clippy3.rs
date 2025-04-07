// clippy3.rs
// 
// Here's a couple more easy Clippy fixes, so you can see its utility.
//
// Execute `rustlings hint clippy3` or use the `hint` watch subcommand for a hint.

/*
这些修改展示了 Rust 中一些重要的编程概念：
安全地处理 Option 类型
正确的数组语法
理解方法的返回值类型
使用标准库提供的工具函数
现在代码应该可以通过 clippy 的检查了。这个练习帮助我们理解了 Rust 中的一些常见陷阱和最佳实践。
*/

#[allow(unused_variables, unused_assignments)]
fn main() {
    // 使用 Option<()> 创建一个 Option 类型
    let my_option: Option<()> = Some(());
    // 使用 if let 语句检查 Option 是否包含值
    if let Some(item) = my_option {
        println!("Option contains a value");
    }

    // 使用 &[i32] 创建一个数组
    let my_arr = &[
        -1, -2, -3,
        -4, -5, -6
    ];
    println!("My array! Here it is: {:?}", my_arr);

    //resize() 方法返回 ()，所以不能直接赋值
    // 将操作分为两步：先创建 Vec，然后调用 resize
    let mut my_empty_vec = vec![1, 2, 3, 4, 5];
    my_empty_vec.clear();
    println!("This Vec is empty, see? {:?}", my_empty_vec);

    let mut value_a = 45;
    let mut value_b = 66;
    // 使用 std::mem::swap 交换两个变量的值
    std::mem::swap(&mut value_a, &mut value_b);
    println!("value a: {}; value b: {}", value_a, value_b);
}
// fn main() {
//     let my_option: Option<()> = None;
//     if my_option.is_none() {
//         my_option.unwrap();
//     }

//     let my_arr = &[
//         -1, -2, -3
//         -4, -5, -6
//     ];
//     println!("My array! Here it is: {:?}", my_arr);

//     let my_empty_vec = vec![1, 2, 3, 4, 5].resize(0, 5);
//     println!("This Vec is empty, see? {:?}", my_empty_vec);

//     let mut value_a = 45;
//     let mut value_b = 66;
//     // Let's swap these two!
//     value_a = value_b;
//     value_b = value_a;
//     println!("value a: {}; value b: {}", value_a, value_b);
// }
