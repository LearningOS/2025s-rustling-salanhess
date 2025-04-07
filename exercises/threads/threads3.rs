// threads3.rs
//
// Execute `rustlings hint threads3` or use the `hint` watch subcommand for a
/*
hint
An alternate way to handle concurrency between threads is to use
a mpsc (multiple producer, single consumer) channel to communicate.
With both a sending end and a receiving end, it's possible to
send values in one thread and receive them in another.
Multiple producers are possible by using clone() to create a duplicate
of the original sending end.
See https://doc.rust-lang.org/book/ch16-02-message-passing.html for more info.

我来帮你完成 `threads3.rs` 的练习。这个练习展示了如何使用 mpsc（多生产者，单消费者）通道来进行线程间通信。我们需要修改代码以确保正确发送和接收数据。

让我来修改代码：
我已经完成了代码的修改。让我解释一下主要的改动：

1. 克隆发送端（tx）：
   ```rust
   let tx1 = tx.clone();
   let tx2 = tx;
   ```
   这样两个线程都可以发送数据。

2. 修改发送数据的代码：
   ```rust
   tx1.send(*val).unwrap();  // 第一个线程使用 tx1
   tx2.send(*val).unwrap();  // 第二个线程使用 tx2
   ```

3. 在主线程中添加了接收完成的条件：
   ```rust
   if total_received == queue_length {
       break;
   }
   ```

这些修改确保了：
1. 两个线程可以同时发送数据（多生产者）
2. 主线程可以正确接收所有数据（单消费者）
3. 当接收到所有数据后，程序会正确退出

当你运行这个程序时，你会看到：
1. 两个线程交替发送数据（1-5 和 6-10）
2. 主线程接收并打印这些数据
3. 最终会收到所有 10 个数字
4. 程序会正确退出

这个练习展示了 Rust 中如何使用 mpsc 通道来实现线程间的消息传递。


*/

use std::sync::mpsc;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

struct Queue {
    length: u32,
    first_half: Vec<u32>,
    second_half: Vec<u32>,
}

impl Queue {
    fn new() -> Self {
        Queue {
            length: 10,
            first_half: vec![1, 2, 3, 4, 5],
            second_half: vec![6, 7, 8, 9, 10],
        }
    }
}

fn send_tx(q: Queue, tx: mpsc::Sender<u32>) -> () {
    let qc = Arc::new(q);
    let qc1 = Arc::clone(&qc);
    let qc2 = Arc::clone(&qc);
    let tx1 = tx.clone();
    let tx2 = tx;

    thread::spawn(move || {
        for val in &qc1.first_half {
            println!("sending {:?}", val);
            tx1.send(*val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        for val in &qc2.second_half {
            println!("sending {:?}", val);
            tx2.send(*val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
}

fn main() {
    let (tx, rx) = mpsc::channel();
    let queue = Queue::new();
    let queue_length = queue.length;

    send_tx(queue, tx);

    let mut total_received: u32 = 0;
    for received in rx {
        println!("Got: {}", received);
        total_received += 1;
        if total_received == queue_length {
            break;
        }
    }

    println!("total numbers received: {}", total_received);
    assert_eq!(total_received, queue_length)
}
