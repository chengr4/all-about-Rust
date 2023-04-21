# Traits: Defining Shared Behavior

Different `types` share the same behavior if we can call the same methods on all of those `types`. Trait definitions are a way to group method signatures together to define a set of behaviors necessary to accomplish some purpose.

## Default Implementations

```rust
pub trait Summary {
    // default behavior
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }

    // instead of
    fn summarize(&self) -> String;
}
```

Default implementations can call other methods in the same trait, even if those other methods don’t have a default implementation. Eg.

```rust
pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}
```