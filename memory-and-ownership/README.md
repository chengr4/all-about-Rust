# Memory and Ownership

- 當變數離開 scope 時，Rust 會幫我們呼叫一個特殊函式。此函式叫做 `drop`
- Passing in references as function parameters is called **borrowing**
- The concepts of ownership, borrowing, and slices ensure memory safety in Rust programs at compile time.

## Ownership

- Ownership 主要就是為了管理 Heap

> Allocating on the heap: 當你要將資料放入堆積，你得要求一定大小的空間。Memory allocator 會找到一塊夠大的空位，標記為已佔用，然後回傳一個 pointer，指著該位置的位址

Three Ownership rules:

1. Each value in Rust has a variable called its owner (one variable one owner)
2. There can only be one owner at a time
3. When the owener goes out of scope, the value will be dropped
  ```rust
  {   // s is not declared yet
      let s = "hello"; // s is valid
      // do stuff with s
  }   // this scope is over, and s is no longer valid
  ```

  > after the scope, rust de-allocates the memory automatically (vs `c` and `c++`)



```rust
fn main() {
    let x = 5;
    let y = x; // copy value

    let s1 = String::from("hello");
    let s2 = s1; // move: copy value of s1 pointer to s2 + Rust 就不再將 s1 視爲有效 (prevent double free)

    printLn!("{}", s1); // ! get error, because value has been moved to s2

    // to deep copy
    let s3 = s1.clone();
}
```

### References

- The purpose of a reference is to use a value without transferring ownership.
- Rust's method call syntax automatically dereferences the reference when calling a method on it.

Rule of References:

- At any given time, you can have either one mutable reference or any number of immutable references.
- References must always be valid.

> Mutable references have one big restriction: if you have a mutable reference to a value, you can have no other references to that value. 

```rust
let mut s = String::from("hello");

let r1 = &s; // 沒問題
let r2 = &s; // 沒問題
println!("{} and {}", r1, r2);
// 變數 r1 和 r2 將不再使用
    
let r3 = &mut s; // 沒問題
println!("{}", r3);
```

Mutable references with function

```rust
fn main() {
    let mut s = String::from("hello");

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```

## The Slice type

- Slices let you reference a contiguous sequence of elements in a collection
- Type: `&str`
    > &str 是個不可變參考

```rust
let s = String::from("hello");

let len = s.len();

// take a slice of the entire string
let slice = &s[0..len];
let slice = &s[..];
```

---

## References

- [Bogdan;Understanding Ownership in Rust (2021.2)](https://youtu.be/VFIOSWy93H0)
