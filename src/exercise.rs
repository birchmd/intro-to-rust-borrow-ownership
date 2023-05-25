/// Exercise: map-reduce
/// Map-Reduce is a common operation in distributed computing.
/// It takes a (large) array of input, applies a function to all the elements (map), then
/// reduces the results down to a single value using a second (associative) function.
/// Below is a single-threaded implementation of map-reduce in Rust.
/// Change the implementation to use two threads (or bonus points, `n` threads).
/// Avoid copying/cloning in your solution as much as possible.
pub fn map_red<T, U, M, R>(data: &[T], map_fn: M, reduce: R, init: U) -> U
where
    // Don't worry about the Send + Sync everywhere, that's just to tell rust the data
    // can be shared between threads safely.
    T: Send + Sync,
    U: Clone + Send + Sync,
    M: Fn(&T) -> U + Send + Sync,
    R: Fn(U, U) -> U + Send + Sync,
{
    //data.iter().map(map_fn).fold(init, reduce)
    let chunk_size = data.len() / 2;
    let process_chunk =
        |input: &[T], init: U| -> U { input.iter().map(&map_fn).fold(init, &reduce) };
    std::thread::scope(|s| {
        let init_copy = init.clone();
        let handle_1 = s.spawn(|| process_chunk(&data[0..chunk_size], init_copy));
        let handle_2 = s.spawn(|| process_chunk(&data[chunk_size..], init));
        let result_1 = handle_1.join().unwrap();
        let result_2 = handle_2.join().unwrap();
        reduce(result_1, result_2)
    })
}

#[test]
fn test_map_red() {
    let data = vec!["x", "xx", "xxx", "xxxx"];
    let result = map_red(&data, |x| x.len(), |x, y| x + y, 0);
    assert_eq!(result, 10);
}
