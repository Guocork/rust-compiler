//! rust_auto_deref_cheat_sheet.rs
//! 📌 Rust 自动解引用 & 引用强制转换 Cheat Sheet

use std::ops::Deref;

pub struct A;
impl A {
    pub fn hello(&self) {
        println!("hello from A");
    }
}
pub struct B(pub A);
impl Deref for B {
    type Target = A;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
pub fn example_method_call() {
    let b = B(A);
    b.hello(); // ✅ 自动解引用 B → A
}

pub fn greet(name: &str) {
    println!("Hello, {name}!");
}
pub fn example_coercion() {
    let s = String::from("Alice");
    greet(&s); // ✅ &String 自动变成 &str
}

#[derive(Clone)]
pub struct Person {
    pub name: String,
}
pub struct Wrapper(pub Person);
impl Deref for Wrapper {
    type Target = Person;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
pub fn say(p: Person) {
    println!("Hey, {}!", p.name);
}
pub fn example_pass_value() {
    let w = Wrapper(Person { name: "Bob".into() });
    say((*w).clone()); // ✅ 手动解引用 + clone
}

pub fn example_field_access() {
    let w = Wrapper(Person { name: "Carol".into() });
    println!("{}", w.deref().name); // ✅ 手动解引用
}

pub enum MyOption {
    Some(i32),
    None,
}
pub struct WrapOpt(pub MyOption);
impl Deref for WrapOpt {
    type Target = MyOption;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
pub fn example_pattern_match() {
    let w = WrapOpt(MyOption::Some(42));
    match w {
        MyOption::Some(v) => println!("Matched {}", v),
        MyOption::None => println!("No value"),
    }
}



//! # Rust 自动解引用 & 引用强制转换 Cheat Sheet
//!
//! 本模块总结了 Rust 中关于 `Deref` 自动解引用以及引用强制转换（coercion）的主要内容，包含以下部分：
//!
//! - **方法调用自动解引用**：当调用方法时，Rust 会自动解引用至能调用该方法的目标类型。
//! - **函数参数引用强制转换**：当函数参数要求引用时，Rust 会自动将符合条件的引用进行转换（例如 `&String` 转换为 `&str`）。
//! - **字段访问注意事项**：直接访问字段时不会自动解引用，需要手动解引用。
//! - **匹配结构自动解引用**：在 `match` 和 `if let` 中，Rust 会自动解引用。
//!
//! ## 示例代码
//!
//! ### 方法调用自动解引用
//!
//! ```rust
//! use std::ops::Deref;
//!
//! struct A;
//! impl A {
//!     fn hello(&self) {
//!         println!("hello from A");
//!     }
//! }
//!
//! struct B(A);
//! impl Deref for B {
//!     type Target = A;
//!     fn deref(&self) -> &Self::Target {
//!         &self.0
//!     }
//! }
//!
//! fn example_method_call() {
//!     let b = B(A);
//!     b.hello(); // 自动解引用 B → A 调用 hello()
//! }
//! ```
//!
//! ### 引用参数强制转换
//!
//! ```rust
//! fn greet(name: &str) {
//!     println!("Hello, {name}!");
//! }
//!
//! fn example_coercion() {
//!     let s = String::from("Alice");
//!     greet(&s); // &String 自动转换为 &str
//! }
//! ```
//!
//! ### 函数参数为值（手动解引用）
//!
//! ```rust
//! struct Person {
//!     name: String,
//! }
//!
//! struct Wrapper(Person);
//! impl std::ops::Deref for Wrapper {
//!     type Target = Person;
//!     fn deref(&self) -> &Self::Target {
//!         &self.0
//!     }
//! }
//!
//! fn say(p: Person) {
//!     println!("Hey, {}!", p.name);
//! }
//!
//! fn example_pass_value() {
//!     let w = Wrapper(Person { name: "Bob".into() });
//!     // say(w); // 编译错误，不会自动解引用为值
//!     say((*w).clone()); // 正确：手动解引用并克隆
//! }
//! ```
//!
//! ### 字段访问（需要手动解引用）
//!
//! ```rust
//! fn example_field_access() {
//!     let w = Wrapper(Person { name: "Carol".into() });
//!     // println!("{}", w.name); // 编译错误
//!     println!("{}", w.deref().name); // 正确：手动解引用后访问字段
//! }
//! ```
//!
//! ### 匹配结构自动解引用
//!
//! ```rust
//! enum MyOption {
//!     Some(i32),
//!     None,
//! }
//!
//! struct WrapOpt(MyOption);
//! impl std::ops::Deref for WrapOpt {
//!     type Target = MyOption;
//!     fn deref(&self) -> &Self::Target {
//!         &self.0
//!     }
//! }
//!
//! fn example_pattern_match() {
//!     let w = WrapOpt(MyOption::Some(42));
//!     match w {
//!         MyOption::Some(v) => println!("Matched {}", v),
//!         MyOption::None => println!("No value"),
//!     }
//! }
//! ```
//!
//! ## 快速记忆口诀
//!
//! ```text
//! 方法调用 自动解，
//! 引用参数 强制转。
//! 字段访问 手动写，
//! 传值不帮 自 deref。
//! ```
