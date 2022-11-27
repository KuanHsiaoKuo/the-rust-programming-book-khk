# Integration testing

- [Unit tests][unit] are testing one module in isolation at a time: they're small and can test private code.
- Integration tests are external to your crate and use only its public interface in the same way any other code would.
- Their purpose is to test that many parts of your library work correctly together.

> Cargo looks for integration tests in `tests` directory next to `src`.

~~~admonish tip title="File *src/lib.rs*:" collapsible=true
```rust,ignore
// Define this in a crate called `adder`.
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
```
~~~

~~~admonish tip title="File with test: *tests/integration_test.rs*:" collapsible=true
```rust,ignore
#[test]
fn test_add() {
    assert_eq!(adder::add(3, 2), 5);
}
```
~~~

~~~admonish tip title="Running tests with *cargo test* command:" collapsible=true
```shell
$ cargo test
running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

     Running target/debug/deps/integration_test-bcd60824f5fbfe19

running 1 test
test test_add ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

   Doc-tests adder

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```
~~~

1. Each Rust source file in the `tests` directory is compiled as a separate crate.
2. In order to share some code between integration tests we can make a module with public
   functions, importing and using it within tests.

~~~admonish tip title="File *tests/common/mod.rs*:" collapsible=true
```rust,ignore
pub fn setup() {
    // some setup code, like creating required files/directories, starting
    // servers, etc.
}
```
~~~

~~~admonish tip title="File with test: *tests/integration_test.rs*" collapsible=true
```rust,ignore
// importing common module.
mod common;

#[test]
fn test_add() {
    // using common code.
    common::setup();
    assert_eq!(adder::add(3, 2), 5);
}
```
~~~

- Creating the module as `tests/common.rs` also works
- but is not recommended because the test runner will treat the file as a test crate and try to run tests inside it.

[unit]: unit_testing.md

[mod]: ../mod.md
