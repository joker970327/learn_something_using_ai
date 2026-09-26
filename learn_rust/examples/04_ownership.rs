//! 第 4 课：所有权 (Ownership) —— Rust 最核心的概念！
//!
//! 运行：cargo run --example 04_ownership
//!
//! 三条规则：
//! 1. Rust 中每个值都有一个「所有者」(owner) 变量。
//! 2. 同一时刻只能有一个所有者。
//! 3. 所有者离开作用域时，值被自动释放 (drop)。
//!
//! 正是这三条规则，让 Rust 不需要垃圾回收 (GC)，也不需要手动 free，
//! 同时保证内存安全。

fn main() {
    // ---------- 作用域 ----------
    {
        let s = String::from("hello"); // s 从这里开始有效
        println!("{s}");
    } // s 离开作用域，内存被自动释放
    // println!("{s}"); // ❌ s 已经不存在了

    // ---------- 移动 (move) ----------
    // String 的数据在堆上。赋值时所有权会「移动」，而不是复制
    let s1 = String::from("hello");
    let s2 = s1; // s1 的所有权移动给 s2，s1 失效
    // println!("{s1}"); // ❌ borrow of moved value: `s1`
    println!("s2 = {s2}");

    // 如果真的想要两份数据，用 clone（深拷贝，有开销）
    let s3 = s2.clone();
    println!("s2 = {s2}, s3 = {s3}");

    // ---------- 复制 (Copy) ----------
    // 整数、浮点、bool、char 以及只包含它们的元组都实现了 Copy，
    // 它们在栈上、体积小，赋值时直接复制，原变量依然可用
    let a = 5;
    let b = a;
    println!("a = {a}, b = {b}");

    // ---------- 函数与所有权 ----------
    let s = String::from("world");
    takes_ownership(s); // s 的所有权移入函数
    // println!("{s}"); // ❌ s 已经被移走了

    let n = 10;
    makes_copy(n); // i32 是 Copy，n 仍然可用
    println!("n 仍然可用: {n}");

    // 函数返回值也可以把所有权交还回来
    let s = gives_ownership();
    let s = takes_and_gives_back(s);
    println!("拿回来了: {s}");

    // 每次都这样传来传去太麻烦了 → 下一课：引用与借用
}

fn takes_ownership(some_string: String) {
    println!("拿到了: {some_string}");
} // some_string 在这里被 drop

fn makes_copy(some_integer: i32) {
    println!("复制了: {some_integer}");
}

fn gives_ownership() -> String {
    String::from("新字符串")
}

fn takes_and_gives_back(s: String) -> String {
    s
}

// 🏋️ 练习：
// 1. 把代码里带 ❌ 的注释逐个取消，运行，仔细阅读编译器报错。
// 2. 思考：为什么 String 不能 Copy，而 i32 可以？（提示：想想「两个所有者同时 free 同一块堆内存」会怎样）
