//! 每道题标注了对应的课程编号，做题前先看完那一课。
//! 卡住了？先读编译器报错，再看 SOLUTIONS.md。

use std::collections::HashMap;

/// ex01（第 3 课）：返回第 n 个斐波那契数，fib(0) = 0, fib(1) = 1
pub fn fib(n: u32) -> u64 {
    todo!("实现斐波那契，n = {n}")
}

/// ex02（第 3 课）：FizzBuzz。3 的倍数返回 "Fizz"，5 的倍数返回 "Buzz"，
/// 同时是 3 和 5 的倍数返回 "FizzBuzz"，否则返回数字本身的字符串
pub fn fizzbuzz(n: u32) -> String {
    todo!("n = {n}")
}

/// ex03（第 5 课）：返回句子中的最后一个单词（按空格分隔）
pub fn last_word(s: &str) -> &str {
    todo!("s = {s}")
}

/// ex04（第 6 课）：矩形结构体，实现 area 和 perimeter
#[derive(Debug)]
pub struct Rect {
    pub w: u32,
    pub h: u32,
}

impl Rect {
    pub fn area(&self) -> u32 {
        todo!()
    }

    pub fn perimeter(&self) -> u32 {
        todo!()
    }
}

/// ex05（第 7 课）：返回切片中第一个偶数，没有则返回 None
pub fn first_even(v: &[i32]) -> Option<i32> {
    todo!("v = {v:?}")
}

/// ex06（第 7 课）：计算形状面积
pub enum Shape {
    Circle(f64),     // 半径
    Rect(f64, f64),  // 宽、高
    Square(f64),     // 边长
}

pub fn shape_area(shape: &Shape) -> f64 {
    let _ = shape;
    todo!("用 match 处理三种情况，圆周率用 std::f64::consts::PI")
}

/// ex07（第 8 课）：统计每个单词出现的次数
pub fn word_count(text: &str) -> HashMap<String, usize> {
    todo!("text = {text}")
}

/// ex08（第 9 课）：把两个字符串解析成整数并求和，任一解析失败就返回错误
/// 提示：用 ? 运算符
pub fn sum_str(a: &str, b: &str) -> Result<i32, std::num::ParseIntError> {
    todo!("a = {a}, b = {b}")
}

/// ex09（第 10 课）：返回切片中的最大值（泛型版本），空切片返回 None
pub fn largest<T: PartialOrd + Copy>(list: &[T]) -> Option<T> {
    let _ = list;
    todo!()
}

/// ex10（第 11 课）：只用迭代器（不用 for / while），
/// 求 1..=n 中所有能被 3 或 5 整除的数之和
pub fn sum_multiples(n: u32) -> u32 {
    todo!("n = {n}")
}

/// ex11（第 12 课）：返回文本中最长的单词；长度相同时返回先出现的那个
pub fn longest_word(text: &str) -> &str {
    todo!("text = {text}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex01_fib() {
        assert_eq!(fib(0), 0);
        assert_eq!(fib(1), 1);
        assert_eq!(fib(10), 55);
        assert_eq!(fib(50), 12586269025);
    }

    #[test]
    fn ex02_fizzbuzz() {
        assert_eq!(fizzbuzz(1), "1");
        assert_eq!(fizzbuzz(3), "Fizz");
        assert_eq!(fizzbuzz(10), "Buzz");
        assert_eq!(fizzbuzz(15), "FizzBuzz");
    }

    #[test]
    fn ex03_last_word() {
        assert_eq!(last_word("hello world"), "world");
        assert_eq!(last_word("rust"), "rust");
    }

    #[test]
    fn ex04_rect() {
        let r = Rect { w: 3, h: 4 };
        assert_eq!(r.area(), 12);
        assert_eq!(r.perimeter(), 14);
    }

    #[test]
    fn ex05_first_even() {
        assert_eq!(first_even(&[1, 3, 4, 6]), Some(4));
        assert_eq!(first_even(&[1, 3, 5]), None);
        assert_eq!(first_even(&[]), None);
    }

    #[test]
    fn ex06_shape_area() {
        assert!((shape_area(&Shape::Circle(1.0)) - std::f64::consts::PI).abs() < 1e-9);
        assert_eq!(shape_area(&Shape::Rect(2.0, 3.0)), 6.0);
        assert_eq!(shape_area(&Shape::Square(4.0)), 16.0);
    }

    #[test]
    fn ex07_word_count() {
        let m = word_count("a b a c a b");
        assert_eq!(m["a"], 3);
        assert_eq!(m["b"], 2);
        assert_eq!(m["c"], 1);
        assert_eq!(m.len(), 3);
    }

    #[test]
    fn ex08_sum_str() {
        assert_eq!(sum_str("1", "2"), Ok(3));
        assert!(sum_str("1", "x").is_err());
    }

    #[test]
    fn ex09_largest() {
        assert_eq!(largest(&[3, 7, 2]), Some(7));
        assert_eq!(largest(&['a', 'z', 'm']), Some('z'));
        assert_eq!(largest::<i32>(&[]), None);
    }

    #[test]
    fn ex10_sum_multiples() {
        assert_eq!(sum_multiples(10), 33);
        assert_eq!(sum_multiples(100), 2418);
    }

    #[test]
    fn ex11_longest_word() {
        assert_eq!(longest_word("I love rustaceans so much"), "rustaceans");
        assert_eq!(longest_word("ab cd"), "ab");
    }
}
