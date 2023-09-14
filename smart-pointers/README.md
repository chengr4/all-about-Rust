# Smart Pointers

## `Deref` trait

### Implicit Deref Coercions with Functions and Methods

- Deref coercion can convert `&String` to `&str` because String implements the `Deref` trait such that it returns `&str`

    ```rust
    #[stable(feature = "rust1", since = "1.0.0")]
    impl ops::Deref for String {
        type Target = str;

        #[inline]
        fn deref(&self) -> &str {
            unsafe { str::from_utf8_unchecked(&self.vec) }
        }
    }
    ```

> Deref coercion converts a reference to a type that implements the Deref trait into a reference to another type (working only on types that implement the Deref)

When will happen?

It happens automatically when we pass a reference to a particular type’s value as an argument to a function or method that doesn’t match the parameter type in the function or method definition.

- Rust will also coerce a mutable reference to an immutable one. But the reverse is not possible

## `Drop` trait


## References

- https://doc.rust-lang.org/std/string/struct.String.html#impl-Deref-for-String
