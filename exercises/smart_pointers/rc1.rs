// rc1.rs
//
// In this exercise, we want to express the concept of multiple owners via the
// Rc<T> type. This is a model of our solar system - there is a Sun type and
// multiple Planets. The Planets take ownership of the sun, indicating that they
// revolve around the sun.
//
// Make this code compile by using the proper Rc primitives to express that the
// sun has multiple owners.
//
// Execute `rustlings hint rc1` or use the `hint` watch subcommand for a hint.

/*
hint
This is a straightforward exercise to use the Rc<T> type. Each Planet has
ownership of the Sun, and uses Rc::clone() to increment the reference count of the Sun.
After using drop() to move the Planets out of scope individually, the reference count goes down.
In the end the sun only has one reference again, to itself. See more at:
https://doc.rust-lang.org/book/ch15-04-rc.html

* Unfortunately Pluto is no longer considered a planet :(
让我帮你完成这个关于 Rc（引用计数）智能指针的练习。这个练习模拟了太阳系，其中太阳有多个所有者（行星）。

我注意到代码中有几个需要修复的地方：
1. Saturn、Uranus 和 Neptune 的创建方式不正确，它们创建了新的 Sun 实例
2. 最后三个 drop 操作缺失

让我来修改代码：
让我解释一下这些修改：

1. 修复行星的创建：
   - 将 Saturn、Uranus 和 Neptune 的创建从 `Rc::new(Sun {})` 改为 `Rc::clone(&sun)`
   - 这样所有行星都共享同一个太阳实例，而不是各自创建新的太阳
   - 使用 `Rc::clone` 只会增加引用计数，不会创建新的数据

2. 添加缺失的 drop 操作：
   - 添加了 `drop(earth)`、`drop(venus)` 和 `drop(mercury)`
   - 这些操作会按顺序减少太阳的引用计数
   - 最终只剩下一个引用（原始的 sun 变量）

这个实现展示了 `Rc<T>` 的关键特性：
- 允许多个所有者共享同一个数据
- 通过引用计数跟踪数据的生命周期
- 当引用计数降为 0 时，数据被自动清理

现在代码应该可以正确运行，并且能看到引用计数随着行星的创建而增加，随着行星的销毁而减少。
这很好地模拟了太阳系中太阳被多个行星共享的概念。

*/

use std::rc::Rc;

#[derive(Debug)]
struct Sun {}

#[derive(Debug)]
enum Planet {
    Mercury(Rc<Sun>),
    Venus(Rc<Sun>),
    Earth(Rc<Sun>),
    Mars(Rc<Sun>),
    Jupiter(Rc<Sun>),
    Saturn(Rc<Sun>),
    Uranus(Rc<Sun>),
    Neptune(Rc<Sun>),
}

impl Planet {
    fn details(&self) {
        println!("Hi from {:?}!", self)
    }
}

fn main() {
    let sun = Rc::new(Sun {});
    println!("reference count = {}", Rc::strong_count(&sun)); // 1 reference

    let mercury = Planet::Mercury(Rc::clone(&sun));
    println!("reference count = {}", Rc::strong_count(&sun)); // 2 references
    mercury.details();

    let venus = Planet::Venus(Rc::clone(&sun));
    println!("reference count = {}", Rc::strong_count(&sun)); // 3 references
    venus.details();

    let earth = Planet::Earth(Rc::clone(&sun));
    println!("reference count = {}", Rc::strong_count(&sun)); // 4 references
    earth.details();

    let mars = Planet::Mars(Rc::clone(&sun));
    println!("reference count = {}", Rc::strong_count(&sun)); // 5 references
    mars.details();

    let jupiter = Planet::Jupiter(Rc::clone(&sun));
    println!("reference count = {}", Rc::strong_count(&sun)); // 6 references
    jupiter.details();
    // TODO
    let saturn = Planet::Saturn(Rc::new(Sun {}));
    let saturn = Planet::Saturn(Rc::clone(&sun));
    println!("reference count = {}", Rc::strong_count(&sun)); // 7 references
    saturn.details();

    let uranus = Planet::Uranus(Rc::clone(&sun));
    println!("reference count = {}", Rc::strong_count(&sun)); // 8 references
    uranus.details();

    let neptune = Planet::Neptune(Rc::clone(&sun));
    println!("reference count = {}", Rc::strong_count(&sun)); // 9 references
    neptune.details();

    assert_eq!(Rc::strong_count(&sun), 9);

    drop(neptune);
    println!("reference count = {}", Rc::strong_count(&sun)); // 8 references

    drop(uranus);
    println!("reference count = {}", Rc::strong_count(&sun)); // 7 references

    drop(saturn);
    println!("reference count = {}", Rc::strong_count(&sun)); // 6 references

    drop(jupiter);
    println!("reference count = {}", Rc::strong_count(&sun)); // 5 references

    drop(mars);
    println!("reference count = {}", Rc::strong_count(&sun)); // 4 references

    drop(earth);
    println!("reference count = {}", Rc::strong_count(&sun)); // 3 references

    drop(venus);
    println!("reference count = {}", Rc::strong_count(&sun)); // 2 references

    drop(mercury);
    println!("reference count = {}", Rc::strong_count(&sun)); // 1 reference

    assert_eq!(Rc::strong_count(&sun), 1);
}
