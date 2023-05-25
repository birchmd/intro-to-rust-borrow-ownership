/// Similar to the example in `copy.rs`, but uses `clone` instead.
/// Note that `Clonable` uses a `String` (heap-allocated type) instead of
/// `&str`, so `Clonable` cannot implement `Copy`.
#[test]
fn test_clone() {
    let alice = Clonable {
        name: "Alice".into(),
    };
    let _owner1 = crate::ownership::Owner {
        inner: alice.clone(),
    };
    let _owner2 = crate::ownership::Owner { inner: alice };
    println!("End of scope");
}

/// `Clone` is a prerequisite for `Copy` and will be discussed in more detail later.
#[derive(Debug, Clone)]
pub struct Clonable {
    pub name: String,
}
