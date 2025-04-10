// try_from_into.rs
//
// TryFrom is a simple and safe type conversion that may fail in a controlled
// way under some circumstances. Basically, this is the same as From. The main
// difference is that this should return a Result type instead of the target
// type itself. You can read more about it at
// https://doc.rust-lang.org/std/convert/trait.TryFrom.html
//
// Execute `rustlings hint try_from_into` or use the `hint` watch subcommand for
// a hint.

/*
hint
Follow the steps provided right before the `TryFrom` implementation.
You can also use the example at https://doc.rust-lang.org/std/convert/trait.TryFrom.html

Is there an implementation of `TryFrom` in the standard library that
can both do the required integer conversion and check the range of the input?

Another hint: Look at the test cases to see which error variants to return.

Yet another hint: You can use the `map_err` or `or` methods of `Result` to
convert errors.

Yet another hint: If you would like to propagate errors by using the `?`
operator in your solution, you might want to look at
https://doc.rust-lang.org/stable/rust-by-example/error/multiple_error_types/reenter_question_mark.html

Challenge: Can you make the `TryFrom` implementations generic over many integer types?
好的，让我们将 `TryFrom` 实现改为泛型版本，以支持多种整数类型。
我们需要使用 `TryFrom` trait 来安全地转换整数类型：

// 为所有整数类型实现 TryFrom
impl<T> TryFrom<(T, T, T)> for Color xxx
impl<T, const N: usize> TryFrom<[T; N]> for Color xxx
impl<T> TryFrom<&[T]> for Color xxx
让我解释一下这个泛型实现：

1. **泛型约束**：
   ```rust
   where
       T: TryInto<u8> + Copy,
       T::Error: Into<TryFromIntError>,
   ```
   - `TryInto<u8>`：确保类型可以安全地转换为 `u8`
   - `Copy`：允许值被复制
   - `T::Error: Into<TryFromIntError>`：确保错误类型可以转换为 `TryFromIntError`

2. **元组实现**：
   - 使用泛型类型 `T` 替代具体的 `i16`
   - 使用 `try_into()` 进行安全的类型转换
   - 使用 `?` 操作符处理转换错误

3. **数组实现**：
   - 使用泛型类型 `T` 和常量泛型 `N`
   - 检查数组长度是否为 3
   - 使用 `try_into()` 进行安全的类型转换

4. **切片实现**：
   - 使用泛型类型 `T`
   - 检查切片长度
   - 使用 `try_into()` 进行安全的类型转换

这个泛型实现的优点：
1. 支持所有可以转换为 `u8` 的整数类型
2. 更安全的类型转换（使用 `try_into` 而不是 `as`）
3. 更好的错误处理
4. 代码更简洁和通用

现在这个实现可以处理：
- 有符号整数（i8, i16, i32, i64, i128）
- 无符号整数（u8, u16, u32, u64, u128）
- 任何实现了 `TryInto<u8>` 的类型

这样的实现更加灵活和安全，同时保持了原有的功能。
*/

use std::convert::{TryFrom, TryInto};
use std::num::TryFromIntError;

#[derive(Debug, PartialEq)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

// We will use this error type for these `TryFrom` conversions.
#[derive(Debug, PartialEq)]
enum IntoColorError {
    // Incorrect length of slice
    BadLen,
    // Integer conversion error
    IntConversion,
}


// Your task is to complete this implementation and return an Ok result of inner
// type Color. You need to create an implementation for a tuple of three
// integers, an array of three integers, and a slice of integers.
//
// Note that the implementation for tuple and array will be checked at compile
// time, but the slice implementation needs to check the slice length! Also note
// that correct RGB color values must be integers in the 0..=255 range.

// 为所有整数类型实现 TryFrom
impl<T> TryFrom<(T, T, T)> for Color
where
    T: TryInto<u8> + Copy,
    T::Error: Into<TryFromIntError>,
{
    type Error = IntoColorError;
    fn try_from(tuple: (T, T, T)) -> Result<Self, Self::Error> {
        let (red, green, blue) = tuple;
        Ok(Color {
            red: red.try_into().map_err(|_| IntoColorError::IntConversion)?,
            green: green.try_into().map_err(|_| IntoColorError::IntConversion)?,
            blue: blue.try_into().map_err(|_| IntoColorError::IntConversion)?,
        })
    }
}

impl<T, const N: usize> TryFrom<[T; N]> for Color
where
    T: TryInto<u8> + Copy,
    T::Error: Into<TryFromIntError>,
{
    type Error = IntoColorError;
    fn try_from(arr: [T; N]) -> Result<Self, Self::Error> {
        if N != 3 {
            return Err(IntoColorError::BadLen);
        }
        Ok(Color {
            red: arr[0].try_into().map_err(|_| IntoColorError::IntConversion)?,
            green: arr[1].try_into().map_err(|_| IntoColorError::IntConversion)?,
            blue: arr[2].try_into().map_err(|_| IntoColorError::IntConversion)?,
        })
    }
}

