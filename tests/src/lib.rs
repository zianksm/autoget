#[derive(autoget::AutoGet)]
struct Something {
    test: String,

    #[exclude]
    test2: String,

    #[no_mut]
    test3: String,
}
