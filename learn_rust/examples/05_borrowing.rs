//! 第 5 课：引用与借用 (References & Borrowing)
//!
//! 运行：cargo run --example 05_borrowing
//!
//! 借用规则（编译器的「借用检查器」会强制执行）：
//! 1. 同一时刻，要么只有「一个可变引用 &mut T」，要么有「任意多个不可变引用 &T」。
//! 2. 引用必须总是有效的（不会有悬垂引用）。
//!
//! 可以类比「读写锁」：多人同时读没问题，但写的时候只能一个人，且没人在读。

fn main() {
    // ---------- 不可变引用 &T ----------
    let s = String::from("hello");
    let len = calculate_length(&s); // 借用 s，不拿走所有权
    println!("'{s}' 的长度是 {len}"); // s 仍然可用

    // ---------- 可变引用 &mut T ----------
    let mut s = String::from("hello");
    change(&mut s);
    println!("修改后: {s}");

    // ---------- 规则 1 的演示 ----------
    let mut text = String::from("abc");
    let r1 = &text;
    let r2 = &text; // 多个不可变引用：OK
    println!("{r1} {r2}");
    // r1、r2 在上一行之后不再使用，它们的借用就「结束」了 (NLL, 非词法生命周期)

    let r3 = &mut text; // 所以这里可以创建可变引用
    r3.push('d');
    println!("{r3}");

    // 下面这样就不行：
    // let r4 = &text;
    // let r5 = &mut text; // ❌ cannot borrow `text` as mutable because it is also borrowed as immutable
    // println!("{r4}");

    // ---------- 切片 (slice)：对集合一部分的引用 ----------
    let sentence = String::from("hello world");
    let word = first_word(&sentence);
    println!("第一个单词: {word}");

    let arr = [1, 2, 3, 4, 5];
    let part: &[i32] = &arr[1..3]; // [2, 3]
    println!("数组切片: {:?}", part);

    // ---------- String vs &str ----------
    // String：拥有所有权、可增长、在堆上
    // &str：字符串切片，是对某段 UTF-8 文本的借用；字符串字面量就是 &'static str
    let literal: &str = "我是字面量";
    let owned: String = literal.to_string();
    let borrowed: &str = &owned; // &String 可以自动转换成 &str
    println!("{literal} / {owned} / {borrowed}");
}

// 参数写成 &str 比 &String 更通用：String 和 &str 都能传进来
fn calculate_length(s: &str) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(", world");
}

fn first_word(s: &str) -> &str {
    for (i, b) in s.bytes().enumerate() {
        if b == b' ' {
            return &s[..i];
        }
    }
    s
}

// ⚠️ 悬垂引用是编译不过的：
// fn dangle() -> &String {
//     let s = String::from("hi");
//     &s // ❌ s 在函数结束时被释放，引用会指向无效内存
// }

// 🏋️ 练习：
// 1. 写 fn last_word(s: &str) -> &str，返回最后一个单词。
// 2. 取消 r4/r5 那三行的注释，读懂报错；再试着调整代码顺序让它编译通过。
