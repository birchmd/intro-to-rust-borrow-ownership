/// Rust uses lexical scopes to manage memory.
/// When variable goes out of scope it is automatically "dropped".
/// When an object is dropped, any memory associated with it is automatically freed.
/// In this test `_alice` is declared at the beginning of the scope,
/// `_bob` is declared in a sub-scope, and there is an additional variable `x`.
/// By Rust's dropping rules we see the following output when running this test:
/// ```text
/// Dropping Bob
/// 42
/// Dropping Alice
/// ```
#[test]
fn test_drop() {
    let _alice = CustomDrop { name: "Alice" };
    let x: u32 = {
        let _bob = CustomDrop { name: "Bob" };
        42
    };
    println!("{x}");
}

/// This struct has a custom implementation of the `Drop` trait.
/// It allows us to see when it is dropped.
#[derive(Debug)]
pub struct CustomDrop {
    pub name: &'static str,
}

/// The `Drop` trait includes a single method `drop` which is called
/// when a variable goes out of scope.
/// This can be used to implement Resource Allocation is Initialization (RAII) patterns.
impl Drop for CustomDrop {
    fn drop(&mut self) {
        println!("Dropping {}", self.name);
    }
}
