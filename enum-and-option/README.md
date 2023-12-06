# Enum and Option

## Control Flow `match`

```rust
let age = 8;
match age2 {
  // range 1 to 18
  1..=18 => printIn!(),
  21 | 50 => printIn!(),
  62..=i32::MAX => printIn!(),
  // default case
  _ => printIn!(),
};

use std::cmp::Ordering

let my_age = 18;
let voting_age = 18;
match my_age.cmp(&voting_age){
  Ordering::Less => printIn!(),
  Ordering::Greater => printIn!(),
  Ordering::Equal => printIn!(),
};
```