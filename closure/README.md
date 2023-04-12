# Closure

- 又稱匿名函式
- 在 Rust 中，`||` 符號是用來定義一個空的匿名函式，即一個無參數的 closure 的語法。這個符號可以理解為 pipeline (將左邊的輸入值傳遞給右邊的匿名函式作為參數)

```rust
// A function
fn  add_one_v1   (x: u32) -> u32 { x + 1 }
// Closures
let add_one_v2 = |x: u32| -> u32 { x + 1 };
let add_one_v3 = |x|             { x + 1 };
let add_one_v4 = |x|               x + 1  ;

let v3 = add_one_v3(3); // 4
```

## `move`

- 要強迫 closure 取得周圍環境數值的所有權的話，你可以在 parameter list (`||`)前使用 `move` 關鍵字
- 此技巧適用於將閉包傳給新 thread 來移動資料，讓新的 thread 能擁有該資料

```rust
use std::thread;

fn main() {
    let list = vec![1, 2, 3];
    println!("呼叫閉包前：{:?}", list);

    thread::spawn(move || println!("來自執行緒：{:?}", list))
        .join() // 會將創建的執行緒加入到主執行緒中，並等待該執行緒完成
        .unwrap(); // 處理可能產生的錯誤
}
```