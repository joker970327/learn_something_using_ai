//! 第 3 课：函数与控制流
//!
//! 运行：cargo run --example 03_functions_control

// 参数必须标注类型；-> 后面是返回类型
fn add(a: i32, b: i32) -> i32 {
    a + b // 最后一个「表达式」(没有分号) 就是返回值
}

// 也可以用 return 提前返回
fn abs(n: i32) -> i32 {
    if n < 0 {
        return -n;
    }
    n
}

fn main() {
    println!("add(2, 3) = {}", add(2, 3));
    println!("abs(-7) = {}", abs(-7));

    // ---------- 语句 vs 表达式 ----------
    // Rust 中「块」{} 也是表达式，有值
    let y = {
        let x = 3;
        x + 1 // 没有分号 → 这个块的值是 4
    };
    println!("y = {y}");

    // ---------- if 是表达式 ----------
    let n = 7;
    let kind = if n % 2 == 0 { "偶数" } else { "奇数" };
    println!("{n} 是{kind}");

    // ---------- loop：无限循环，可以用 break 带回一个值 ----------
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("loop 结果 = {result}");

    // ---------- while ----------
    let mut n = 3;
    while n > 0 {
        println!("{n}!");
        n -= 1;
    }
    println!("发射！🚀");

    // ---------- for：最常用 ----------
    for i in 1..=3 {
        // 1..=3 包含 3；1..3 不包含 3
        print!("{i} ");
    }
    println!();

    let fruits = ["苹果", "香蕉", "橘子"];
    for (idx, fruit) in fruits.iter().enumerate() {
        println!("{idx}: {fruit}");
    }

    // 循环标签：跳出外层循环
    'outer: for i in 0..5 {
        for j in 0..5 {
            if i * j == 6 {
                println!("找到 i={i}, j={j}");
                break 'outer;
            }
        }
    }
}

// 🏋️ 练习：
// 1. 写一个函数 fn fib(n: u32) -> u64，返回第 n 个斐波那契数。
// 2. 用 for 循环打印 1..=15 的 FizzBuzz（3 的倍数打印 Fizz，5 的倍数打印 Buzz，都是打印 FizzBuzz）。
