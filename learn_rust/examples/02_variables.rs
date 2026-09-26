//! 第 2 课：变量、可变性与基本类型
//!
//! 运行：cargo run --example 02_variables

fn main() {
    // ---------- 1. 变量默认不可变 ----------
    let x = 5;
    // x = 6; // ❌ 编译错误：cannot assign twice to immutable variable
    println!("x = {x}");

    // 需要修改就加 mut
    let mut y = 5;
    y += 1;
    println!("y = {y}");

    // ---------- 2. 遮蔽 (shadowing) ----------
    // 用 let 重新声明同名变量，可以改变类型
    let spaces = "   ";
    let spaces = spaces.len(); // 现在 spaces 是 usize
    println!("空格数 = {spaces}");

    // ---------- 3. 常量 ----------
    // 必须标注类型，命名用全大写
    const MAX_POINTS: u32 = 100_000; // 下划线只是为了好读
    println!("MAX_POINTS = {MAX_POINTS}");

    // ---------- 4. 标量类型 ----------
    let a: i32 = -42; // 有符号整数：i8 i16 i32 i64 i128 isize
    let b: u8 = 255; // 无符号整数：u8 u16 u32 u64 u128 usize
    let c: f64 = 2.5; // 浮点数：f32 f64（默认 f64）
    let d: bool = true; // 布尔
    let e: char = '🦀'; // char 是 4 字节 Unicode 字符，用单引号
    println!("{a} {b} {c} {d} {e}");

    // 类型转换必须显式使用 as
    let f = a as f64 + c;
    println!("f = {f}");

    // 整数溢出：debug 模式下会 panic，可以用 checked_/wrapping_/saturating_ 系列方法
    println!("255u8.checked_add(1) = {:?}", b.checked_add(1)); // None
    println!("255u8.wrapping_add(1) = {}", b.wrapping_add(1)); // 0
    println!("255u8.saturating_add(1) = {}", b.saturating_add(1)); // 255

    // ---------- 5. 复合类型 ----------
    // 元组：长度固定，元素类型可以不同
    let tup: (i32, f64, char) = (500, 6.4, 'z');
    let (t1, t2, t3) = tup; // 解构
    println!("元组: {t1} {t2} {t3}, 第一个元素也可以用 tup.0 = {}", tup.0);

    // 数组：长度固定，元素类型相同，存放在栈上
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    let zeros = [0; 3]; // [0, 0, 0]
    println!("arr[0] = {}, 长度 = {}, zeros = {:?}", arr[0], arr.len(), zeros);
}

// 🏋️ 练习：
// 1. 取消注释 `x = 6;`，运行看看编译器的报错信息——Rust 的报错非常友好，要学会读它。
// 2. 声明一个 i8 变量，值为 127，分别用 checked_add / wrapping_add 加 1，打印结果。
