// quiz3.rs
//
// This quiz tests:
// - Generics
// - Traits
//
// An imaginary magical school has a new report card generation system written
// in Rust! Currently the system only supports creating report cards where the
// student's grade is represented numerically (e.g. 1.0 -> 5.5). However, the
// school also issues alphabetical grades (A+ -> F-) and needs to be able to
// print both types of report card!
//
// Make the necessary code changes in the struct ReportCard and the impl block
// to support alphabetical report cards. Change the Grade in the second test to
// "A+" to show that your changes allow alphabetical grades.
//To find the best solution to this challenge you're going to need to think back to your
//knowledge of traits, specifically Trait Bound Syntax
// -  you may also need this: `use std::fmt::Display;`.
// Execute `rustlings hint quiz3` or use the `hint` watch subcommand for a hint.

/*
让我帮你完成这个关于成绩单的练习。这个练习要求我们修改 ReportCard 结构体，使其能够同时支持数字成绩和字母成绩。
我们需要：
使用泛型来让 grade 字段支持不同类型
添加 trait bound 来确保这些类型可以被格式化为字符串
修改测试用例中的字母成绩
让我来修改代码：
做了以下修改：
添加了 use std::fmt::Display; 来引入 Display trait
将 ReportCard 改为泛型结构体 ReportCard<T: Display>，其中 T 是成绩的类型
使用 Display trait bound 确保任何用作成绩的类型都可以被格式化为字符串
相应地修改了 impl 块，使其支持泛型
在第二个测试用例中，将数字成绩 2.1 改为字母成绩 "A+"
这些修改允许 ReportCard 结构体同时支持：
数字成绩（如第一个测试中的 2.1）
字母成绩（如第二个测试中的 "A+"）
现在代码应该可以通过所有测试了。这个解决方案的优点是：
类型安全：编译器会确保只有实现了 Display trait 的类型才能用作成绩
*/

use std::fmt::Display;

pub struct ReportCard<T: Display> {
    pub grade: T,
    pub student_name: String,
    pub student_age: u8,
}

impl<T: Display> ReportCard<T> {
    pub fn print(&self) -> String {
        format!("{} ({}) - achieved a grade of {}",
            &self.student_name, &self.student_age, &self.grade)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_numeric_report_card() {
        let report_card = ReportCard {
            grade: 2.1,
            student_name: "Tom Wriggle".to_string(),
            student_age: 12,
        };
        assert_eq!(
            report_card.print(),
            "Tom Wriggle (12) - achieved a grade of 2.1"
        );
    }

    #[test]
    fn generate_alphabetic_report_card() {
        let report_card = ReportCard {
            grade: "A+",
            student_name: "Gary Plotter".to_string(),
            student_age: 11,
        };
        assert_eq!(
            report_card.print(),
            "Gary Plotter (11) - achieved a grade of A+"
        );
    }
}
