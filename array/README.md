# Array

```rust
let arr_1 = [1,2,3,4];
arr_1.len();
ley arr_2 = [1,2,3,4];

// loop
let mut loop_idx = 0;
loop {
    if arr_2[loop_idx] % 2 == 0 {
      loop_idx += 1;
      continue;
    }
    if arr_2[loop_idx] == 9 {
        break;
    }
    loop_idx +=1;
}
while loop_idx < arr_2.len() {
  loop_idx +=1;
}
for val in arr_2.iter() {

}
```