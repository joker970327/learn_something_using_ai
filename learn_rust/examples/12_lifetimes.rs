//! 第 12 课：生命周期 (Lifetimes)
//!
//! 运行：cargo run --example 12_lifetimes
//!
//! 生命周期注解 'a 不会改变引用活多久，
//! 它只是告诉编译器「多个引用的存活时间之间是什么关系」，
//! 好让借用检查器确认不会出现悬垂引用。
//!
//! 大部分时候编译器能自动推断（生命周期省略规则），只有模糊时才需要手写。

// 返回值的引用来自 x 或 y，编译器无法自己判断 → 需要标注：
// 「返回值的存活时间不超过 x 和 y 中较短的那个」
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

// 结构体持有引用时，也必须标注生命周期：
// Excerpt 实例不能比它引用的文本活得更久
#[derive(Debug)]
struct Excerpt<'a> {
    part: &'a str,
}

impl<'a> Excerpt<'a> {
    fn announce(&self, msg: &str) -> &'a str {
        println!("注意: {msg}");
        self.part
    }
}

// 省略规则：只有一个输入引用时，输出自动获得同样的生命周期，不用写
fn first_char(s: &str) -> &str {
    &s[..s.chars().next().map_or(0, |c| c.len_utf8())]
}

fn main() {
    let s1 = String::from("长长的字符串");
    {
        let s2 = String::from("短");
        let result = longest(&s1, &s2);
        println!("更长的是: {result}"); // result 在 s2 有效期内使用 → OK
    }

    // 下面这样就会报错：
    // let result;
    // {
    //     let s2 = String::from("短");
    //     result = longest(&s1, &s2); // ❌ s2 活得不够久
    // }
    // println!("{result}");

    let novel = String::from("很久很久以前。有一只螃蟹……");
    let first_sentence = novel.split('。').next().unwrap();
    let ex = Excerpt { part: first_sentence };
    println!("{:?}", ex);
    println!("{}", ex.announce("摘录如下"));

    println!("首字符: {}", first_char("Rust"));

    // 'static：在整个程序运行期间都有效，例如字符串字面量
    let s: &'static str = "我永远有效";
    println!("{s}");
}

// 🏋️ 练习：
// 1. 取消 ❌ 那段代码的注释，阅读报错，理解为什么不行。
// 2. 写 fn longest_word<'a>(text: &'a str) -> &'a str，返回文本中最长的单词。
