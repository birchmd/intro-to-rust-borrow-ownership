//! A variable must have a single owner (this prevents invalid memory access).
//! Therefore when a variable is moved then it cannot be moved again.
//! For example the following code would fail to compile
//! ```ignore
//! fn test_double_move() {
//!     let alice = crate::drop::CustomDrop { name: "Alice" };
//!     let _owner1 = crate::ownership::Owner {inner: alice};
//!     let _owner2 = crate::ownership::Owner {inner: alice};
//! }
//! ```
//! ```text
//! error[E0382]: use of moved value: `alice`
//!   --> src/ownership.rs:51:33
//!    |
//! 49 |     let alice = crate::drop::CustomDrop { name: "Alice" };
//!    |         ----- move occurs because `alice` has type `CustomDrop`, which does not implement the `Copy` trait
//! 50 |     let _owner1 = Owner {inner: alice};
//!    |                                 ----- value moved here
//! 51 |     let _owner2 = Owner {inner: alice};
//!    |                                 ^^^^^ value used here after move
//!
//! For more information about this error, try `rustc --explain E0382`.
//!```
//! Notice the error message mentions the `Copy` trait.
//! The `Copy` trait is a marker trait that tells the Rust compiler a value is cheap to
//! make a copy. This means the Rust compiler will automatically make a copy if the value
//! is moved multiple times. For example, basic types like `u32` implement Copy.

/// This test illustrates copying a value.
/// The variable `alice` is defined at the beginning of the scope, it is copied to
/// `_owner1` and moved to `_owner2`.
#[test]
fn test_copy() {
    let alice = Copyable { name: "Alice" };
    let _owner1 = crate::ownership::Owner { inner: alice };
    let _owner2 = crate::ownership::Owner { inner: alice };
    println!("End of scope");
}

/// `Clone` is a prerequisite for `Copy` and will be discussed in more detail later.
#[derive(Debug, Clone, Copy)]
pub struct Copyable {
    pub name: &'static str,
}
