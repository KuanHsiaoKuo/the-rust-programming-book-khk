# Considering Rust

<!--ts-->


<!-- Created by https://github.com/ekalinin/github-markdown-toc -->
<!-- Added by: runner, at: Wed Apr 19 08:50:46 UTC 2023 -->

<!--te-->

## Introduction

Section Overview: The speaker introduces themselves and their experience with Rust. They explain the purpose of the talk
and what they will cover.

### Meet Rust

- Rust is a systems programming language made by Mozilla that had its 1.0 release about six years ago today. [](t=2:54)
- It is primarily community-driven, developed out in the open, and essentially the work of a large group of volunteers
  both associated with Mozilla and not. [](t=3:32)
- Rust has this slogan of fast, reliable, and productive, pick three. as sort of their marketing pitch or this notion
  of "fearless concurrency". [](t=3:23)

### Why Consider Rust?

- The speaker is not here to tell you that you must use Rust but rather to convince you why it's worth
  considering. [](t=0:01)
- In this talk, they will give comparisons against other languages at a relatively high level of what the strengths and
  weaknesses of Rust are compared to those languages.[](t=1:45)
- They will also tell you about some of Rust's features and their advantages as well as some drawbacks that should be
  considered before adopting it.[](t=1:45)

## What is Rust?

Section Overview: The speaker explains what systems programming means in relation to runtime environments and
performance. They discuss how Rust compares to other languages in terms of trade-offs between speed, reliability, and
productivity.

### Systems Programming

- Systems programming is for programs that care about their runtime environment or performance.
  [](t=3:00)

### Fast, Reliable & Productive

- Other languages force you to make trade-offs where you give up one of these three (fast, reliable & productive),
  whereas in Rust you do not have to.
  [](t=3:23)

### Rust's Features

- Rust is a compiled language that has strong static typing, so no typing occurs at runtime, it's all statically checked
  by the compiler.[](t=3:57)
- It is imperative but does have some functional aspects to it.[](t=4:06)

## Teaching Rust

Section Overview: The speaker explains that they will not be teaching Rust in this talk but rather giving an overview of
its strengths and weaknesses.

### Not Here to Teach Rust

- The goal of today's talk is not to teach you Rust. [](t=2:18)
- They will show you some code on certain slides and walk you through some of it to show particular features, but urge
  you to listen to the words that they say rather than try to exactly understand how the thing works. [](t=2:36)

## Conclusion

Section Overview: The speaker summarizes what was covered in the talk and encourages listeners to consider using Rust.

### Recap

- In this talk, the speaker gave comparisons against other languages at a relatively high level of what the strengths
  and weaknesses of Rust are compared to those languages.
  [](t=1:45)
- They also told us about some of Rust's features and their advantages as well as some drawbacks that should be
  considered before adopting it.
  [](t=1:45)

### Consider Using Rust

- The speaker encourages listeners to consider using Rust for their projects. [](t=2:07)

## Introduction to Rust

Section Overview: This section introduces Rust and its type system. It also compares Rust with other programming
languages.

### Rust's Type System

- Rust has an elaborate type system that provides guarantees at compile time.
- Types are used for enforcing many of these guarantees statically.
- Algebraic data types and pattern matching allow you to write nice code using types.
- The comprehensive static typing in Rust reduces runtime crashes compared to Python.

### Comparison with Other Languages

#### Python

- Rust is much faster than Python because it is a compiled language without a runtime, resulting in lower memory use and
  better multi-threading performance.

#### Java

- Rust has no runtime, resulting in lower memory use and generally higher performance than Java.
- Zero cost abstractions in Rust enable the use of additional classes without any runtime performance cost, unlike Java
  where this often comes at a cost.
- Data races and concurrent modification exceptions common in Java go away in Rust due to compile-time guarantees
  against them.

#### C/C++

- Segfaults, buffer overflows, null pointers, and data races common in C/C++ do not occur in Rust due to compile-time
  checks against them.
- The powerful type system provided by the language eliminates the need for void pointers or nasty type classing found
  in C/C++.

## Additional Features of Rust

Section Overview: This section covers additional features of the language that make it easy to write modular code bases.

### Unified Build System

- Cargo, the unified build system that comes with the compiler tool chain makes dependency management a lot easier.
- All libraries and tools use the same build system, making it easy to add external dependencies from other libraries or
  public repositories.

## Comparing Rust and Go

Section Overview: This section compares Rust and Go, two languages that are often compared to each other. The speaker
notes that while Go is an excellent language, Rust has some advantages over it.

### Rust vs. Go

- Rust has no garbage collector or runtime, which allows for higher performance than what can be achieved with Go code.
- In Rust, there are no null pointers, whereas in Go, you have to manually check for nil values. Additionally, error
  handling is much nicer in Rust than in Go.
