// options3.rs
//
// Execute `rustlings hint options3` or use the `hint` watch subcommand for a
// hint.

/*
让我来帮你完成 `options3.rs` 的练习。这个练习的关键问题在于最后一行的 `y;` 语句会导致所有权问题，因为前面的 `match` 语句已经消耗了 `y` 的所有权。

我对代码做了一个简单但重要的修改：在 `match` 表达式中使用了 `&y` 而不是 `y`。这样的改动解决了所有权问题，因为：

1. 通过使用引用 `&y`，`match` 语句只是借用了 `y` 的值，而不是获取其所有权
2. 这样在 `match` 语句之后，`y` 仍然可以被使用
3. 在 `match` 模式中，`p` 现在是对 `Point` 的引用，但我们仍然可以通过 `.` 操作符访问其字段

这个修改保持了原有代码的功能，同时解决了所有权问题。这是 Rust 所有权系统的一个很好的例子，展示了如何通过引用来共享数据访问而不转移所有权。

现在代码应该可以编译和运行了，并且保持了要求的不删除最后一行 `y;` 的条件。

*/

struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let y: Option<Point> = Some(Point { x: 100, y: 200 });

    match &y {
        Some(p) => println!("Co-ordinates are {},{} ", p.x, p.y),
        _ => panic!("no match!"),
    }
    y; // Fix without deleting this line.
}
