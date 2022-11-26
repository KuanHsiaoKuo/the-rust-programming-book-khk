# File/Directory hierarchy

Modules can be mapped to a `file/directory hierarchy`.

~~~admonish tip title="Let's break down the [visibility example][visibility] in files:" collapsible=true
```shell
$ tree .
.
├── my
│   ├── inaccessible.rs
│   └── nested.rs
├── my.rs
└── split.rs
```
~~~

~~~admonish tip title="In *split.rs*:" collapsible=true
```rust,ignore
// This declaration will look for a file named `my.rs` and will
// insert its contents inside a module named `my` under this scope
mod my;

fn function() {
    println!("called `function()`");
}

fn main() {
    my::function();

    function();

    my::indirect_access();

    my::nested::function();
}

```
~~~

- mod my; ->
  This declaration will look for a file named `my.rs` and will insert its contents inside a module named `my` under this scope

~~~admonish tip title="In *my.rs*:" collapsible=true
```rust,ignore
// Similarly `mod inaccessible` and `mod nested` will locate the `nested.rs`
// and `inaccessible.rs` files and insert them here under their respective
// modules
mod inaccessible;
pub mod nested;

pub fn function() {
    println!("called `my::function()`");
}

fn private_function() {
    println!("called `my::private_function()`");
}

pub fn indirect_access() {
    print!("called `my::indirect_access()`, that\n> ");

    private_function();
}
```
~~~

- Similarly `mod inaccessible` and `mod nested` will locate the `nested.rs`
- and `inaccessible.rs` files and insert them here under their respective modules

~~~admonish tip title="In *my/nested.rs*:" collapsible=true
```rust,ignore
pub fn function() {
    println!("called `my::nested::function()`");
}

#[allow(dead_code)]
fn private_function() {
    println!("called `my::nested::private_function()`");
}
```
~~~

~~~admonish tip title="In *my/inaccessible.rs*:" collapsible=true
```rust,ignore
#[allow(dead_code)]
pub fn public_function() {
    println!("called `my::inaccessible::public_function()`");
}
```
~~~

~~~admonish tip title="Let's check that things still work as before:" collapsible=true
```shell
$ rustc split.rs && ./split
called `my::function()`
called `function()`
called `my::indirect_access()`, that
> called `my::private_function()`
called `my::nested::function()`
```
~~~

[visibility]: visibility.md
