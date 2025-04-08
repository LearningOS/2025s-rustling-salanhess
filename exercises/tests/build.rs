//! This is the build script for both tests7 and tests8.
//!
//! You should modify this file to make both exercises pass.
//[Prompt]@build.rs  should work for  @tests7.rs and @tests8.rs , pls keep original comment and finish
/*
我来帮你完成 `build.rs` 文件，使其能够同时满足 `tests7.rs` 和 `tests8.rs` 的要求。

首先，让我分析一下两个测试文件的需求：

1. `tests7.rs` 需要一个名为 `TEST_FOO` 的环境变量，其值应该是一个时间戳，并且测试会验证这个时间戳是否在合理范围内。
2. `tests8.rs` 需要一个名为 `pass` 的特性（feature）被启用，这样测试就会提前返回而不是抛出 panic。

让我修改 `build.rs` 文件：
我已经完成了 `build.rs` 文件的修改，现在它应该能够同时满足 `tests7.rs` 和 `tests8.rs` 的要求。以下是我所做的修改：

1. 对于 `tests7.rs`：
   - 我使用了 `rustc-env=TEST_FOO={}` 命令，将当前的时间戳设置为 `TEST_FOO` 环境变量的值。
   - 这样测试就能获取到这个环境变量，并验证它是否在合理范围内。

2. 对于 `tests8.rs`：
   - 我使用了 `rustc-cfg=feature="pass"` 命令，启用了 `pass` 特性。
   - 这样测试就会提前返回，而不是抛出 panic。

这些修改保留了原始文件中的所有注释，只替换了需要完成的命令部分。现在这两个测试应该都能通过了。

*/

fn main() {
    // In tests7, we should set up an environment variable
    // called `TEST_FOO`. Print in the standard output to let
    // Cargo do it.
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs(); // What's the use of this timestamp here?
    let your_command = format!(
        "rustc-env=TEST_FOO={}",
        timestamp
    );
    println!("cargo:{}", your_command);

    // In tests8, we should enable "pass" feature to make the
    // testcase return early. Fill in the command to tell
    // Cargo about that.
    let your_command = "rustc-cfg=feature=\"pass\"";
    println!("cargo:{}", your_command);
}