- Concurrency is easy in both languages but safer in Rust as it ensures there are no data races. In contrast,
  concurrency in Go can lead to shooting oneself in the foot.
- Both languages have a strong type system and zero-cost abstractions but Rust also offers modern features such as
  efficient generics and algebraic data types with pattern matching. The toolchain that comes with the language makes it
  easier to use than older languages like C++.

## Features of Rust

Section Overview: This section discusses some of the key features of the Rust programming language.

### Modern Language

- One of the most important things about Rust is that it's a modern language that doesn't feel like a low-level systems
  language. It has nice and efficient generics similar to Java or type classes in C++ and algebraic data types with
  pattern matching. The toolchain that comes with the language makes it easier to use than older languages like C++.

### Efficient Generics

- Generics allow for code reuse by allowing users to choose what type T should be used when writing generic code. In
  Rust, this code gets compiled as if the generics weren't there leading to really fast runtime performance.
- The cost of function calls in generic code is optimized by the compiler, which can inline functions and compile them
  for each instance of T and P.

### Algebraic Data Types with Pattern Matching

- Rust has algebraic data types with pattern matching, which allows for more concise and expressive code. This feature
  is similar to sum types in functional programming languages like Haskell or ML.

### Modern Toolchain

- Rust comes with a modern toolchain that makes it easier to use than older languages like C++. Pain points that exist
  with many older languages are just gone in Rust.

## Rust Features Overview

Section Overview: This section provides an overview of some of the key features of Rust, including algebraic data types,
pattern matching, and modern tooling.

### Option Type and Algebraic Data Types

- The return type of a function in Rust is `Option reference to T`, which means that it can either return `Some` with a
  pointer or `None`.
- Rust allows for defining enumerations or types that contain other types. Unlike enums in other languages, these enums
  can contain other things.
- The `Option` type is an example of this. It is an enum that is generic over some T and contains either the `Some`
  variant with that T or the `None` variant.

### Pattern Matching

- Rust provides pattern matching to match over types that are these enums. This lets you tease out only the specific
  values and variants that you care about.
- The compiler will check that you have exhaustively matched, meaning if someone adds a variant to an enum later on,
  your code will not break. The compiler will tell you "you also need to update this match over here".

### Modern Tooling

- By default, in Rust, the compiler knows about things like tests and documentation. You can annotate any function in
  your code with #[test] attribute and then run cargo test to run it as a unit test.
- You can place these tests in separate files or outside your source directory, and then they will be compiled as
  integration tests which only have access to your public API. The compiler and build system know how to run all of
  them.

## Rust: Safety by Construction

Section Overview: This section discusses how Rust provides safety by construction and the primary reasons for this.

### Checking Pointers at Compile Time

- The Rust compiler ensures that every value in your program has a single owner and that owner is responsible for
  freeing that resource.
- The compiler checks two properties for every variable: there is only ever one owner, and no pointers outlive the
  owner.
- If you guarantee these invariants, you cannot have double frees or use after free errors.
- The borrow checker detects references that live past when their owners go away and rejects invalid code.

### Immutable Pointers and Variables

- Pointers and variables in Rust are immutable by default.
- Functions can take references to variables but not modify them unless given mutable access.

## Rust's Key Features

Section Overview: This section covers the key features of Rust, including its memory safety, thread safety, and no
hidden states.

### Memory Safety

- Rust variables are immutable by default.
- The `const` keyword in Rust applies transitively to all reachable values.
- Immutable references can be safely shared with other code without fear of modification.

### Thread Safety

- Rust types know whether it's safe for them to cross thread boundaries.
- Rc and Arc are two reference counted wrapper types that ensure thread safety.
- There can only ever be either one mutable reference or any number of immutable references to any given value at any
  given point in time. This ensures there can't be data races.
- The compiler checks for potential data races even across thread boundaries.

### No Hidden States

- Rust uses the type system to ensure that you check every case.
- Anytime where you see a reference, it is guaranteed not to be null.

## Rust's Error Handling

Section Overview: In this section, the speaker discusses Rust's error handling system and how it differs from other
languages.

### Rust's Error Handling System

- Rust's error handling system ensures that null checks are not forgotten by forcing the programmer to deal with errors.
- The compiler will not allow errors to be ignored accidentally.
- Examples of code that require error handling include find and parsing a string.
- The question mark operator is a shortcut for bubbling up errors to the caller.

## Advantages of Rust

Section Overview: In this section, the speaker discusses some advantages of using Rust over other programming languages.

### Lack of Runtime and Garbage Collector

- Rust does not have a garbage collector or runtime, which means there are no garbage collection pauses and lower memory
  overhead in general.
