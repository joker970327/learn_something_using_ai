//! 第 8 课：常用集合 —— Vec、String、HashMap
//!
//! 运行：cargo run --example 08_collections

use std::collections::HashMap;

fn main() {
    // ================= Vec<T>：可增长数组 =================
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    let v2 = vec![10, 20, 30]; // vec! 宏快速创建

    // 两种读取方式
    let third = &v[2]; // 越界会 panic
    println!("第三个: {third}");
    match v.get(99) {
        // 越界返回 None，更安全
        Some(x) => println!("{x}"),
        None => println!("索引 99 不存在"),
    }

    // 遍历并修改
    for x in &mut v {
        *x *= 10; // * 解引用
    }
    println!("v = {:?}, v2 = {:?}", v, v2);

    // 迭代器三件套：map / filter / collect
    let evens_squared: Vec<i32> = (1..=10).filter(|n| n % 2 == 0).map(|n| n * n).collect();
    println!("偶数的平方: {:?}", evens_squared);
    let sum: i32 = v2.iter().sum();
    println!("v2 总和 = {sum}, 最大值 = {:?}", v2.iter().max());

    // ================= String =================
    let mut s = String::from("你好");
    s.push_str(", Rust");
    s.push('!');
    let s2 = format!("{s} —— {}", "欢迎"); // format! 拼接最方便
    println!("{s2}");

    // ⚠️ String 是 UTF-8 编码，不能用 s[0] 取字符
    // 中文字符占 3 个字节：
    println!("\"{s}\" 字节数 = {}, 字符数 = {}", s.len(), s.chars().count());
    for c in "Rust🦀".chars() {
        print!("[{c}]");
    }
    println!();

    // 常用方法
    let text = "  Hello World  ";
    println!("trim: '{}'", text.trim());
    println!("大写: {}", text.to_uppercase());
    println!("包含 World? {}", text.contains("World"));
    println!("替换: {}", text.replace("World", "Rust"));
    let parts: Vec<&str> = "a,b,c".split(',').collect();
    println!("split: {:?}", parts);

    // ================= HashMap<K, V> =================
    let mut scores: HashMap<String, i32> = HashMap::new();
    scores.insert("蓝队".to_string(), 10);
    scores.insert("红队".to_string(), 50);

    if let Some(score) = scores.get("蓝队") {
        println!("蓝队得分: {score}");
    }

    // entry API：不存在就插入默认值，返回可变引用
    let text = "hello world wonderful world";
    let mut word_count: HashMap<&str, i32> = HashMap::new();
    for word in text.split_whitespace() {
        *word_count.entry(word).or_insert(0) += 1;
    }
    println!("词频: {:?}", word_count); // HashMap 遍历顺序不固定

    for (team, score) in &scores {
        println!("{team}: {score}");
    }
}

// 🏋️ 练习：
// 1. 给定 vec![3, 1, 4, 1, 5, 9, 2, 6]，求中位数（提示：先 .sort()）和众数（用 HashMap 计数）。
// 2. 把一句英文转成 "Pig Latin"：first → irst-fay，apple → apple-hay。
