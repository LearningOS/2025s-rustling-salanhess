// threads1.rs
//
// This program spawns multiple threads that each run for at least 250ms, and
// each thread returns how much time they took to complete. The program should
// wait until all the spawned threads have finished and should collect their
// return values into a vector.
//
// Execute `rustlings hint threads1` or use the `hint` watch subcommand for a
/*
hint
`JoinHandle` is a struct that is returned from a spawned thread:
https://doc.rust-lang.org/std/thread/fn.spawn.html

A challenge with multi-threaded applications is that the main thread can
finish before the spawned threads are completed.
https://doc.rust-lang.org/book/ch16-01-threads.html#waiting-for-all-threads-to-finish-using-join-handles

Use the JoinHandles to wait for each thread to finish and collect their results.
https://doc.rust-lang.org/std/thread/struct.JoinHandle.html

我来帮你完成这个线程练习。这个练习要求我们等待所有线程完成并收集它们的返回值。
我们需要使用 JoinHandle 的 join() 方法来等待线程完成并获取返回值。
我已经完成了代码的修改。让我解释一下这个修改：
我们使用 handle.join().unwrap() 来等待每个线程完成并获取其返回值
join() 方法会阻塞当前线程，直到被等待的线程完成
unwrap() 用于处理可能的错误（如果线程发生 panic）
将获取到的结果（线程执行时间）添加到 results 向量中
这样修改后，程序会：
创建 10 个线程，每个线程运行至少 250ms
等待所有线程完成
收集每个线程的执行时间
最后打印出每个线程的执行时间
你可以运行这个程序来测试结果。每个线程应该会打印出它完成的消息，最后会显示每个线程的具体执行时间。
*/

use std::thread;
use std::time::{Duration, Instant};

fn main() {
    let mut handles = vec![];
    for i in 0..10 {
        handles.push(thread::spawn(move || {
            let start = Instant::now();
            thread::sleep(Duration::from_millis(250));
            println!("thread {} is complete", i);
            start.elapsed().as_millis()
        }));
    }

    let mut results: Vec<u128> = vec![];
    for handle in handles {
        // TODO: a struct is returned from thread::spawn, can you use it?
        let result = handle.join().unwrap();
        results.push(result);
    }

    if results.len() != 10 {
        panic!("Oh no! All the spawned threads did not finish!");
    }

    println!();
    for (i, result) in results.into_iter().enumerate() {
        println!("thread {} took {}ms", i, result);
    }
}