- You can issue system calls directly, such as fork and exec, which is often not possible in managed languages because
  the runtime controls the program execution.
- You can even run on a system without an OS because there is no code apart from your own that you need to deal with.

### Free FFI Calls

- FFI calls are free in Rust because there is no runtime that needs to be informed when calling out to another language
  through an API.

### Low-Level Control

- In Rust, you have low-level control over both allocation and dispatch. Unlike Java or Go, nothing is automatically
  heap allocated for you.
- You can opt into heap allocation using the Box keyword or by declaring a vector.
- You can swap out the entire allocator globally in your program if you wish, allowing you to use jemalloc, Google's new
  tcmalloc, or Microsoft's mimalloc.

## Rust: A Language for the Next 40 Years

Section Overview: This section introduces Rust as a language that offers low-level control and compatibility with other
languages.

### Low-Level Control

- Rust allows for low-level code that is often written in C or assembly.
- The `unsafe` keyword in Rust is used to assert invariants that the compiler cannot check. It provides an escape hatch
  for when low-level control is needed.
- The advantage of using `unsafe` is that it allows for memory errors to be audited easily, and it also makes it clear
  which parts of the code are dangerous.

### Compatibility with Other Languages

- Rust offers zero overhead FFI, allowing for easy integration with C and C++ code.
- Rust has great web assembly support and works well with traditional tools from C++, Python, and Java.
- Rust functions can be exported through some kind of C ABI, making them callable from C.

## [#](t=0:36:18s) Rust's Interoperability with Other Languages

Section Overview: This section discusses how Rust can interact with other languages using the C ABI, and how this makes
incremental rewriting easy.

### Rust's Compatibility with Other Languages

- Rust can interact with any language that can interact with a C ABI. This includes C, C++, Java's JNI, Go's cgo,
  Python's C++ bindings, and Ruby.
- Tools like bindgen and cbindgen generate Rust code or C headers for you from a given C header file or set of methods.
- The FFI (Foreign Function Interface) makes incremental rewriting easy by allowing you to implement modules in Rust and
  swap them out in your existing project.
- Rust has good interoperability with web assembly, which is built into browsers. It also has one of the best
  integrations for web assembly among programming languages.
- Traditional tools like perf, gdb, lldb, Valgrind work on Rust binaries because it is compiled using LLVM to machine
  code.

## Dependency Management and Standard Tools

Section Overview: This section covers the built-in dependency management and standard tools that come with Rust.

### Dependency Management

- Rust has a built-in dependency management system called Cargo.toml.
- The Rust compiler automatically fetches dependencies, builds them, and ensures that it builds any given dependency
  only once, even if it's transitively included by different paths.
- The Rust toolchain knows about versioning and will let you use the most up-to-date versions of your dependencies that
  your code is still compatible with.
- You can spin up your own private repository if you have libraries or "crates" in the Rust ecosystem lingo that you
  don't want to be public to the world.

### Standard Tools

- Rust comes with standardized tools like cargo format for formatting code, cargo doc for generating documentation from
  documentation comments, cargo clippy for linting, RLS and rust analyzer for IDE integration.
- Docs.rs is a website that automatically generates all of the documentation for every version of every library uploaded
  to the standard repository.

## Rust's Primary Features

Section Overview: In this section, the speaker discusses some of the primary features of the Rust language.

### Metaprogramming in Rust

- Rust has good support for writing programs that manipulate other Rust programs.
- This is known as metaprogramming and involves manipulating the AST or syntax tree of a program.
- Rust macros are more controlled than C++ or C macros and are fully-fledged Rust programs with well-defined syntax for
  transforming Rust trees.

### Procedural Macros

- Procedural macros allow you to write a Rust function from a stream of tokens to a new stream of tokens.
- This lets you do more elaborate rewritings of Rust code.
- The "serde" library provides serialization and deserialization for any type automatically for any format using these
  macros.

### Asynchronous Code Support

- Rust has built-in support for asynchronous code execution.
- You can choose your own runtime instead of being dictated by the language like in nodejs, Go, or Java.
- You define an executor that can take futures from the standard library and run them cooperatively.

## Rust: Advantages and Drawbacks

Section Overview: In this section, the speaker discusses the advantages and drawbacks of using Rust as a programming
language.

### Rust Learning Curve

- Rust has a steep learning curve due to its unique features such as the borrowed checker.
- The borrowed checker is a different way to reason about pointers in your program, which can be challenging to learn.
- Although it takes longer to learn than other languages, when your program compiles, it is more likely to run
  correctly.

### Ecosystem

- Rust is a relatively young language with a small ecosystem that has few maintainers.
- There are high-quality libraries available in the ecosystem like rayon and serde that can do things you couldn't do in
  other languages.
