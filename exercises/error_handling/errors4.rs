// errors4.rs
//
// Execute `rustlings hint errors4` or use the `hint` watch subcommand for a
// hint.

/*
这个问题涉及到创建一个正的非零整数类型。根据测试用例，我们可以看到：
当输入为正数时（如10）应该返回 Ok
当输入为负数时应该返回 Err(CreationError::Negative)
当输入为零时应该返回 Err(CreationError::Zero)
当前的实现有问题，因为它总是返回 Ok 而没有进行任何验证。让我来修复这个问题：
添加了对输入值的验证：
如果值小于 0，返回 Err(CreationError::Negative)
如果值等于 0，返回 Err(CreationError::Zero)
如果值大于 0，返回 Ok(PositiveNonzeroInteger(value as u64))
这样修改后，PositiveNonzeroInteger 类型就能保证：
只能创建正数（通过错误处理拒绝负数）
不能为零（通过错误处理拒绝零）
所有合法的值都会被正确转换为 u64 类型存储
*/

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        if value < 0 {
            Err(CreationError::Negative)
        } else if value == 0 {
            Err(CreationError::Zero)
        } else {
            Ok(PositiveNonzeroInteger(value as u64))
        }
    }
}

#[test]
fn test_creation() {
    assert!(PositiveNonzeroInteger::new(10).is_ok());
    assert_eq!(
        Err(CreationError::Negative),
        PositiveNonzeroInteger::new(-10)
    );
    assert_eq!(Err(CreationError::Zero), PositiveNonzeroInteger::new(0));
}
