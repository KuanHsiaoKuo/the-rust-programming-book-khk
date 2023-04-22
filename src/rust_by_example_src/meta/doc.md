# Documentation: cargo doc

<!--ts-->


<!-- Created by https://github.com/ekalinin/github-markdown-toc -->
<!-- Added by: runner, at: Sat Apr 22 15:51:14 UTC 2023 -->

<!--te-->
Use `cargo doc` to build documentation in `target/doc`.

Use `cargo test` to run all tests (including documentation tests), and `cargo test --doc` to only run documentation tests.

These commands will appropriately invoke `rustdoc` (and `rustc`) as required.

## Doc comments

Doc comments are very useful for big projects that require documentation. When
running `rustdoc`, these are the comments that get compiled into
documentation.

~~~admonish tip title="They are denoted by a *///*, and support [Markdown]." collapsible=true
````rust,editable,ignore
#![crate_name = "doc"]

/// A human being is represented here
pub struct Person {
    /// A person must have a name, no matter how much Juliet may hate it
    name: String,
}

impl Person {
    /// Returns a person with the name given them
    ///
    /// # Arguments
    ///
    /// * `name` - A string slice that holds the name of the person
    ///
    /// # Examples
    ///
    /// ```
    /// // You can have rust code between fences inside the comments
    /// // If you pass --test to `rustdoc`, it will even test it for you!
    /// use doc::Person;
    /// let person = Person::new("name");
    /// ```
    pub fn new(name: &str) -> Person {
        Person {
            name: name.to_string(),
        }
    }

    /// Gives a friendly hello!
    ///
    /// Says "Hello, [name](Person::name)" to the `Person` it is called on.
    pub fn hello(& self) {
        println!("Hello, {}!", self.name);
    }
}

fn main() {
    let john = Person::new("John");

    john.hello();
}
````
~~~

~~~admonish tip title="To run the tests, first build the code as a library, then tell *rustdoc* where to find the library so it can link it into each doctest program:" collapsible=true
```shell
$ rustc doc.rs --crate-type lib
$ rustdoc --test --extern doc="libdoc.rlib" doc.rs
```
~~~

## Doc attributes

Below are a few examples of the most common `#[doc]` attributes used with `rustdoc`.

### `inline`

~~~admonish tip title="Used to inline docs, instead of linking out to separate page." collapsible=true
```rust,ignore
#[doc(inline)]
pub use bar::Bar;

/// bar docs
mod bar {
    /// the docs for Bar
    pub struct Bar;
}
```
~~~

### `no_inline`

~~~admonish tip title="Used to prevent linking out to separate page or anywhere." collapsible=true
```rust,ignore
// Example from libcore/prelude
#[doc(no_inline)]
pub use crate::mem::drop;
```
~~~

### `hidden`

~~~admonish tip title="Using this tells *rustdoc* not to include this in documentation:" collapsible=true
```rust,editable,ignore
// Example from the futures-rs library
#[doc(hidden)]
pub use self::async_await::*;
```
~~~

For documentation, `rustdoc` is widely used by the community. It's what is used to generate the [std library docs](https://doc.rust-lang.org/std/).

### See also:

- [The Rust Book: Making Useful Documentation Comments][book]
- [The rustdoc Book][rustdoc-book]
- [The Reference: Doc comments][ref-comments]
- [RFC 1574: API Documentation Conventions][api-conv]
- [RFC 1946: Relative links to other items from doc comments (intra-rustdoc links)][intra-links]
- [Is there any documentation style guide for comments? (reddit)][reddit]

[markdown]: https://en.wikipedia.org/wiki/Markdown

[book]: https://doc.rust-lang.org/book/ch14-02-publishing-to-crates-io.html#making-useful-documentation-comments

[ref-comments]: https://doc.rust-lang.org/stable/reference/comments.html#doc-comments

[rustdoc-book]: https://doc.rust-lang.org/rustdoc/index.html

[api-conv]: https://rust-lang.github.io/rfcs/1574-more-api-documentation-conventions.html#appendix-a-full-conventions-text

[intra-links]: https://rust-lang.github.io/rfcs/1946-intra-rustdoc-links.html

[reddit]: https://www.reddit.com/r/rust/comments/ahb50s/is_there_any_documentation_style_guide_for/
