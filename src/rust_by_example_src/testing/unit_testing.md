# Unit testing for panic: `#[cfg(test)]`, `#[test]`, `#[should_panic]`, `#[ignore]`

<!--ts-->
* [Unit testing for panic: #[cfg(test)], #[test], #[should_panic], #[ignore]](#unit-testing-for-panic-cfgtest-test-should_panic-ignore)
   * [Unit tests basic](#unit-tests-basic)
   * [Tests and ?](#tests-and-)
   * [Testing panics: #[should_panic]](#testing-panics-should_panic)
   * [Running specific tests](#running-specific-tests)
   * [Ignoring tests: #[ignore]](#ignoring-tests-ignore)

<!-- Created by https://github.com/ekalinin/github-markdown-toc -->
<!-- Added by: runner, at: Sun Apr  9 15:51:22 UTC 2023 -->

<!--te-->

- Tests are Rust functions that verify that the non-test code is functioning in the expected manner.
- The bodies of test functions typically perform some setup, run the code we want to test, then assert whether the results are what we expect.

## Unit tests basic

1. Most unit tests go into a `tests` [mod][mod] with the `#[cfg(test)]` [attribute][attribute].
2. Test functions are marked with the `#[test]` attribute.

Tests fail when something in the test function [panics][panic].

> There are some helper [macros][macros]:

* `assert!(expression)` - panics if expression evaluates to `false`.
* `assert_eq!(left, right)` and `assert_ne!(left, right)` - testing left and right expressions for equality and inequality respectively.

~~~admonish tip title="Unit tests example" collapsible=true
```rust,ignore
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

// This is a really bad adding function, its purpose is to fail in this
// example.
#[allow(dead_code)]
fn bad_add(a: i32, b: i32) -> i32 {
    a - b
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
    }

    #[test]
    fn test_bad_add() {
        // This assert would fire and test will fail.
        // Please note, that private functions can be tested too!
        assert_eq!(bad_add(1, 2), 3);
    }
}
```
~~~

1. Note this useful idiom: importing names from outer (for mod tests) scope.
2. This assert would fire and test will fail.
3. Please note, that private functions can be tested too!

~~~admonish tip title="Tests can be run with *cargo test*." collapsible=true
```shell
$ cargo test

running 2 tests
test tests::test_bad_add ... FAILED
test tests::test_add ... ok

failures:

---- tests::test_bad_add stdout ----
        thread 'tests::test_bad_add' panicked at 'assertion failed: `(left == right)`
  left: `-1`,
 right: `3`', src/lib.rs:21:8
note: Run with `RUST_BACKTRACE=1` for a backtrace.


failures:
    tests::test_bad_add

test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
```
~~~

## Tests and `?`

None of the previous unit test examples had a return type.

~~~admonish tip title="But in Rust 2018, your unit tests can return *Result<()>*, which lets you use *?* in them! This can make them much more concise." collapsible=true
```rust,editable
fn sqrt(number: f64) -> Result<f64, String> {
    if number >= 0.0 {
        Ok(number.powf(0.5))
    } else {
        Err("negative floats don't have square roots".to_owned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sqrt() -> Result<(), String> {
        let x = 4.0;
        assert_eq!(sqrt(x)?.powf(2.0), x);
        Ok(())
    }
}
```
~~~

1. to_owned()
2. assert_eq!(sqrt(x)?.powf(2.0), x);

See ["The Edition Guide"][editionguide] for more details.

## Testing panics: #[should_panic]

> To check functions that should panic under certain circumstances, use attribute `#[should_panic]`:

- This attribute accepts optional parameter `expected = ` with the text of the panic message.
- If your function can panic in multiple ways, it helps make sure your test is testing the correct panic.

~~~admonish tip title="#[should_panic] example" collapsible=true
```rust,ignore
pub fn divide_non_zero_result(a: u32, b: u32) -> u32 {
    if b == 0 {
        panic!("Divide-by-zero error");
    } else if a < b {
        panic!("Divide result is zero");
    }
    a / b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_divide() {
        assert_eq!(divide_non_zero_result(10, 2), 5);
    }

    #[test]
    #[should_panic]
    fn test_any_panic() {
        divide_non_zero_result(1, 0);
    }

    #[test]
    #[should_panic(expected = "Divide result is zero")]
    fn test_specific_panic() {
        divide_non_zero_result(1, 10);
    }
}
```
~~~

~~~admonish tip title="Running these tests gives us:" collapsible=true
```shell
$ cargo test

running 3 tests
test tests::test_any_panic ... ok
test tests::test_divide ... ok
test tests::test_specific_panic ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

   Doc-tests tmp-test-should-panic

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```
~~~

## Running specific tests

~~~admonish tip title="To run specific tests one may specify the test name to *cargo test* command." collapsible=true
```shell
$ cargo test test_any_panic
running 1 test
test tests::test_any_panic ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out

   Doc-tests tmp-test-should-panic

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```
~~~

~~~admonish tip title="To run multiple tests one may specify part of a test name that matches all the tests that should be run." collapsible=true
```shell
$ cargo test panic
running 2 tests
test tests::test_any_panic ... ok
test tests::test_specific_panic ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out

   Doc-tests tmp-test-should-panic

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```
~~~

## Ignoring tests: #[ignore]

~~~admonish tip title="Tests can be marked with the *#[ignore]* attribute to exclude some tests. Or to run them with command *cargo test -- --ignored*" collapsible=true
```rust
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 2), 4);
    }

    #[test]
    fn test_add_hundred() {
        assert_eq!(add(100, 2), 102);
        assert_eq!(add(2, 100), 102);
    }

    #[test]
    #[ignore]
    fn ignored_test() {
        assert_eq!(add(0, 0), 0);
    }
}
```
~~~

~~~admonish tip title="cargo test" collapsible=true
```shell
$ cargo test
running 3 tests
test tests::ignored_test ... ignored
test tests::test_add ... ok
test tests::test_add_hundred ... ok

test result: ok. 2 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out

   Doc-tests tmp-ignore

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

$ cargo test -- --ignored
running 1 test
test tests::ignored_test ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

   Doc-tests tmp-ignore

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```
~~~

[attribute]: ../attribute.md

[panic]: ../std/panic.md

[macros]: ../macros.md

[mod]: ../mod.md

[editionguide]: https://doc.rust-lang.org/edition-guide/rust-2018/error-handling-and-panics/question-mark-in-main-and-tests.html
