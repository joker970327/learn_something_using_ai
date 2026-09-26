//! 第 6 课：结构体 (struct) 与方法
//!
//! 运行：cargo run --example 06_structs

// #[derive(Debug)] 让结构体可以用 {:?} 打印
#[derive(Debug, Clone, PartialEq)]
struct User {
    username: String,
    email: String,
    active: bool,
    sign_in_count: u64,
}

// 元组结构体：字段没有名字
#[derive(Debug)]
struct Color(u8, u8, u8);

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// impl 块里定义方法
impl Rectangle {
    // 关联函数（没有 self），类似其他语言的静态方法，常用作构造函数
    fn new(width: u32, height: u32) -> Self {
        Self { width, height } // 字段名和变量名相同时可以简写
    }

    fn square(size: u32) -> Self {
        Self::new(size, size)
    }

    // &self：不可变借用自己
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // &mut self：可变借用自己
    fn scale(&mut self, factor: u32) {
        self.width *= factor;
        self.height *= factor;
    }
}

fn main() {
    let mut user1 = User {
        username: String::from("rustacean"),
        email: String::from("crab@example.com"),
        active: true,
        sign_in_count: 1,
    };
    user1.sign_in_count += 1; // 整个实例是 mut 才能改字段
    println!("{:?}", user1);

    // 结构体更新语法：其余字段从 user1 拿
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1.clone()
    };
    println!("{:#?}", user2);
    println!("user1 == user2 ? {}", user1 == user2);

    let black = Color(0, 0, 0);
    println!("黑色: {:?}, RGB = {} {} {}", black, black.0, black.1, black.2);

    let mut rect = Rectangle::new(30, 50);
    let sq = Rectangle::square(10);
    println!("rect 面积 = {}", rect.area());
    println!("rect 能装下 sq 吗？{}", rect.can_hold(&sq));
    rect.scale(2);
    println!("放大后: {:?}, 面积 = {}", rect, rect.area());
}

// 🏋️ 练习：
// 1. 给 Rectangle 加一个方法 perimeter(&self) -> u32 计算周长。
// 2. 定义 struct Point { x: f64, y: f64 }，实现 distance(&self, other: &Point) -> f64。
//    提示：f64 有 .sqrt() 和 .powi(2) 方法。
