# Loop

Sepcial loop pattern in Rust:

```rust
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            // add returned value for result
            break counter * 2;
        }
    };

    println!("結果為：{result}");
}
```

## Early Return

https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=2620d2b7d0c507f222a29b355d4658a3