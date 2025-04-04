// modules2.rs
//
// You can bring module paths into scopes and provide new names for them with
// the 'use' and 'as' keywords. Fix these 'use' statements to make the code
// compile.
//
// Execute `rustlings hint modules2` or use the `hint` watch subcommand for a
// hint.
/*
让我先查看一下代码的具体问题。

Ran tool
根据代码和错误提示，我发现这是一个关于Rust模块系统的练习。问题出在`delicious_snacks`模块中的可见性。让我们来修复这个问题。

主要问题是：
1. `fruits`和`veggies`子模块是私有的
2. 我们需要让`fruit`和`veggie`在`delicious_snacks`模块外可见

我已经修复了代码。主要的改动是：

1. 将 `use` 声明改为 `pub use`，这样重导出的名称 `fruit` 和 `veggie` 在 `delicious_snacks` 模块外也可见。

这样修改后，代码应该可以正常工作了。解释一下为什么这样修改：

1. 在Rust中，所有项（函数、类型、模块等）默认都是私有的
2. 通过添加 `pub` 关键字，我们可以让这些项在外部可见
3. 使用 `pub use` 可以重导出一个名称，使其在当前模块的公共API中可用

现在 `main` 函数可以通过 `delicious_snacks::fruit` 和 `delicious_snacks::veggie` 访问这些常量了。

*/

mod delicious_snacks {
    pub use self::fruits::PEAR as fruit;
    pub use self::veggies::CUCUMBER as veggie;

    mod fruits {
        pub const PEAR: &'static str = "Pear";
        pub const APPLE: &'static str = "Apple";
    }

    mod veggies {
        pub const CUCUMBER: &'static str = "Cucumber"; 
        pub const CARROT: &'static str = "Carrot";
    }
}

fn main() {
    println!(
        "favorite snacks: {} and {}",
        delicious_snacks::fruit,
        delicious_snacks::veggie
    );
}
