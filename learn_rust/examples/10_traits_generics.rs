//! 第 10 课：泛型 (Generics) 与特征 (Trait)
//!
//! 运行：cargo run --example 10_traits_generics
//!
//! trait 类似其他语言的「接口」：定义一组行为，由类型去实现。

use std::fmt::Display;

// ---------- 定义 trait ----------
trait Summary {
    fn author(&self) -> String;

    // 可以有默认实现
    fn summarize(&self) -> String {
        format!("(来自 {} 的更多内容...)", self.author())
    }
}

struct Article {
    title: String,
    author: String,
}

struct Tweet {
    username: String,
    content: String,
}

impl Summary for Article {
    fn author(&self) -> String {
        self.author.clone()
    }
    fn summarize(&self) -> String {
        format!("《{}》 作者：{}", self.title, self.author)
    }
}

impl Summary for Tweet {
    fn author(&self) -> String {
        format!("@{}", self.username)
    }
    // 不重写 summarize，使用默认实现
}

// ---------- trait 作为参数：三种等价写法 ----------
fn notify(item: &impl Summary) {
    println!("快讯！{}", item.summarize());
}

fn notify_generic<T: Summary>(item: &T) {
    println!("泛型版：{}", item.summarize());
}

fn notify_where<T>(item: &T)
where
    T: Summary,
{
    println!("where 版：{}", item.summarize());
}

// ---------- 泛型函数 + trait 约束 ----------
// PartialOrd 表示可以比较大小，Copy 表示可以复制
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// ---------- 泛型结构体 ----------
#[derive(Debug)]
struct Pair<T> {
    a: T,
    b: T,
}

impl<T: PartialOrd + Display> Pair<T> {
    fn show_bigger(&self) {
        if self.a >= self.b {
            println!("较大的是 a = {}", self.a);
        } else {
            println!("较大的是 b = {}", self.b);
        }
    }
}

// ---------- 为自己的类型实现标准库 trait ----------
impl Display for Tweet {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "@{}: {}", self.username, self.content)
    }
}

fn main() {
    let article = Article {
        title: "Rust 入门".to_string(),
        author: "张三".to_string(),
    };
    let tweet = Tweet {
        username: "ferris".to_string(),
        content: "我是一只螃蟹 🦀".to_string(),
    };

    notify(&article);
    notify(&tweet);
    notify_generic(&article);
    notify_where(&tweet);

    println!("Display: {tweet}");

    println!("最大整数: {}", largest(&[34, 50, 25, 100, 65]));
    println!("最大字符: {}", largest(&['y', 'm', 'a', 'q']));

    let p = Pair { a: 3.5, b: 7.2 };
    p.show_bigger();

    // ---------- trait 对象：dyn Trait，实现运行时多态 ----------
    // 不同类型放在同一个 Vec 里，只要它们都实现了 Summary
    let items: Vec<Box<dyn Summary>> = vec![Box::new(article), Box::new(tweet)];
    for item in &items {
        println!("动态分发 → {}", item.summarize());
    }
}

// 🏋️ 练习：
// 1. 定义 trait Animal { fn name(&self) -> String; fn speak(&self) -> String; }，
//    为 Dog 和 Cat 实现，并放进 Vec<Box<dyn Animal>> 里循环调用。
// 2. 为 Pair<T> 实现一个 swap(self) -> Pair<T> 方法。
