# Enum and Option

## Control Flow `match`

```rust
let age = 8;
match age2 {
  // range 1 to 18
  1..=18 => printIn!(),
  21 | 50 => printIn!(),
  62..=i32::MAX => printIn!(),
  // default case
  _ => printIn!(),
};

use std::cmp::Ordering

let my_age = 18;
let voting_age = 18;
match my_age.cmp(&voting_age){
  Ordering::Less => printIn!(),
  Ordering::Greater => printIn!(),
  Ordering::Equal => printIn!(),
};
```

## Methods of `Option<T>`

`taken()`: Returns `Some` if the `Option` is `Some`, otherwise returns `None` and consumes the original `Option`.

`taken()` 的使用目的:

- 避免副本（Avoiding Cloning）： 如果 T 實現了 Copy 特性，使用 take 可以避免對值進行克隆，而是取得原始值的拷貝。這樣可以節省克隆的開銷。
- 所有權轉移（Ownership Transfer）： 如果 T 只實現了 Clone 特性，take 可以將值的所有權轉移到新的 Option 中，同時將原始的 Option 設為 None。這樣可以在需要的情況下進行所有權的轉移，而不是創建額外的克隆。

`as_mut()`: Returns a mutable reference to the contained value if the `Option` is `Some`, or `None` if it is `None`.

> 也是 Result 的 method

```rust
let mut optional_value: Option<i32> = Some(42);

if let Some(value) = optional_value.as_mut() {
    // 改變 optional_value 內部的值
    *value += 1;
}

println!("{:?}", optional_value); // Some(43)
```