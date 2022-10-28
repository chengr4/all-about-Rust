# Variable

## Number

> unsigned: > 0

- `usize`, `isize`: size depending on the computer (eg 64 bits)
- Get max `u32`: `u32::MAX`

## Tuple

```rust
    let my_tuple: (u8, String, f64) = (47, "Derek".to_string(), 50_000.00);
    let (v1, v2, v3) = my_tuple;
```

## str

- string slice
- Usually seen in its **borrowed** form, `&str`
- It is also the type of string literals, `&'static str` (?)
- String slices are always valid `UTF-8`

## String

```rust
    let mut str1 = String::new();
    str1.push('A');
    str1.push_str(" word");
    for word in str1.split_whitespace() {

    }
    let str2 = str1.replace("A", "Another");
```

## Vector

```rust
let str3 = String::from("x r t b h k k ");
let mut v1: Vec<char> = st3.chars().collect();
v1.sort();
// remove duplicates
v1.dedup();
for char in v1 {

}
let str4: &str = "Random string";
let mut str5: Sring = str4.to_string();
ley byye_arr1 = st5.as_bytes();
let str6 = &st5[0..6];
str6.len(); // 6
str5.clear();
let str7 = String::from("apple");
let str8 = String::from("banana");
// str7 will be deleted
let str9 = str7 + &str8

```
