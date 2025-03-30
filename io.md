# I/O

## `BufRead` and `BufReader`

| Method | 讀取類型 | Return Type | auto remove `\n` | Scenario |
|--------|----------|-------------|------------------|----------|
| `read_line()` | 逐行 | `String` | X | Read a line from the buffer |
| `lines()` | 逐行 (迭代器) | `Result<String>` | O | Simple iterator over lines |
| `read_until()` | customized delimiter | `Vec<u8>` | X | Deal with byte or sepcific data |
| `fill_buf()` | Buffer block | &[u8] | X | 直接讀取緩衝資料，不自動消耗 |