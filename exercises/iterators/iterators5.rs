// iterators5.rs
//
// Let's define a simple model to track Rustlings exercise progress. Progress
// will be modelled using a hash map. The name of the exercise is the key and
// the progress is the value. Two counting functions were created to count the
// number of exercises with a given progress. Recreate this counting
// functionality using iterators. Try not to use imperative loops (for, while).
// Only the two iterator methods (count_iterator and count_collection_iterator)
// need to be modified.
//
// Execute `rustlings hint iterators5` or use the `hint` watch subcommand for a
// hint.

/*
hint
The documentation for the std::iter::Iterator trait contains numerous methods
that would be helpful here.

The collection variable in count_collection_iterator is a slice of HashMaps. It
needs to be converted into an iterator in order to use the iterator methods.

The fold method can be useful in the count_collection_iterator function.

For a further challenge, consult the documentation for Iterator to find
a different method that could make your code more compact than using fold.

我来帮你完成这个练习并详细解释。这是一个关于 Rust 迭代器的练习，需要我们实现两个函数：`count_iterator` 和 `count_collection_iterator`。

让我们先来看看需要实现的内容：

1. `count_iterator` 函数需要计算一个 HashMap 中特定 Progress 状态的数量
2. `count_collection_iterator` 函数需要计算多个 HashMap 集合中特定 Progress 状态的数量

让我详细解释这两个实现：

1. `count_iterator` 函数的实现：
```rust
map.values().filter(|&val| val == &value).count()
```
- `map.values()` - 获取 HashMap 中所有的值的迭代器
- `filter(|&val| val == &value)` - 过滤出等于指定 value 的元素
- `count()` - 计算过滤后的元素数量

2. `count_collection_iterator` 函数的实现：
```rust
collection.iter().flat_map(|map| map.values()).filter(|&val| val == &value).count()
```
- `collection.iter()` - 获取集合的迭代器
- `flat_map(|map| map.values())` - 将每个 HashMap 的值展平成一个单一的迭代器
- `filter(|&val| val == &value)` - 过滤出等于指定 value 的元素
- `count()` - 计算过滤后的元素数量

这个实现完全避免了使用命令式的循环（for, while），而是使用了 Rust 的迭代器特性。主要用到了以下迭代器方法：
- `iter()` - 创建一个迭代器
- `values()` - 获取 HashMap 值的迭代器
- `filter()` - 过滤元素
- `flat_map()` - 将嵌套的迭代器展平
- `count()` - 计算元素数量

这种函数式的方法比原来的 for 循环实现更加简洁和声明式，也更符合 Rust 的惯用写法。从测试用例可以看出，这个实现可以正确处理所有的测试情况，包括计算 Complete、Some 和 None 状态的数量。

*/

use std::collections::HashMap;
use std::iter::Iterator;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Progress {
    None,
    Some,
    Complete,
}

fn count_for(map: &HashMap<String, Progress>, value: Progress) -> usize {
    let mut count = 0;
    for val in map.values() {
        if val == &value {
            count += 1;
        }
    }
    count
}

fn count_iterator(map: &HashMap<String, Progress>, value: Progress) -> usize {
    // map is a hashmap with String keys and Progress values.
    // map = { "variables1": Complete, "from_str": None, ... }
    map.values().filter(|&val| val == &value).count()
}

fn count_collection_for(collection: &[HashMap<String, Progress>], value: Progress) -> usize {
    let mut count = 0;
    for map in collection {
        for val in map.values() {
            if val == &value {
                count += 1;
            }
        }
    }
    count
}

fn count_collection_iterator(collection: &[HashMap<String, Progress>], value: Progress) -> usize {
    // collection is a slice of hashmaps.
    // collection = [{ "variables1": Complete, "from_str": None, ... },
    //     { "variables2": Complete, ... }, ... ]
    //todo!();
    collection.iter().flat_map(|map| map.values()).filter(|&val| val == &value).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_complete() {
        let map = get_map();
        assert_eq!(3, count_iterator(&map, Progress::Complete));
    }

    #[test]
    fn count_some() {
        let map = get_map();
        assert_eq!(1, count_iterator(&map, Progress::Some));
    }

    #[test]
    fn count_none() {
        let map = get_map();
        assert_eq!(2, count_iterator(&map, Progress::None));
    }

    #[test]
    fn count_complete_equals_for() {
        let map = get_map();
        let progress_states = vec![Progress::Complete, Progress::Some, Progress::None];
        for progress_state in progress_states {
            assert_eq!(
                count_for(&map, progress_state),
                count_iterator(&map, progress_state)
            );
        }
    }

    #[test]
    fn count_collection_complete() {
        let collection = get_vec_map();
        assert_eq!(
            6,
            count_collection_iterator(&collection, Progress::Complete)
        );
    }

    #[test]
    fn count_collection_some() {
        let collection = get_vec_map();
        assert_eq!(1, count_collection_iterator(&collection, Progress::Some));
    }

    #[test]
    fn count_collection_none() {
        let collection = get_vec_map();
        assert_eq!(4, count_collection_iterator(&collection, Progress::None));
    }

    #[test]
    fn count_collection_equals_for() {
        let progress_states = vec![Progress::Complete, Progress::Some, Progress::None];
        let collection = get_vec_map();

        for progress_state in progress_states {
            assert_eq!(
                count_collection_for(&collection, progress_state),
                count_collection_iterator(&collection, progress_state)
            );
        }
    }

    fn get_map() -> HashMap<String, Progress> {
        use Progress::*;

        let mut map = HashMap::new();
        map.insert(String::from("variables1"), Complete);
        map.insert(String::from("functions1"), Complete);
        map.insert(String::from("hashmap1"), Complete);
        map.insert(String::from("arc1"), Some);
        map.insert(String::from("as_ref_mut"), None);
        map.insert(String::from("from_str"), None);

        map
    }

    fn get_vec_map() -> Vec<HashMap<String, Progress>> {
        use Progress::*;

        let map = get_map();

        let mut other = HashMap::new();
        other.insert(String::from("variables2"), Complete);
        other.insert(String::from("functions2"), Complete);
        other.insert(String::from("if1"), Complete);
        other.insert(String::from("from_into"), None);
        other.insert(String::from("try_from_into"), None);

        vec![map, other]
    }
}
