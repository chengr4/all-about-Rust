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

### Why We Need?

When we’re defining this function, we don’t know the concrete values that will be passed into this function. We also don’t know the concrete lifetimes of the references that will be passed in.