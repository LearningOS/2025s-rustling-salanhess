// arc1.rs
//
// In this exercise, we are given a Vec of u32 called "numbers" with values
// ranging from 0 to 99 -- [ 0, 1, 2, ..., 98, 99 ] We would like to use this
// set of numbers within 8 different threads simultaneously. Each thread is
// going to get the sum of every eighth value, with an offset.
//
// The first thread (offset 0), will sum 0, 8, 16, ...
// The second thread (offset 1), will sum 1, 9, 17, ...
// The third thread (offset 2), will sum 2, 10, 18, ...
// ...
// The eighth thread (offset 7), will sum 7, 15, 23, ...
//
// Because we are using threads, our values need to be thread-safe.  Therefore,
// we are using Arc.  We need to make a change in each of the two TODOs.
//
// Make this code compile by filling in a value for `shared_numbers` where the
// first TODO comment is, and create an initial binding for `child_numbers`
// where the second TODO comment is. Try not to create any copies of the
// `numbers` Vec!
//
// Execute `rustlings hint arc1` or use the `hint` watch subcommand for a hint.

/*
hint
Step 1
The compiler's message should help: since we cannot store the value of the actual type
when working with recursive types, we need to store a reference (pointer) to its value.
We should, therefore, place our `List` inside a `Box`. More details in the book here:
https://doc.rust-lang.org/book/ch15-01-box.html#enabling-recursive-types-with-boxes

Step 2
Creating an empty list should be fairly straightforward (hint: peek at the assertions).
For a non-empty list keep in mind that we want to use our Cons "list builder".
Although the current list is one of integers (i32), feel free to change the definition
and try other types!
让我帮你完成这个关于 Arc（原子引用计数）智能指针的练习。

这个练习的目标是在 8 个不同的线程中共享一个数字向量，每个线程负责计算特定偏移量的数字之和。由于涉及多线程，我们需要使用 Arc 来确保线程安全。

我会修改代码中的两个 TODO 部分：

1. 首先创建一个 Arc 来包装 numbers 向量
2. 然后在每个线程中克隆这个 Arc

让我来编辑这个文件：
让我解释一下这些修改：

1. `let shared_numbers = Arc::new(numbers);`
   - 我们使用 `Arc::new()` 来创建一个新的 Arc 智能指针，它包装了我们的 numbers 向量
   - 这样就可以在多个线程间安全地共享这个数据

2. `let child_numbers = Arc::clone(&shared_numbers);`
   - 在每个线程中，我们使用 `Arc::clone()` 来克隆 Arc 的引用
   - 这会增加引用计数，而不是复制底层数据
   - 当线程结束时，引用计数会自动减少

这个实现：
- 确保了线程安全性（通过 Arc）
- 避免了数据的复制（只复制了引用）
- 允许多个线程同时访问同一份数据
- 当最后一个引用被丢弃时，数据会被自动清理

现在代码应该可以正确编译和运行了，每个线程会计算它负责的那些数字（基于偏移量）的总和。

*/

#![forbid(unused_imports)] // Do not change this, (or the next) line.
use std::sync::Arc;
use std::thread;

fn main() {
    let numbers: Vec<_> = (0..100u32).collect();
    let shared_numbers = Arc::new(numbers);
    let mut joinhandles = Vec::new();

    for offset in 0..8 {
        let child_numbers = Arc::clone(&shared_numbers);
        joinhandles.push(thread::spawn(move || {
            let sum: u32 = child_numbers.iter().filter(|&&n| n % 8 == offset).sum();
            println!("Sum of offset {} is {}", offset, sum);
        }));
    }
    for handle in joinhandles.into_iter() {
        handle.join().unwrap();
    }
}
