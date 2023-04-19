# Generics

- 如果你發現你的程式碼需要使用大量泛型的話，這通常代表你的程式碼需要重新組織成更小的元件
- 當你的程式碼有許多 `struct` 或 `enum` 都「只有」儲存的值不同時，你可以使用 generic type 來避免重複

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