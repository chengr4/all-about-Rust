# Memory

- Passing in references as function parameters is called **borrowing**

Rule of References:

- At any given time, you can only have one mutable ref and multiple immutable refs
- References must be valid
- mutable and immutable cannot be mixed together

## Ownership

Three Ownership rules:

1. Each value in Rust has a variable that is called its owner (one variable one owner)
2. There can only be one owner at a time
3. When the owener goes out of scope, the value will be dropped
  ```rust
  { // s is not declared yet
    let s = "hello"; // s is valid
    // do stuff with s
  } // this scope is over, and s is no longer valid
  ```

  > after the scope, rust de-allocates the memory automatically (vs `c` and `c++`)

```rust
fn main() {
  let x = 5;
  let y = x; //copy

  let s1 = String::from("hello");
  let s2 = s1; // move (not shallow copy)

  printLn!("{}", s1); // ! get error, because value has been moved to s2

  // to clone
  let s3 = s1.clone();
}
```

---

## References

- [Bogdan;Understanding Ownership in Rust (2021.2)](https://youtu.be/VFIOSWy93H0)