- Many crates are still in early stages, so there's some churn in the ecosystem. However, more libraries are hitting
  stable releases with stable APIs that you can depend on.
- The community is very friendly and supportive. You'll find people willing to help if you have trouble with Rust code
  or need support on a library.

### Object-Oriented Model

- Rust does not have an object-oriented model like Python or C++. This can be pretty alien for those who come from these
  languages.
- Although you still have things like interfaces, they work differently than in other languages.

## Conclusion

Section Overview: In this section, the speaker concludes by reiterating their goal of providing information for others
to decide whether Rust is right for them.

The speaker's goal was not to tell the audience that Rust is the right language but to provide information for them to
decide whether it's right for them. Rust has advantages such as being a performant language with zero-cost abstractions
and ensuring thread safety at compile time. However, it also has drawbacks like a steep learning curve due to its unique
features and a small ecosystem with few maintainers. Despite this, Rust is worth considering because of its friendly
community and high-quality libraries.

## Rust Downsides

Section Overview: This section discusses some of the downsides of using Rust.

### No Runtime Reflection

- Rust's lack of a runtime means that there is no runtime reflection.
- In Java, you can attach to a program in the middle of executing something and use the runtime to inspect various
  things about its execution. In Rust, you can't do the same.
- You can use gdb or lldb to try to tease out that information, but the lack of a runtime means that that information is
  less rich.
- It also means that the program can't really introspect itself at runtime.

### Longer Compile Times

- Because Rust is a fully compiled language and uses LLVM, there's sort of a long pipeline to get to the final binary,
  and everything has to be compiled.
- The compile times are somewhat longer than what you might be used to in something like Python where there's just no
  compile time at all.

### Compiling from Source

- In Rust, there aren't really pre-built libraries you can download. You sort of need to compile from source.
- If I want to call a function in your library, and your library function is generic, I need to compile the version of
  your method for the type I'm using. And that type might be defined in my library. So I need the source in order to do
  that compilation.

### Vendor Support

- Often when working with very large software projects you have dependencies on things that you didn't build. That were
  just given to you by a vendor.
- The FFI interface works pretty well as long as the library you get has an API that isn't too complicated. But
  sometimes, working with these large vendored things can be a bit of a pain.
- Similarly, if you work with a vendor that has some kind of tooling that they provide when you use their library that
  tooling might not work if you're working through Rust code because they might assume that you're just writing C++
  code.

## Rust Support for Windows

Section Overview: This section discusses the level of support that Rust has for Windows.

### Rust Compiler and Standard Library Support

- Rust compiler and standard library have full support for Windows.
- Windows is a tier one platform with official support.

### Ecosystem and Library Support

- The ecosystem, particularly libraries, is mainly focused on Linux and macOS.
- Some low-level libraries may lack support for Windows, especially those that interact directly with the operating
  system or implement asynchronous executors.

### Long-Term Viability

- Rust was rated the most loved programming language four years in a row in the StackOverflow developer survey.
- Big companies such as Microsoft, Google, Facebook, Amazon, CloudFlare, Mozilla, Atlassian, and npm are using Rust code
  in some of their projects.
- The increased company involvement in Rust itself helps to develop the enterprise part of the ecosystem and build more
  maturity over time.
- The Rust world is expanding with yearly conferences spanning the globe and hundreds of meetups around the world.
- The focus on developer experience by providing good compiler error messages makes a big difference when working on
  large refactors or when new to the language.

## Importance of Long-Term Viability

Section Overview: This section emphasizes why long-term viability is important when considering adopting a young
language like Rust.

### Reasons for Choosing a Young Language

- When choosing to adopt a young language like Rust, it's important to consider its potential to keep going for a long
  time.

### Why Choose Rust?

- Despite being relatively young compared to other languages, there are several reasons why choosing Rust is a good bet
  into the future:
    - Rated as most loved programming language four years in a row
    - Adoption by big companies
    - Good interoperability story
    - Increasing company involvement in Rust
    - Expanding Rust community
    - Focus on developer experience

## Incremental Rewrite with Rust

Section Overview: This section discusses how to rewrite projects incrementally with Rust.

### Benefits of Incremental Rewrite

- Incremental rewrite helps reduce the risk when adopting a new language like Rust.
- You can rewrite only the core concurrency that needs to be highly concurrent in Rust, leaving the rest of your
  application as it is.

### How to Do an Incremental Rewrite

- Identify the core part of your application that needs to be rewritten for safety or performance reasons.
- Rewrite that core part in Rust while leaving the rest of your application as it is.
- This approach allows you to gradually adopt Rust and its benefits without having to rewrite your entire project.

## Generated by Video Highlight

https://videohighlight.com/video/summary/DnT-LUQgc7s