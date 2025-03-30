# Trait Objects

A trait object points to

1. an instance of a type implementing our specified trait
2. a table used to look up trait methods on that type at runtime.

Example:

```rust
pub trait Draw {
    fn draw(&self);
}

type Component = Box<dyn Draw>;
```

## Attributes

- Trait objects must use a pointer, eg. `&` or `Box<T>`
- 當我們使用特徵物件時，Rust 必須使用動態調度
- 我們可以對泛型或實際型別使用特徵物件
- 泛型型別參數一次只能替換成一個實際型別，特徵物件則是在執行時允許數個實際型別能填入特徵物件中
- 特徵物件與傳統物件不同的地方在於，我們無法向特徵物件新增資料

## Purpose

One purpose of using trait object is to enable polymorphism in Rust

> Polymorphism: code that can work with data of multiple types.

## Trait Objects Perform Dynamic Dispatch

> Do not get it yet