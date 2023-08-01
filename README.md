# Auto Getters

A simple macro for generating getters for rust struct members.

## **Usage**

```rust
#[derive(AutoGet)]
struct Something {
    test: String,

    #[exclude]
    test2: String,

    #[no_mut]
    test3: String,
}
```
