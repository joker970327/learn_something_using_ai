//! 第 9 课：错误处理 —— panic! 与 Result
//!
//! 运行：cargo run --example 09_error_handling
//!
//! Rust 把错误分两类：
//! - 不可恢复错误 → panic!（程序崩溃，通常代表 bug）
//! - 可恢复错误   → Result<T, E>：enum Result<T, E> { Ok(T), Err(E) }

use std::fmt;
use std::num::ParseIntError;

// 解析一个数字字符串并翻倍
fn double(s: &str) -> Result<i32, ParseIntError> {
    // ? 运算符：如果是 Err 就立即返回这个 Err；如果是 Ok 就取出里面的值
    let n: i32 = s.trim().parse()?;
    Ok(n * 2)
}

// ---------- 自定义错误类型 ----------
#[derive(Debug)]
enum AgeError {
    NotANumber(ParseIntError),
    OutOfRange(i32),
}

// 实现 Display，让错误可以友好地打印
impl fmt::Display for AgeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AgeError::NotANumber(e) => write!(f, "不是数字: {e}"),
            AgeError::OutOfRange(n) => write!(f, "年龄 {n} 不在 0~150 范围内"),
        }
    }
}

impl std::error::Error for AgeError {}

// 实现 From 后，? 会自动把 ParseIntError 转成 AgeError
impl From<ParseIntError> for AgeError {
    fn from(e: ParseIntError) -> Self {
        AgeError::NotANumber(e)
    }
}

fn parse_age(s: &str) -> Result<i32, AgeError> {
    let n: i32 = s.parse()?; // ParseIntError 自动转换为 AgeError
    if !(0..=150).contains(&n) {
        return Err(AgeError::OutOfRange(n));
    }
    Ok(n)
}

// main 也可以返回 Result；Box<dyn Error> 可以装下任何错误类型
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 用 match 处理
    for input in ["21", "abc"] {
        match double(input) {
            Ok(v) => println!("double({input}) = {v}"),
            Err(e) => println!("double({input}) 出错: {e}"),
        }
    }

    for input in ["30", "200", "x"] {
        match parse_age(input) {
            Ok(age) => println!("年龄 = {age}"),
            Err(e) => println!("错误: {e}"),
        }
    }

    // 常用便捷方法
    let r: Result<i32, String> = Err("糟糕".into());
    println!("unwrap_or: {}", r.clone().unwrap_or(-1));
    println!("map_err: {:?}", r.map_err(|e| e.len()));

    // unwrap / expect：确定不会出错时使用，出错就 panic
    let n: i32 = "42".parse().expect("这必须是数字");
    println!("expect 得到 {n}");

    // Result 和 Option 可以互相转换
    let maybe: Option<i32> = "7".parse::<i32>().ok();
    println!("ok(): {:?}", maybe);

    // 在 main 里也能用 ?
    let age = parse_age("18")?;
    println!("main 中用 ? 解析到 {age}");

    Ok(())
}

// 🏋️ 练习：
// 1. 写 fn sum_str(a: &str, b: &str) -> Result<i32, ParseIntError>，用 ? 实现。
// 2. 把 parse_age("200")? 放进 main 运行，看看 main 返回 Err 时会输出什么。
