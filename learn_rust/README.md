# 🦀 从零学 Rust

一套「边读边跑」的 Rust 入门教程：每一课都是一个可以直接运行的 `.rs` 文件，代码里用中文注释讲解概念，文末附带小练习。

## 0. 准备环境

```bash
# 安装 Rust（官方工具链管理器 rustup）
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

rustc --version   # 编译器
cargo --version   # 包管理 + 构建工具，日常几乎只用它
```

推荐编辑器：VS Code + **rust-analyzer** 插件（自动补全、类型提示、实时报错）。

常用 cargo 命令：

| 命令 | 作用 |
| --- | --- |
| `cargo new 项目名` | 新建项目 |
| `cargo run` | 编译并运行 |
| `cargo check` | 只检查能否编译（很快） |
| `cargo test` | 运行测试 |
| `cargo fmt` | 自动格式化代码 |
| `cargo clippy` | 代码风格/潜在问题检查 |

## 1. 课程路线

在 `learn_rust` 目录下运行：`cargo run --example <课程名>`

| # | 课程 | 运行命令 | 你会学到 |
| --- | --- | --- | --- |
| 1 | Hello | `cargo run --example 01_hello` | `main`、`println!`、格式化输出 |
| 2 | 变量与类型 | `cargo run --example 02_variables` | `let`/`mut`、遮蔽、整数/浮点/元组/数组 |
| 3 | 函数与控制流 | `cargo run --example 03_functions_control` | 表达式 vs 语句、`if`/`loop`/`while`/`for` |
| 4 | **所有权** ⭐ | `cargo run --example 04_ownership` | move、clone、Copy、作用域与 drop |
| 5 | **借用** ⭐ | `cargo run --example 05_borrowing` | `&`/`&mut`、借用规则、切片、`String` vs `&str` |
| 6 | 结构体 | `cargo run --example 06_structs` | `struct`、`impl`、方法、`derive` |
| 7 | 枚举与匹配 | `cargo run --example 07_enums_match` | `enum`、`Option`、`match`、`if let` |
| 8 | 集合 | `cargo run --example 08_collections` | `Vec`、`String`、`HashMap` |
| 9 | 错误处理 | `cargo run --example 09_error_handling` | `Result`、`?`、自定义错误 |
| 10 | 泛型与 Trait | `cargo run --example 10_traits_generics` | trait、泛型约束、`dyn Trait` |
| 11 | 闭包与迭代器 | `cargo run --example 11_closures_iterators` | 闭包、`map`/`filter`/`collect` 等 |
| 12 | 生命周期 | `cargo run --example 12_lifetimes` | `'a` 注解、结构体中的引用 |

⭐ 第 4、5 课是 Rust 与其他语言最不一样的地方，值得多读几遍。

## 2. 学习方法

1. **先读，再跑，再改**：打开 `examples/xx.rs` 读注释 → 运行看输出 → 随便改改再运行。
2. **故意写错**：代码里标着 ❌ 的注释行，取消注释运行一下。Rust 编译器的报错非常详细，读懂报错是学 Rust 最重要的能力。
3. **做练习**：每课结尾都有 🏋️ 练习，可以直接写在该文件里。

## 3. 闯关练习（测试驱动）

`src/exercises.rs` 里有 11 道题，每道题里有一个 `todo!()`，换成你的实现就行：

```bash
cargo test            # 一开始会有 11 个失败
cargo test ex01       # 只跑第 1 题
```

目标：让所有测试变绿 ✅。卡住了可以看 [SOLUTIONS.md](./SOLUTIONS.md)。

## 4. 下一步

- 📖 [The Rust Programming Language（中文版）](https://kaisery.github.io/trpl-zh-cn/)：官方教程，本教程的内容大体按它的顺序编排
- 🧩 [Rustlings](https://github.com/rust-lang/rustlings)：官方小练习集
- 🔍 [Rust by Example（中文）](https://rustwiki.org/zh-CN/rust-by-example/)：用例子学
- 🚀 进阶主题：模块与 crate、智能指针（`Box`/`Rc`/`RefCell`）、并发（线程、`Arc`/`Mutex`）、async
- 🛠️ 小项目：命令行猜数字、`grep` 简易版、TODO 清单 CLI