impl<T> TryFrom<&[T]> for Color
where
    T: TryInto<u8> + Copy,
    T::Error: Into<TryFromIntError>,
{
    type Error = IntoColorError;
    fn try_from(slice: &[T]) -> Result<Self, Self::Error> {
        if slice.len() != 3 {
            return Err(IntoColorError::BadLen);
        }
        Ok(Color {
            red: slice[0].try_into().map_err(|_| IntoColorError::IntConversion)?,
            green: slice[1].try_into().map_err(|_| IntoColorError::IntConversion)?,
            blue: slice[2].try_into().map_err(|_| IntoColorError::IntConversion)?,
        })
    }
}

fn main() {
    // Use the `try_from` function
    let c1 = Color::try_from((183, 65, 14));
    println!("{:?}", c1);

    // Since TryFrom is implemented for Color, we should be able to use TryInto
    let c2: Result<Color, _> = [183, 65, 14].try_into();
    println!("{:?}", c2);

    let v = vec![183, 65, 14];
    // With slice we should use `try_from` function
    let c3 = Color::try_from(&v[..]);
    println!("{:?}", c3);
    // or take slice within round brackets and use TryInto
    let c4: Result<Color, _> = (&v[..]).try_into();
    println!("{:?}", c4);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tuple_out_of_range_positive() {
        assert_eq!(
            Color::try_from((256, 1000, 10000)),
            Err(IntoColorError::IntConversion)
        );
    }
    #[test]
    fn test_tuple_out_of_range_negative() {
        assert_eq!(
            Color::try_from((-1, -10, -256)),
            Err(IntoColorError::IntConversion)
        );
    }
    #[test]
    fn test_tuple_sum() {
        assert_eq!(
            Color::try_from((-1, 255, 255)),
            Err(IntoColorError::IntConversion)
        );
    }
    #[test]
    fn test_tuple_correct() {
        let c: Result<Color, _> = (183, 65, 14).try_into();
        assert!(c.is_ok());
        assert_eq!(
            c.unwrap(),
            Color {
                red: 183,
                green: 65,
                blue: 14
            }
        );
    }
    #[test]
    fn test_array_out_of_range_positive() {
        let c: Result<Color, _> = [1000, 10000, 256].try_into();
        assert_eq!(c, Err(IntoColorError::IntConversion));
    }
    #[test]
    fn test_array_out_of_range_negative() {
        let c: Result<Color, _> = [-10, -256, -1].try_into();
        assert_eq!(c, Err(IntoColorError::IntConversion));
    }
    #[test]
    fn test_array_sum() {
        let c: Result<Color, _> = [-1, 255, 255].try_into();
        assert_eq!(c, Err(IntoColorError::IntConversion));
    }
    #[test]
    fn test_array_correct() {
        let c: Result<Color, _> = [183, 65, 14].try_into();
        assert!(c.is_ok());
        assert_eq!(
            c.unwrap(),
            Color {
                red: 183,
                green: 65,
                blue: 14
            }
        );
    }
    #[test]
    fn test_slice_out_of_range_positive() {
        let arr = [10000, 256, 1000];
        assert_eq!(
            Color::try_from(&arr[..]),
            Err(IntoColorError::IntConversion)
        );
    }
    #[test]
    fn test_slice_out_of_range_negative() {
        let arr = [-256, -1, -10];
        assert_eq!(
            Color::try_from(&arr[..]),
            Err(IntoColorError::IntConversion)
        );
    }
    #[test]
    fn test_slice_sum() {
        let arr = [-1, 255, 255];
        assert_eq!(
            Color::try_from(&arr[..]),
            Err(IntoColorError::IntConversion)
        );
    }
    #[test]
    fn test_slice_correct() {
        let v = vec![183, 65, 14];
        let c: Result<Color, _> = Color::try_from(&v[..]);
        assert!(c.is_ok());
        assert_eq!(
            c.unwrap(),
            Color {
                red: 183,
                green: 65,
                blue: 14
            }
        );
    }
    #[test]
    fn test_slice_excess_length() {
        let v = vec![0, 0, 0, 0];
        assert_eq!(Color::try_from(&v[..]), Err(IntoColorError::BadLen));
    }
    #[test]
    fn test_slice_insufficient_length() {
        let v = vec![0, 0];
        assert_eq!(Color::try_from(&v[..]), Err(IntoColorError::BadLen));
    }
}
