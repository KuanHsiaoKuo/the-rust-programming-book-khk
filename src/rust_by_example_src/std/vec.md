# Vectors: re-sizable arrays

> Like slices, their size is not known at compile time, but they can grow or shrink at any time.

A vector is represented using 3 parameters:

- pointer to the data
- length
- capacity: The capacity indicates how much memory is reserved for the vector.

~~~admonish tip title="The vector can grow as long as the length is smaller than the capacity. When this threshold needs to be surpassed, the vector is reallocated with a larger capacity." collapsible=true
```rust,editable,ignore,mdbook-runnable
fn main() {
    // Iterators can be collected into vectors
    let collected_iterator: Vec<i32> = (0..10).collect();
    println!("Collected (0..10) into: {:?}", collected_iterator);

    // The `vec!` macro can be used to initialize a vector
    let mut xs = vec![1i32, 2, 3];
    println!("Initial vector: {:?}", xs);

    // Insert new element at the end of the vector
    println!("Push 4 into the vector");
    xs.push(4);
    println!("Vector: {:?}", xs);

    // Error! Immutable vectors can't grow
    collected_iterator.push(0);
    // FIXME ^ Comment out this line

    // The `len` method yields the number of elements currently stored in a vector
    println!("Vector length: {}", xs.len());

    // Indexing is done using the square brackets (indexing starts at 0)
    println!("Second element: {}", xs[1]);

    // `pop` removes the last element from the vector and returns it
    println!("Pop last element: {:?}", xs.pop());

    // Out of bounds indexing yields a panic
    println!("Fourth element: {}", xs[3]);
    // FIXME ^ Comment out this line

    // `Vector`s can be easily iterated over
    println!("Contents of xs:");
    for x in xs.iter() {
        println!("> {}", x);
    }

    // A `Vector` can also be iterated over while the iteration
    // count is enumerated in a separate variable (`i`)
    for (i, x) in xs.iter().enumerate() {
        println!("In position {} we have value {}", i, x);
    }

    // Thanks to `iter_mut`, mutable `Vector`s can also be iterated
    // over in a way that allows modifying each value
    for x in xs.iter_mut() {
        *x *= 3;
    }
    println!("Updated vector: {:?}", xs);
}
```
~~~

1. Iterators can be collected into vectors
2. The `vec!` macro can be used to initialize a vector
3. push to Insert new element at the end of the vector
4. `pop` removes the last element from the vector and returns it
5. Immutable vectors can't grow
6. The `len` method yields the number of elements currently stored in a vector
7. Indexing is done using the square brackets (indexing starts at 0)
8. Out of bounds indexing yields a panic
9. `Vector`s can be easily iterated over
10. Enumerate: A `Vector` can also be iterated over while the iteration count is enumerated in a separate variable (`i`)
11. Thanks to `iter_mut`, mutable `Vector`s can also be iterated over in a way that allows modifying each value

More `Vec` methods can be found under the
[std::vec][vec] module

[vec]: https://doc.rust-lang.org/std/vec/
