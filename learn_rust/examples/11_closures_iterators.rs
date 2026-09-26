//! 第 11 课：闭包 (Closure) 与迭代器 (Iterator)
//!
//! 运行：cargo run --example 11_closures_iterators

#[derive(Debug, Clone)]
struct Student {
    name: String,
    score: u32,
}

fn apply<F: Fn(i32) -> i32>(f: F, x: i32) -> i32 {
    f(x)
}

fn make_adder(n: i32) -> impl Fn(i32) -> i32 {
    move |x| x + n // move：把 n 的所有权移进闭包
}

fn main() {
    // ---------- 闭包：可以捕获环境变量的匿名函数 ----------
    let factor = 3;
    let times = |x: i32| x * factor; // 捕获了 factor
    println!("times(5) = {}", times(5));
    println!("apply = {}", apply(|x| x - 1, 10));

    let add10 = make_adder(10);
    println!("add10(5) = {}", add10(5));

    // 可变捕获需要 FnMut
    let mut count = 0;
    let mut inc = || count += 1;
    inc();
    inc();
    println!("count = {count}");

    // ---------- 迭代器：惰性求值，链式调用 ----------
    let students = vec![
        Student { name: "小红".into(), score: 92 },
        Student { name: "小明".into(), score: 58 },
        Student { name: "小刚".into(), score: 75 },
        Student { name: "小美".into(), score: 88 },
    ];

    // 及格学生的名字
    let passed: Vec<&str> = students
        .iter()
        .filter(|s| s.score >= 60)
        .map(|s| s.name.as_str())
        .collect();
    println!("及格: {:?}", passed);

    // 平均分
    let total: u32 = students.iter().map(|s| s.score).sum();
    println!("平均分: {:.1}", total as f64 / students.len() as f64);

    // 最高分
    if let Some(top) = students.iter().max_by_key(|s| s.score) {
        println!("最高分: {} {}", top.name, top.score);
    }

    // any / all / find / position
    println!("有人满分吗? {}", students.iter().any(|s| s.score == 100));
    println!("全部及格吗? {}", students.iter().all(|s| s.score >= 60));
    println!("第一个 80+ 的: {:?}", students.iter().find(|s| s.score > 80).map(|s| &s.name));

    // 排序（sort_by 需要可变 Vec）
    let mut sorted = students.clone();
    sorted.sort_by(|a, b| b.score.cmp(&a.score));
    for (rank, s) in sorted.iter().enumerate() {
        println!("第{}名 {} {}", rank + 1, s.name, s.score);
    }

    // zip / take / skip / rev / fold
    let a = [1, 2, 3];
    let b = ["一", "二", "三"];
    let zipped: Vec<(i32, &str)> = a.iter().copied().zip(b.iter().copied()).collect();
    println!("zip: {:?}", zipped);
    println!("take/skip: {:?}", (1..10).skip(2).take(3).collect::<Vec<_>>());
    println!("rev: {:?}", (1..=5).rev().collect::<Vec<_>>());
    println!("fold 阶乘 5! = {}", (1..=5).fold(1, |acc, x| acc * x));

    // iter() 借用元素；iter_mut() 可变借用；into_iter() 拿走所有权
    let names: Vec<String> = students.into_iter().map(|s| s.name).collect();
    println!("into_iter 后: {:?}", names);
    // println!("{:?}", students); // ❌ students 已被消耗
}

// 🏋️ 练习：
// 1. 用一行迭代器链求 1..=100 中所有能被 3 或 5 整除的数之和（答案 2418）。
// 2. 实现一个自己的迭代器：struct Counter { n: u32 }，impl Iterator for Counter，从 1 数到 5。
