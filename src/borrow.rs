#[cfg(test)]
use crate::drop::CustomDrop;

/// Borrowing allows accessing a value without moving it.
/// This is useful when you want to use the same value multiple times in different scopes.
/// In this test we create a variable `alice` and borrow it twice before finishing.
/// The output of this test is
/// ```text
/// Borrowed CustomDrop { name: "Alice" }
/// Borrowed CustomDrop { name: "Alice" }
/// End of scope
/// Dropping Alice
/// ```
/// Notice `alice` is dropped at the end of the text, not in the function scope
/// because borrowing does not take ownership of the variable.
#[test]
fn test_borrow() {
    fn borrow_to_print<T: std::fmt::Debug>(value: &T) {
        println!("Borrowed {value:?}");
    }

    let alice = CustomDrop { name: "Alice" };
    borrow_to_print(&alice);
    borrow_to_print(&alice);
    println!("End of scope");
}

/// In this test `alice` is borrowed by another data type that lives in a more limited scope.
/// The variable `alice` is not dropped until the end of the test because borrowing
/// does not change ownership.
#[test]
fn test_data_type_borrow() {
    let alice = CustomDrop { name: "Alice" };
    let _x: u32 = {
        let _borrowed1 = Borrower { value: &alice };
        let _borrowed2 = Borrower { value: &alice };
        42
    };
    println!("End of scope");
}

/// Mutable references are distinct from immutable ones.
/// Any number of immutable references are allowed at the same time,
/// while only one mutable is allowed at the same time. It is also
/// forbidden to have mutable and immutable references at the same time.
/// This prevents data races (multiple threads modifying data at the same time).
/// In this test we define a function to mutate a value by reference.
/// The output from this test is
/// ```text
/// End of scope
/// Dropping Bob
/// ```
#[test]
fn test_mut_borrow() {
    fn transmute_to_bob(value: &mut CustomDrop) {
        value.name = "Bob";
    }

    let mut alice = CustomDrop { name: "Alice" };
    transmute_to_bob(&mut alice);
    println!("End of scope");
}

/// Data types can hold borrowed values (references) too.
/// If they do then they must include a "lifetime" parameter,
/// which is how the compiler keeps track of when values are dropped
/// and prevents invalid memory access (seg faults).
/// For example, the following code would fail to compile
/// ```ignore
/// fn test_value_does_not_live_long_enough() {
///     let _borrow = {
///         let alice = crate::drop::CustomDrop { name: "Alice" };
///         Borrower { value: &alice }
///     };
/// }
/// ```
pub struct Borrower<'a, T> {
    pub value: &'a T,
}
