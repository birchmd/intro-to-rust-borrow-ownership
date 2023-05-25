#[cfg(test)]
use crate::drop::CustomDrop;

/// Data types can take ownership of other values.
/// This means the value will not be dropped until its owner goes out of scope.
/// In this example `alice` is defined in a sub-scope, so would be dropped,
/// however ownership of the variable is given to another data type (`Owner`)
/// which persists for a larger scope and hence `alice` also persists longer.
/// When the owner of a variable changes we say it is "moved".
/// The output from this test is
/// ```text
/// End of scope
/// Dropping owner container
/// Dropping Alice
/// ```
#[test]
fn test_move() {
    let _container = {
        let alice = CustomDrop { name: "Alice" };
        Owner { inner: alice }
    };
    println!("End of scope");
}

/// Functions can also take ownership of values.
/// In this test we create a variable `alice` then pass it to a function.
/// The output of this test is
/// ```text
/// Taken ownership of CustomDrop { name: "Alice" }
/// Dropping Alice
/// End of scope
/// ```
/// Notice that `alice` is dropped when the function scope ends, not when the test scope ends.
#[test]
fn test_function_move() {
    fn move_var<T: std::fmt::Debug>(value: T) {
        println!("Taken ownership of {value:?}");
    }

    let alice = CustomDrop { name: "Alice" };
    move_var(alice);
    println!("End of scope");
}

/// This struct takes ownership of any type `T`, storing the value in its `inner` field.
pub struct Owner<T> {
    pub inner: T,
}

/// Custom `Drop` impl so we can see when it leaves scope.
impl<T> Drop for Owner<T> {
    fn drop(&mut self) {
        println!("Dropping owner container");
    }
}
