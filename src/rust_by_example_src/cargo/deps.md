# Dependencies

Most programs have dependencies on some libraries. If you have ever managed
dependencies by hand, you know how much of a pain this can be. Luckily, the Rust
ecosystem comes standard with `cargo`! `cargo` can manage dependencies for a
project.

~~~admonish tip title="To create a new Rust project: " collapsible=true
```sh
# A binary
cargo new foo

# A library
cargo new --lib bar
```
~~~

For the rest of this chapter, let's assume we are making a binary, rather than
a library, but all of the concepts are the same.

~~~admonish tip title="After the above commands, you should see a file hierarchy like this: " collapsible=true
```txt
.
├── bar
│   ├── Cargo.toml
│   └── src
│       └── lib.rs
└── foo
    ├── Cargo.toml
    └── src
        └── main.rs
```
~~~

1. The `main.rs` is the root source file for your new `foo` project -- nothing new there.
2. The `Cargo.toml` is the config file for `cargo` for this project.

~~~admonish tip title="If you look inside it, you should see something like this:" collapsible=true
```toml
[package]
name = "foo"
version = "0.1.0"
authors = ["mark"]

[dependencies]
```
~~~

1. The `name` field under `[package]` determines the name of the project.
2. This is used by `crates.io` if you publish the crate (more later).
3. It is also the name of the output binary when you compile.

4. The `version` field is a crate version number using [Semantic Versioning](http://semver.org/).

5. The `authors` field is a list of authors used when publishing the crate.

6. The `[dependencies]` section lets you add dependencies for your project.

For example, suppose that we want our program to have a great CLI:

- You can find lots of great packages on [crates.io](https://crates.io) (the official Rust
  package registry).
- One popular choice is [clap](https://crates.io/crates/clap).
- As of this writing, the most recent published version of `clap` is `2.27.1`.
- To add a dependency to our program, we can simply add the following to our
  `Cargo.toml` under `[dependencies]`: `clap = "2.27.1"`.
- And that's it! You can start using
  `clap` in your program.

`cargo` also supports [other types of dependencies][dependencies].

~~~admonish tip title="Here is just a small sampling: " collapsible=true
```toml
[package]
name = "foo"
version = "0.1.0"
authors = ["mark"]

[dependencies]
clap = "2.27.1" # from crates.io
rand = { git = "https://github.com/rust-lang-nursery/rand" } # from online repo
bar = { path = "../bar" } # from a path in the local filesystem
```
~~~

`cargo` is more than a dependency manager. All of the available
configuration options are listed in the [format specification][manifest] of
`Cargo.toml`.

1. cargo build: To build our project we can execute `cargo build` anywhere in the project
   directory (including subdirectories!).
2. cargo run: We can also do `cargo run` to build and run.
3. Notice that these commands will:

- resolve all dependencies,
- download crates if needed,
- and build everything, including your crate.

> (Note that it only rebuilds what it has not already built, similar to `make`).

Voila! That's all there is to it!


[manifest]: https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]: https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html
