# autoget

A simple macro for generating getters for rust struct members.

## Usage

```rust
#[derive(AutoGet)]
struct Something {
    test: String,
    test2: String,
    test3: String,
}
```

this will generate code that looks something like this:

```rust
impl Something {
    pub fn test(&self) -> &String {
        &self.test
    }
    pub fn test_mut(&mut self) -> &mut String {
        &mut self.test
    }
    pub fn test2(&self) -> &String {
        &self.test2
    }
    pub fn test2_mut(&mut self) -> &mut String {
        &mut self.test2
    }
    pub fn test3(&self) -> &String {
        &self.test3
    }
    pub fn test3_mut(&mut self) -> &mut String {
        &mut self.test3
    }
}
```

to disable mutable getters you can use `#[no_mut]` macro helper attributes on selected member structs.

```rust
#[derive(AutoGet)]
struct Something {
    test: String,
    #[no_mut]
    test2: String,
    test3: String,
}
```

or you can disable getters altogether by using `#[exclude]`

```rust
#[derive(AutoGet)]
struct Something {
    test: String,
    #[exclude]
    test2: String,
    test3: String,
}
```

you can use them alongside eachother such as:

```rust
#[derive(autoget::AutoGet)]
struct Something {
    test: String,

    #[exclude]
    test2: String,

    #[no_mut]
    test3: String,
}
```

## License

autoget is available under the MIT license. See the LICENSE file for more info.
