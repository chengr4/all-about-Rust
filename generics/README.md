# Generics

- 如果你發現你的程式碼需要使用大量泛型的話，這通常代表你的程式碼需要重新組織成更小的元件
- 當你的程式碼有許多 `struct` 或 `enum` 都「只有」儲存的值不同時，你可以使用 generic type 來避免重複
- Using generic types won't make your program run any slower than it would with concrete types
  > 藉由 monomorphization 能讓 generic 程式碼在編譯時填入實際的 type，因此不會降低效能

## In Struct Definitions

```rust
// multiple generic type parameters
struct Point<T, U> {
    x: T,
    y: U,
}

fn main() {
    let both_integer = Point { x: 5, y: 10 };
    let both_float = Point { x: 1.0, y: 4.0 };
    let integer_and_float = Point { x: 5, y: 4.0 };
}
```

## In Enum Definitions

```rust
enum Option<T> {
    Some(T),
    None,
}
```

## In Method Definitions

```rust
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn main() {
    let p = Point { x: 5, y: 10 };

    println!("p.x = {}", p.x());
}
```

## Lifetimes

Lifetimes are another kind of generic

The main aim of lifetimes is to prevent `dangling references (迷途參考)`, which cause a program to reference data other than the data it’s intended to reference.

**Why we need?**

When we’re defining this function, we don’t know the concrete values that will be passed into this function. We also don’t know the concrete lifetimes of the references that will be passed in.

One lifetime annotation by itself doesn’t have much meaning, because the annotations are meant to tell Rust **how generic lifetime parameters of multiple references relate to each other.**

### Lifetime Annotation Syntax

- people usually use the name `'a` for the first lifetime annotation

```rust
&i32        // a reference
&'a i32     // a reference with an explicit lifetime
&'a mut i32 // a mutable reference with an explicit lifetime
```

### Lifetime Annotations in Function Signatures

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

### Lifetime Annotations in Struct Definitions

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