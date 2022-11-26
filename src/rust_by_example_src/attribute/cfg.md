# `cfg`: Configuration conditional checks

> Configuration conditional checks are possible through two different operators:

1. the `cfg` attribute: `#[cfg(...)]` in `attribute position`
2. the `cfg!` macro: `cfg!(...)` in `boolean expressions`

While the former enables conditional compilation, the latter conditionally
evaluates to `true` or `false` literals allowing for checks at run-time.

Both utilize identical argument syntax.

`cfg!`, unlike `#[cfg]`, does not remove any code and only evaluates to true or false.

~~~admonish tip title="For example, all blocks in an if/else expression need to be valid when *cfg!* is used for the condition, regardless of what *cfg!* is evaluating." collapsible=true
```rust,editable
// This function only gets compiled if the target OS is linux
#[cfg(target_os = "linux")]
fn are_you_on_linux() {
    println!("You are running linux!");
}

// And this function only gets compiled if the target OS is *not* linux
#[cfg(not(target_os = "linux"))]
fn are_you_on_linux() {
    println!("You are *not* running linux!");
}

fn main() {
    are_you_on_linux();

    println!("Are you sure?");
    if cfg!(target_os = "linux") {
        println!("Yes. It's definitely linux!");
    } else {
        println!("Yes. It's definitely *not* linux!");
    }
}
```
~~~

1. This function only gets compiled if the target OS is linux
2. And this function only gets compiled if the target OS is *not* linux

### See also:

[the reference][ref], [`cfg!`][cfg], and [macros][macros].

[cfg]: https://doc.rust-lang.org/std/macro.cfg!.html

[macros]: ../macros.md

[ref]: https://doc.rust-lang.org/reference/attributes.html#conditional-compilation
