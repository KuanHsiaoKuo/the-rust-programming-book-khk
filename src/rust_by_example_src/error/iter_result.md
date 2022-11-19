# Iterating over `Result`s

~~~admonish tip title="An *Iter::map* operation might fail, for example:" collapsible=true
```rust,editable
fn main() {
    let strings = vec!["tofu", "93", "18"];
    let numbers: Vec<_> = strings
        .into_iter()
        .map(|s| s.parse::<i32>())
        .collect();
    println!("Results: {:?}", numbers);
}
```
~~~

Let's step through strategies for handling this.

## Ignore the failed items with `filter_map()`

~~~admonish tip title="*filter_map* calls a function and filters out the results that are *None*." collapsible=true
```rust,editable
fn main() {
    let strings = vec!["tofu", "93", "18"];
    let numbers: Vec<_> = strings
        .into_iter()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect();
    println!("Results: {:?}", numbers);
}
```
~~~

## Collect the failed items with `map_err()` and `filter_map()`

~~~admonish tip title="*map_err* calls a function with the error, so by adding that to the previous *filter_map* solution we can save them off to the side while iterating." collapsible=true
```rust,editable
fn main() {
    let strings = vec!["42", "tofu", "93", "999", "18"];
    let mut errors = vec![];
    let numbers: Vec<_> = strings
        .into_iter()
        .map(|s| s.parse::<u8>())
        .filter_map(|r| r.map_err(|e| errors.push(e)).ok())
        .collect();
    println!("Numbers: {:?}", numbers);
    println!("Errors: {:?}", errors);
}
```
~~~

## Fail the entire operation with `collect()`

~~~admonish tip title="*Result* implements *FromIterator* so that a vector of results (*Vec<Result<T, E>>*) can be turned into a result with a vector (*Result<Vec<T>, E>*). Once an *Result::Err* is found, the iteration will terminate." collapsible=true
```rust,editable
fn main() {
    let strings = vec!["tofu", "93", "18"];
    let numbers: Result<Vec<_>, _> = strings
        .into_iter()
        .map(|s| s.parse::<i32>())
        .collect();
    println!("Results: {:?}", numbers);
}
```
~~~

This same technique can be used with `Option`.

## Collect all valid values and failures with `partition()`

~~~admonish tip title="Example" collapsible=true
```rust,editable
fn main() {
    let strings = vec!["tofu", "93", "18"];
    let (numbers, errors): (Vec<_>, Vec<_>) = strings
        .into_iter()
        .map(|s| s.parse::<i32>())
        .partition(Result::is_ok);
    println!("Numbers: {:?}", numbers);
    println!("Errors: {:?}", errors);
}
```
~~~

When you look at the results, you'll note that everything is still wrapped in
`Result`.

~~~admonish tip title="A little more boilerplate is needed for this." collapsible=true
```rust,editable
fn main() {
    let strings = vec!["tofu", "93", "18"];
    let (numbers, errors): (Vec<_>, Vec<_>) = strings
        .into_iter()
        .map(|s| s.parse::<i32>())
        .partition(Result::is_ok);
    let numbers: Vec<_> = numbers.into_iter().map(Result::unwrap).collect();
    let errors: Vec<_> = errors.into_iter().map(Result::unwrap_err).collect();
    println!("Numbers: {:?}", numbers);
    println!("Errors: {:?}", errors);
}
```
~~~
