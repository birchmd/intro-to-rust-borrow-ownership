/// This test illustrates the `Rc` type from the standard library.
/// This type allows "shared ownership" of a value.
/// The value is not dropped until all `Rc` instances are dropped.
/// The output from this test is:
/// ```text
/// Dropped one reference
/// Dropped second reference
/// Dropping Alice
/// Dropped final reference
/// ```
#[test]
fn test_reference_counted() {
    use std::rc::Rc;

    let alice = crate::drop::CustomDrop { name: "Alice" };
    let rc1 = Rc::new(alice);
    let rc2 = Rc::clone(&rc1);
    let rc3 = Rc::clone(&rc1);
    drop(rc1);
    println!("Dropped one reference");
    drop(rc2);
    println!("Dropped second reference");
    drop(rc3);
    println!("Dropped final reference");
}
