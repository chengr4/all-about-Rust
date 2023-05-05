# Lifetimes

Lifetimes are another kind of generic

The main aim of lifetimes is to prevent `dangling references (迷途參考)`, which cause a program to reference data other than the data it’s intended to reference.

**Why we need?**

When we’re defining this function, we don’t know the concrete values that will be passed into this function. We also don’t know the concrete lifetimes of the references that will be passed in.

One lifetime annotation by itself doesn’t have much meaning, because the annotations are meant to tell Rust **how generic lifetime parameters of multiple references relate to each other.**

## Lifetime Annotation Syntax

- people usually use the name `'a` for the first lifetime annotation

```rust
&i32        // a reference
&'a i32     // a reference with an explicit lifetime
&'a mut i32 // a mutable reference with an explicit lifetime
```

## Lifetime Annotations in Function Signatures

```rust
// the signature expresses the constraint: the returned reference will be valid as long as both the parameters are valid. 
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

## Lifetime Annotations in Struct Definitions

- We can define structs to hold references, but in that case we would need to **add a lifetime annotation on every reference in the struct’s definition.**

```rust
// 此 annotation 代表 ImportantExcerpt 的實例不能比它持有的欄位 part 活得還久
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    // next(): 取出第一個「子」串
    // expect(): 返回一個錯誤信息
    let first_sentence = novel.split('.').next().expect("無法找到 '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}
```

## Lifetime Elision (生命週期省略)

Three rules by the compiler:

1. 編譯器會給予每個參考參數一個生命週期參數。換句話說，一個函式只有一個參數的話，就只會有一個生命週期：`fn foo<'a>(x: &'a i32)`
2. 如果剛好**只有**一個輸入生命週期參數，該參數就會賦值給**所有**輸出生命週期參數：`fn foo<'a>(x: &'a i32) -> &'a i32`
3. 如果有多個輸入生命週期參數，但其中一個是 `&self` 或 `&mut self`，由於這是 method，`self` 的生命週期會賦值給**所有**輸出生命週期參數。此規則讓 method 更容易讀寫，因為不用寫更多符號出來。

## The Static Lifetime