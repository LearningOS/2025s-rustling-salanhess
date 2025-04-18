// threads2.rs
//
// Building on the last exercise, we want all of the threads to complete their
// work but this time the spawned threads need to be in charge of updating a
// shared value: JobStatus.jobs_completed
//
// Execute `rustlings hint threads2` or use the `hint` watch subcommand for a
/*
hint
`Arc` is an Atomic Reference Counted pointer that allows safe, shared access
to **immutable** data. But we want to *change* the number of `jobs_completed`
so we'll need to also use another type that will only allow one thread to
mutate the data at a time. Take a look at this section of the book:
https://doc.rust-lang.org/book/ch16-03-shared-state.html#atomic-reference-counting-with-arct
and keep reading if you'd like more hints :)


Do you now have an `Arc` `Mutex` `JobStatus` at the beginning of main? Like:
`let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));`
Similar to the code in the example in the book that happens after the text
that says "We can use Arc<T> to fix this.". If not, give that a try! If you
do and would like more hints, keep reading!!


Make sure neither of your threads are holding onto the lock of the mutex
while they are sleeping, since this will prevent the other thread from
being allowed to get the lock. Locks are automatically released when
they go out of scope.

If you've learned from the sample solutions, I encourage you to come
back to this exercise and try it again in a few days to reinforce
what you've learned :)
我来帮你完成 `threads2.rs` 的练习。这个练习要求我们使用 `Arc` 和 `Mutex` 来安全地共享和修改数据。我们需要修改 `JobStatus` 结构体，并在更新共享值时使用互斥锁。

让我来修改代码：
我已经完成了代码的修改。让我解释一下主要的改动：

1. 导入了 `Mutex`：`use std::sync::{Arc, Mutex};`
2. 使用 `Mutex` 包装 `JobStatus`：`let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));`
3. 在更新共享值时获取互斥锁：
   ```rust
   let mut status = status_shared.lock().unwrap();
   status.jobs_completed += 1;
   ```
4. 在主线程中打印完成的工作数量时也获取互斥锁：
   ```rust
   let status = status.lock().unwrap();
   println!("jobs completed {}", status.jobs_completed);
   ```

这些修改确保了：
1. 多个线程可以安全地共享和修改 `jobs_completed` 值
2. 每次只有一个线程可以修改这个值
3. 所有线程都能看到最新的值

当你运行这个程序时，你会看到：
1. 每个线程都会增加 `jobs_completed` 的值
2. 由于使用了互斥锁，值的更新是线程安全的
3. 最终 `jobs_completed` 的值应该是 10（因为有 10 个线程）

这个练习展示了 Rust 中如何使用 `Arc` 和 `Mutex` 来实现线程安全的共享状态。



*/

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed: u32,
}

fn main() {
    let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));
    let mut handles = vec![];
    for _ in 0..10 {
        let status_shared = Arc::clone(&status);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));
            // TODO: You must take an action before you update a shared value
            let mut status = status_shared.lock().unwrap();
            status.jobs_completed += 1;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
        // TODO: Print the value of the JobStatus.jobs_completed. Did you notice
        // anything interesting in the output? Do you have to 'join' on all the
        // handles?
        let status = status.lock().unwrap();
        println!("jobs completed {}", status.jobs_completed);
    }
}
