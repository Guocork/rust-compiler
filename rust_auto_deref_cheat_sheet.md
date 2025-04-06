# 🦀 Rust 自动解引用 & 引用强制转换 Cheat Sheet

这份速查表总结了 Rust 中常见的自动 `Deref` 和引用强制转换（coercion）行为，方便开发中快速理解和定位行为差异。

---

## ✅ 方法调用：会自动解引用

```rust
use std::ops::Deref;

struct A;
impl A {
    fn hello(&self) {
        println!("hello from A");
    }
}
struct B(A);
impl Deref for B {
    type Target = A;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
fn example_method_call() {
    let b = B(A);
    b.hello(); // ✅ 自动解引用 B → A
}
```

---

## ✅ 函数参数引用：会自动强制类型转换

```rust
fn greet(name: &str) {
    println!("Hello, {name}!");
}
fn example_coercion() {
    let s = String::from("Alice");
    greet(&s); // ✅ &String 自动变成 &str
}
```

---

## ❌ 函数参数为值：不会自动解引用为值

```rust
struct Person {
    name: String,
}
struct Wrapper(Person);
impl std::ops::Deref for Wrapper {
    type Target = Person;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
fn say(p: Person) {
    println!("Hey, {}!", p.name);
}
fn example_pass_value() {
    let w = Wrapper(Person { name: "Bob".into() });
    // say(w); // ❌ 不行：不会自动 * 解引用为值
    say((*w).clone()); // ✅ 手动解引用 + clone
}
```

---

## ❌ 字段访问：不会自动解引用

```rust
fn example_field_access() {
    let w = Wrapper(Person { name: "Carol".into() });
    // println!("{}", w.name); // ❌ 不行
    println!("{}", w.deref().name); // ✅ 手动解引用
}
```

---

## ✅ 匹配结构（`match` / `if let`）：会自动解引用

```rust
enum MyOption {
    Some(i32),
    None,
}
struct WrapOpt(MyOption);
impl std::ops::Deref for WrapOpt {
    type Target = MyOption;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
fn example_pattern_match() {
    let w = WrapOpt(MyOption::Some(42));
    match w {
        MyOption::Some(v) => println!("Matched {}", v),
        MyOption::None => println!("No value"),
    }
}
```

---

## 📌 快速记忆口诀

```
方法调用 自动解，
引用参数 强制转。
字段访问 手动写，
传值不帮 自 deref。
```
