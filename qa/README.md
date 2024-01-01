# Q/A

## Q: Are `&s` and `s.as_str()` the same?

A: In telegram group (2023.12),

[@Kan-Ru Chen](https://github.com/kanru):

- `&s` is from `Deref`, `s.as_str()` is `String`'s method.
- Method 可以用在參數需要 `Fn` 的時候，例如 `Iterator::map` 就可以用 `x.iter().map(String::as_str)`，或是可以 chain `x.to_string().as_str().xxx`

@popdailydose: Type of `&s` is `&String`, type of `s.as_str()` is `&str`. `&String` can be converted to `&str` by `Deref` coercion.

@bdbai: `&String` 可以用 `.capacity()` ，但 `&str` 不行，其他都一樣

@zwindr:

```rust
let s = String::from("123");
    
// error[E0308]: mismatched types
match &s {
    "123" => println!("123"),
    _ => println!("other"),
}
    
// OK
match s.as_str() {
    "123" => println!("123"),
    _ => println!("other"),
}
```

## Q: Why is `&s.repeat(3);` equal to `&(s.repeat(3));` rather than `(&s).repeat(3);`?

A: The `.` operator has a higher precedence than `&` operator. So `&s.repeat(3)` is equal to `&(s.repeat(3))`.

## Q: What is exclamation mark (!) for?

A: The exclamation mark is part of the syntax for Rust's macro system. Macros are a powerful feature that allow developers to define custom code transformations(?) that can be applied to source code before it is compiled.

`println!` is a Rust macro that prints formatted text to the console. The exclamation mark after `println` indicates that it is a **macro**, not a regular function.

## Q: What is question mark (?) for?

A: `?` is an operator used for error handling. When a function returns a `Result` or `Option` type, the question mark can be used to propagate errors (誤差傳播) up the call stack without the need for explicit error handling.

- If the value is `Ok`, the expression returns the value inside the `Ok` variant. 
- If the value is `Err`, the `?` operator returns early from the function and passes the error up the call stack
