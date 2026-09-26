# 练习参考答案

> 先自己做！卡住超过 15 分钟再来看。答案不唯一，能通过测试就是好答案。

```rust
pub fn fib(n: u32) -> u64 {
    let (mut a, mut b) = (0u64, 1u64);
    for _ in 0..n {
        (a, b) = (b, a + b);
    }
    a
}

pub fn fizzbuzz(n: u32) -> String {
    match (n % 3, n % 5) {
        (0, 0) => "FizzBuzz".to_string(),
        (0, _) => "Fizz".to_string(),
        (_, 0) => "Buzz".to_string(),
        _ => n.to_string(),
    }
}

pub fn last_word(s: &str) -> &str {
    s.split(' ').last().unwrap_or("")
}

impl Rect {
    pub fn area(&self) -> u32 { self.w * self.h }
    pub fn perimeter(&self) -> u32 { 2 * (self.w + self.h) }
}

pub fn first_even(v: &[i32]) -> Option<i32> {
    v.iter().copied().find(|x| x % 2 == 0)
}

pub fn shape_area(shape: &Shape) -> f64 {
    match shape {
        Shape::Circle(r) => std::f64::consts::PI * r * r,
        Shape::Rect(w, h) => w * h,
        Shape::Square(a) => a * a,
    }
}

pub fn word_count(text: &str) -> HashMap<String, usize> {
    let mut map = HashMap::new();
    for w in text.split_whitespace() {
        *map.entry(w.to_string()).or_insert(0) += 1;
    }
    map
}

pub fn sum_str(a: &str, b: &str) -> Result<i32, std::num::ParseIntError> {
    Ok(a.parse::<i32>()? + b.parse::<i32>()?)
}

pub fn largest<T: PartialOrd + Copy>(list: &[T]) -> Option<T> {
    let mut iter = list.iter().copied();
    let mut max = iter.next()?;
    for x in iter {
        if x > max { max = x; }
    }
    Some(max)
}

pub fn sum_multiples(n: u32) -> u32 {
    (1..=n).filter(|x| x % 3 == 0 || x % 5 == 0).sum()
}

pub fn longest_word(text: &str) -> &str {
    // max_by_key 在相等时返回「最后一个」，所以这里用 fold 保证返回先出现的
    text.split_whitespace()
        .fold("", |best, w| if w.len() > best.len() { w } else { best })
}
```
