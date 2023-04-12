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

## Fn Traits

Closure 會依照 closure 本體從周圍環境獲取處理數值的方式，自動實作一種或多種 Fn trait：

1. `FnOnce` 適用於可以呼叫一次的閉包。**所有閉包至少都會有此特徵**，因為所有閉包都能被呼叫。會將獲取的數值移出本體的閉包只會實作 `FnOnce` 而不會再實作其他 `Fn` 特徵，因為這樣它只能被呼叫一次。
2. `FnMut` 適用於不會將獲取數值移出本體，而且可能會改變獲取數值的閉包。這種閉包**可以被呼叫多次**。
3. `Fn` 適用於不會將獲取數值移出本體，而且不會改變獲取數值或是甚至不從環境獲取數值的閉包。這種閉包可以被呼叫多次，而且不會改變周圍環境，這**對於並行呼叫閉包多次來說非常重要**。

> Function 也可以實作這三種 Fn 特徵