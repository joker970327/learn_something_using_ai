//! 第 7 课：枚举 (enum)、Option 与模式匹配 (match)
//!
//! 运行：cargo run --example 07_enums_match

// Rust 的枚举每个变体都可以携带不同类型的数据（代数数据类型）
#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(u8, u8, u8),
}

impl Message {
    fn describe(&self) -> String {
        // match 必须「穷尽」所有情况，漏了会编译报错
        match self {
            Message::Quit => "退出".to_string(),
            Message::Move { x, y } => format!("移动到 ({x}, {y})"),
            Message::Write(text) => format!("写入: {text}"),
            Message::ChangeColor(r, g, b) => format!("改颜色为 #{r:02x}{g:02x}{b:02x}"),
        }
    }
}

// Rust 没有 null！用 Option<T> 表达「可能没有值」：
// enum Option<T> { Some(T), None }
fn divide(a: f64, b: f64) -> Option<f64> {
    if b == 0.0 { None } else { Some(a / b) }
}

fn main() {
    let msgs = vec![
        Message::Quit,
        Message::Move { x: 1, y: 2 },
        Message::Write(String::from("你好")),
        Message::ChangeColor(255, 128, 0),
    ];
    for m in &msgs {
        println!("{:?} → {}", m, m.describe());
    }

    // ---------- 处理 Option ----------
    match divide(10.0, 2.0) {
        Some(v) => println!("10 / 2 = {v}"),
        None => println!("不能除以 0"),
    }

    // if let：只关心一种情况时更简洁
    if let Some(v) = divide(1.0, 0.0) {
        println!("结果 {v}");
    } else {
        println!("1 / 0 没有结果");
    }

    // 常用方法
    let x: Option<i32> = Some(5);
    let none: Option<i32> = None;
    println!("unwrap_or: {} {}", x.unwrap_or(0), none.unwrap_or(0));
    println!("map: {:?}", x.map(|v| v * 2)); // Some(10)
    println!("is_some: {}", none.is_some());

    // let else：取不到就提前返回/跳出
    let Some(val) = x else {
        panic!("不会走到这里");
    };
    println!("let else 拿到 {val}");

    // ---------- match 的更多模式 ----------
    for n in [0, 3, 7, 15, 42] {
        let desc = match n {
            0 => "零",
            1..=5 => "小",
            6 | 7 | 8 => "中",
            x if x % 2 == 1 => "大奇数", // 匹配守卫 (guard)
            _ => "其他", // _ 通配符
        };
        println!("{n}: {desc}");
    }

    // 解构元组
    let point = (0, -2);
    match point {
        (0, y) => println!("在 y 轴上, y = {y}"),
        (x, 0) => println!("在 x 轴上, x = {x}"),
        _ => println!("其他位置"),
    }
}

// 🏋️ 练习：
// 1. 定义 enum Shape { Circle(f64), Rect(f64, f64), Triangle(f64, f64, f64) }，
//    实现 area(&self) -> f64（三角形用海伦公式）。
// 2. 写 fn first_even(v: &[i32]) -> Option<i32>，返回第一个偶数。
