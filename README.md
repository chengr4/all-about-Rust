# All about Rust

- Core: traits and safe access to data
- Package manager: cargo
- Common indent: 4 spaces
- Print line is called "marco" (see by exclamation)
- Shadowing: Define a variable with same name but different data types 

---

- [String](./string/)
- [Loop](./loop/)
- [Variable](./variable/)
- [Array](./array/)
- [Memory and OwnerShip](./memory-and-ownership/)
- [Struct](./struct/)
- [Closure](./closure/)
- [Enum and Option](./enum-and-option/)
- [Generics](./generics/)
- [Traits](./trait/)
- [Lifretimes](./lifetimes/)
- [Q/A](./qa/)

Compile code:

```sh
rustc <file_name>
```

## Cargo

Create project:

```sh
cargo new <folder_name>
or
cargo init
```

Compile code:

```sh
cargo r
cargo run
cargo build
cargo build --release
```

## Conditions

```rust
fn main() {
    let age: i32 = 8;
    if (age >= 1) && (age <= 18) {
      println!();
    } else if (age == 21) || (age == 21) {

    } else {

    }
}
```

```rust
let mut my_age = 47;
let can_vote = if my_age >= 18 {
  true
} else {

}
```

match:

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

## Researchable Resources

- https://github.com/tbillington/rust_serverless_runtime
- [fuzzing(?)](https://github.com/gamozolabs)
- https://github.com/cccriscv/mini-riscv-os
- [Mara Bos; Rust Atomics and Locks](https://marabos.nl/atomics/?fbclid=IwAR0MNn1M8Sty_Pqv2ONLpqX2lL-57d-QGTGGZwYEG0z2E5yO2DlY0_KGFdU)
- [https://doc.rust-lang.org/stable/book/](The Rust Programming Language)
- [https://rust-lang.tw/book-tw/title-page.html](Rust 程式設計語言)
