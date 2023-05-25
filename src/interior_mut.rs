/// In this test we intentionally create a data race to illustrate the kind of
/// bug that Rust's type system protects us from. This test uses unsafe code to
/// break Rust's safety model. In the test we have a cell which holds a number.
/// The cell allows itself to be modified using an immutable reference (BAD!)
/// and we take advantage of this to concurrently perform an addition operation
/// on the cell in two threads. This creates a data race which causes the cell
/// to have the wrong value at the end.
#[test]
#[should_panic]
fn test_data_race() {
    use std::{thread, time::Duration};

    // Define a cell with value equal to 40
    let cell = SimpleCell::new(40);
    let borrow = &cell;
    // Define a function which will increment the value of the cell by 1.
    let add_one = || {
        let mut x = borrow.get();
        x += 1;
        thread::sleep(Duration::from_millis(10));
        borrow.set(x);
    };
    std::thread::scope(|s| {
        // Create two threads, both incrementing the cell by 1.
        let thread1 = s.spawn(add_one);
        let thread2 = s.spawn(add_one);

        thread1.join().unwrap();
        thread2.join().unwrap();
    });

    // 40 + 1 + 1 = 42, so we expect the new value to be 42 because two increment calls were made.
    // However the actual value will be 41 because of the data race we created.
    assert_eq!(cell.get(), 42);
}

/// This is similar to the test above, but it uses a lock to prevent the data race.
/// Locks are useful because they allow for safe interior mutability, but they come
/// at the cost of performance. It is better to use lock-free concurrency patterns.
#[test]
fn test_safe_interior_mutability() {
    use std::{sync::RwLock, thread, time::Duration};

    let cell = RwLock::new(40_u32);
    let borrow = &cell;
    let add_one = || {
        let mut x = borrow.write().unwrap();
        thread::sleep(Duration::from_millis(10));
        *x += 1;
    };
    std::thread::scope(|s| {
        // Create two threads, both incrementing the cell by 1.
        let thread1 = s.spawn(add_one);
        let thread2 = s.spawn(add_one);

        thread1.join().unwrap();
        thread2.join().unwrap();
    });
    assert_eq!(*cell.read().unwrap(), 42);
}

/// A simple wrapper around a `u32` value with "interior mutability".
#[cfg(test)]
struct SimpleCell {
    value: u32,
}

#[cfg(test)]
impl SimpleCell {
    fn new(value: u32) -> Self {
        Self { value }
    }

    /// Read the value of the cell.
    fn get(&self) -> u32 {
        self.value
    }

    /// Modify the value of the cell.
    /// This function is intentionally written badly.
    /// It uses an immutable borrow to mutate the struct which breaks Rust's safety guarantees.
    /// Note: this is a real pattern called "interior mutability" and is implemented as part
    /// of the standard library in the `RefCell` data type. However, in that case there are runtime
    /// checks for multiple mutable borrows to prevent data races.
    fn set(&self, new_value: u32) {
        unsafe {
            let mutable_ptr =
                std::ptr::NonNull::new_unchecked((&self.value) as *const u32 as *mut u32).as_mut();
            *mutable_ptr = new_value;
        }
    }
}
