# Struct

- [Lifetime Annotations in Struct Definitions](../generics/README.md#lifetime-annotations-in-struct-definitions)

```rust
// basic usage
struct User {
    username: String,
    email: String,  
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let mut user1 = User {
          email: String::from("test@mail.com"),
          username: String::from("test"),
          active: true,
          sign_in_count: 1
    }
    let name = user1.username;
    user1.username = String::from("test2");
    let user2 = build_user(
        String::from("user2@e.com"),
        String::from("user2");
      );

    let user3 = User {
        email: String::from("user3@e.com"),
        username: String::from("user3"),
        ..user2
    }
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
```

```rust
fn main() {
    // tuple struct
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
}
```

```rust
// method
struct Rectangle {
    wid: u32,
    hei: u32
}

// impl will house the functions and methods associated with the strcut
impl Rectangle {
    // reference, immutable reference and ownership are all possible
    fn area(&self) -> u32 {
        self.wid * self.hei
    }

    fn hold(&self, other: &Rectangle) -> bool {
        self.wid > other.wid && self.hei > other.hei
    }
}

// associated function
impl Rectangle {
    // no self
    fn square(size: u32) -> Rectangle {
        Rectangle {
            wid: size,
            hei: size
        }
    }
}

fn main() {
    rect.area();
    let rect2 = Rectangle::sqare(25);
}
```
