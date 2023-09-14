# String

- String literals: `&str`
- `String`

## string literals

- 不可變的
- 字串的數值是寫死在程式內的 => 編譯時期就知道內容

有些時候，不能使用 string literals: 

1. 因為 string literals 是不可變的，所以它不可能完全適用於我們使用 string 時的所有狀況
2. 並非所有字串值在我們編寫程式時就會知道。Eg. User's input

## `String`

- 管理分配在 Heap 上的資料 => 可以儲存我們在編譯期間未知的一些文字
- 支援可變性
- 一個指向儲存字串內容記憶體的 pointer、它的 length 和它的 capacity是儲存在 Stack 上，但內容則是儲存在 Heap 上

Eg.

```rust
let mut s = String::from("hello"); // String::from 的實作會請求分配一塊它需要的記憶體

    s.push_str(", world!"); // push_str() 將字面值加到字串後面

    println!("{}", s); // 這會印出 `hello, world!`
```