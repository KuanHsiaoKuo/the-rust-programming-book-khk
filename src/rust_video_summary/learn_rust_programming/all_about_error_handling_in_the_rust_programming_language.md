# All About Error Handling In the Rust Programming Language

<!--ts-->
* [All About Error Handling In the Rust Programming Language](#all-about-error-handling-in-the-rust-programming-language)
   * [Introduction to Errors in Rust](#introduction-to-errors-in-rust)
      * [What is an Error?](#what-is-an-error)
      * [Dealing with Errors](#dealing-with-errors)
   * [Handling File I/O Errors](#handling-file-io-errors)
      * [Understanding File I/O](#understanding-file-io)
      * [Handling File I/O Errors](#handling-file-io-errors-1)
      * [Introduction to Result Enum](#introduction-to-result-enum)
      * [Defining Custom Structs with Results](#defining-custom-structs-with-results)
      * [Desugaring Question Mark Operator](#desugaring-question-mark-operator)
   * [Handling Errors in Rust](#handling-errors-in-rust)
      * [Printing Out Errors](#printing-out-errors)
      * [Exiting Cleanly](#exiting-cleanly)
   * [Debugging Rust Code](#debugging-rust-code)
      * [Compile Error: Expected a Unit but Found usize](#compile-error-expected-a-unit-but-found-usize)
      * [Compile Error: Expected a Result but Found an Enum](#compile-error-expected-a-result-but-found-an-enum)
      * [Compile Error: Not Found in Scope Expecting a Type Here Because of Type Description I Cannot Find the Value Buffer](#compile-error-not-found-in-scope-expecting-a-type-here-because-of-type-description-i-cannot-find-the-value-buffer)
      * [No Such File or Directory Error](#no-such-file-or-directory-error)
   * [Rust Language and Productivity](#rust-language-and-productivity)
      * [Influence of Rust on Productivity](#influence-of-rust-on-productivity)
   * [Q&amp;A Session](#qa-session)
      * [Viewer Interaction](#viewer-interaction)
   * [Creating a Cache in Rust](#creating-a-cache-in-rust)
      * [Creating a Cache](#creating-a-cache)
   * [Building a Cache](#building-a-cache)
      * [Building a Cache](#building-a-cache-1)
      * [Fixing Compiler Errors](#fixing-compiler-errors)
      * [Additional Information](#additional-information)
   * [Unlocking Content and Rust Basics](#unlocking-content-and-rust-basics)
      * [Unlocking Content](#unlocking-content)
      * [Rust Programming Language Basics](#rust-programming-language-basics)
   * [Understanding Error Types in Rust](#understanding-error-types-in-rust)
      * [Error Types in Rust](#error-types-in-rust)
   * [Composing Error Types and Networking Example](#composing-error-types-and-networking-example)
      * [Composing Error Types](#composing-error-types)
      * [Networking Example](#networking-example)
   * [Writing to a Server and Handling Errors](#writing-to-a-server-and-handling-errors)
      * [Attempting to Write Code for Reading Data from a Server](#attempting-to-write-code-for-reading-data-from-a-server)
      * [Understanding Formatting Strings and IPv4 Addresses](#understanding-formatting-strings-and-ipv4-addresses)
      * [Handling Errors in Rust Programming](#handling-errors-in-rust-programming)
      * [Conceptualizing Reading Data from a Server](#conceptualizing-reading-data-from-a-server)
      * [Understanding Traits in Rust Programming](#understanding-traits-in-rust-programming)
   * [Implementing the "From" Trait](#implementing-the-from-trait)
      * [Strategies for Implementing the "From" Trait](#strategies-for-implementing-the-from-trait)
      * [Handling Errors](#handling-errors)
   * [Running Cargo and Address Parser](#running-cargo-and-address-parser)
      * [Running Cargo](#running-cargo)
      * [Troubleshooting the Error](#troubleshooting-the-error)
      * [Parsing Issues](#parsing-issues)
   * [Understanding Box Pointers and Custom Error Types](#understanding-box-pointers-and-custom-error-types)
      * [Box Pointers](#box-pointers)
      * [Custom Error Types](#custom-error-types)
   * [Creating a Struct for Dirty Data](#creating-a-struct-for-dirty-data)
      * [Creating the "Dirty Data" Struct](#creating-the-dirty-data-struct)
   * [Adding Debug Trait to the Struct](#adding-debug-trait-to-the-struct)
      * [Adding Debug Trait](#adding-debug-trait)
   * [Distinguishing Error Types](#distinguishing-error-types)
      * [Creating Address Parser Error](#creating-address-parser-error)
   * [Converting Between Different Error Types](#converting-between-different-error-types)
      * [Using Rust's "From" Trait for Conversion](#using-rusts-from-trait-for-conversion)
   * [Using Zero-Sized Types for Internal Errors](#using-zero-sized-types-for-internal-errors)
      * [Using Zero-Sized Types](#using-zero-sized-types)
   * [Using Rust to Handle Errors](#using-rust-to-handle-errors)
      * [Bringing Modules into Local Scope](#bringing-modules-into-local-scope)
      * [Understanding Function Pointers](#understanding-function-pointers)
      * [Strategies for Handling Errors](#strategies-for-handling-errors)
         * [Building Up Rapid Types](#building-up-rapid-types)
         * [Using Trait Objects](#using-trait-objects)
         * [Unwrapping Errors](#unwrapping-errors)
      * [Mixing Options and Results](#mixing-options-and-results)
   * [Error Conditions and Conversions](#error-conditions-and-conversions)
      * [Error Conditions](#error-conditions)
      * [Conversions](#conversions)
   * [Using Options and Results Together](#using-options-and-results-together)
      * [Example Scenario](#example-scenario)
   * [Converting Option to Result](#converting-option-to-result)
      * [Converting Option i32 to Result](#converting-option-i32-to-result)
      * [Understanding Rust's Orphan Rule](#understanding-rusts-orphan-rule)
   * [Defining Traits for Arbitrary Types](#defining-traits-for-arbitrary-types)
      * [Wrapping Option in a New Type](#wrapping-option-in-a-new-type)
      * [Limitations of Using Conversion Mechanisms](#limitations-of-using-conversion-mechanisms)
   * [Conclusion](#conclusion)
   * [Generated by Video Highlight](#generated-by-video-highlight)

<!-- Created by https://github.com/ekalinin/github-markdown-toc -->
<!-- Added by: runner, at: Sat Apr 22 15:51:31 UTC 2023 -->

<!--te-->

## Introduction to Errors in Rust

Section Overview: In this section, the speaker introduces the concept of errors and how they are handled in Rust.

### What is an Error?

- An error is a message that indicates something has gone wrong during program execution.
- A panic is a type of error that results in a crash. It can be altered to have different semantics.
- Errors and results happen most often when dealing with input/output (I/O).
- I/O refers to input/output not of function arguments but of devices interacting with a program from the outside world.

### Dealing with Errors

- The speaker introduces three strategies for dealing with errors: Propagating, Returning, and Unwrapping.
- The example provided in the book demonstrates how to use the Result type to handle errors in Rust.

## Handling File I/O Errors

Section Overview: In this section, the speaker discusses file input/output (I/O), its importance, and how it can result
in errors.

### Understanding File I/O

- File I/O refers to reading from or writing to files on disk.
- Reading from or writing to files can result in errors due to various reasons such as permission issues or file not
  found.

### Handling File I/O Errors

- The Result type can be used to handle file I/O errors by propagating them up through functions.
- Another way of handling file I/O errors is by using unwrap() method which will either return Ok value or panic if
  there's an error. However, this approach should be used carefully as it can result in unexpected crashes.
- The speaker recommends using the ? operator to handle file I/O errors as it provides a concise and safe way of
  handling errors.

## [#](t=0:06:13s) Result Enum

Section Overview: In this section, the speaker introduces the `Result` enum and explains how it is a more elegant
mechanism than just getting a panic error message. They also explain that `Result` is an enum with two variants - `Ok`
and `Error`.

### Introduction to Result Enum

- The `Result` enum is a more useful mechanism than just getting a panic error message. [](t=0:06:13s)
- The `Result` enum has two variants - `Ok` and `Error`. [](t=0:07:21s)

### Defining Custom Structs with Results

- To define custom structs that can use results, we need to implement the error for config. [](t=0:09:52s)
- We can add methods to our custom struct. [](t=0:10:21s)

### Desugaring Question Mark Operator

- The speaker talks about desugaring the question mark operator. [](t=0:11:24s)
- If we have a result type, we can call match on it. [](t=0:11:51s)

## Handling Errors in Rust

Section Overview: In this section, the speaker discusses how to handle errors in Rust and demonstrates how to print out
errors and exit the program cleanly.

### Printing Out Errors

- To print out an error type, use `println!`.
- Use the debug syntax to make sure that there is a human-readable version of what is being said.

### Exiting Cleanly

- To exit cleanly, use `std::process::exit`.
- Exit with a value of 1 using `std::process::exit(1)`.

## Debugging Rust Code

Section Overview: In this section, the speaker demonstrates how to debug Rust code by going through compile errors and
fixing them.

### Compile Error: Expected a Unit but Found usize

- This error occurs because Rust is an expression-based language.
- Add a semicolon at the end of the expression to end it and return an empty unit type.

### Compile Error: Expected a Result but Found an Enum

- The type of the function has changed.
- Duplicate the function so that it can be brought back later if needed.

### Compile Error: Not Found in Scope Expecting a Type Here Because of Type Description I Cannot Find the Value Buffer

- Rust does not have keyword arguments.
- Change mutable variables to immutable variables.

### No Such File or Directory Error

- Wrap "Ok" in a unit.
- A unit is similar to void in other languages.

## Rust Language and Productivity

Section Overview: In this section, the speaker talks about how Rust language can influence productivity and growth.

### Influence of Rust on Productivity

- Rust language has the ability to influence productivity and growth.
- The comparison between two versions is not accurate because it involves reading into a string.
- The speaker is looking at their structure configuration and wants to simplify things by taking file away and calling
  it a string.
- The speaker discusses how to enable synthetic sugar for custom types.

## Q&A Session

Section Overview: In this section, the speaker interacts with viewers who have joined the stream.

### Viewer Interaction

- The speaker welcomes everyone who has just joined the stream and invites them to ask questions in the chat.
- The speaker notices that his chat has been hidden and offers to send a DM to anyone who wants to join his Discord.

## Creating a Cache in Rust

Section Overview: In this section, the speaker creates a cache in Rust.

### Creating a Cache

- The speaker creates boilerplate code for creating a cache.
- They change source file to source data.
- Source data is cloned, and dirty is defined as true.
- The ability to update data is updated.
- A flush method is created which guarantees that data is physically written before returning.
- The concept of "blush" is discussed.
- The speaker mentions that they want to change their mind halfway through building something on the fly.
- A value is returned as a result of a string or another string.
- The speaker acknowledges that using strings in multiple places is not a good way to do Rust.
- If self is dirty, garbage is returned.

## Building a Cache

Section Overview: In this section, the speaker discusses building a cache and enabling it to be printed on the screen.
They also talk about deriving debug that will enable syntax to happen.

### Building a Cache

- The speaker creates a super cache using the main function.
- They label the point and enable it to be printed on the screen.
- The compiler figures out the code that needs to get written.
- There are some compiler errors that need fixing.

### Fixing Compiler Errors

- Line 19: The speaker changes "return self" to "return ()".
- Line 20: "self.data" is an unknown field, so they change it to "source data".
- There is an unexpected token warning that needs fixing by removing some things.

### Additional Information

- The chat has been paused for new viewers who are interested in learning about Rust programming language.
- A link to a chapter of the speaker's book on Rust is provided in the chat for those who want more information.

## Unlocking Content and Rust Basics

Section Overview: In this section, the speaker talks about unlocking content for a chapter and introduces Rust
programming language basics.

### Unlocking Content

- The publisher has unlocked the content of a chapter for free.
- Anyone watching can access it.

### Rust Programming Language Basics

- Rust is immutable by default, which means that things are read-only until you satisfy them as mutable (read-write).
- Update method is used to update data in Rust.
- Booleans are not a good data type because they can always get it wrong.

## Understanding Error Types in Rust

Section Overview: In this section, the speaker discusses error types in Rust programming language.

### Error Types in Rust

- Error types don't compose in Rust programming language.
- Errors from different places cannot be treated similarly.
- A better way to handle errors would be to use enums instead of booleans.

## Composing Error Types and Networking Example

Section Overview: In this section, the speaker talks about composing error types and gives an example of networking
using Rust programming language.

### Composing Error Types

- As we become more proficient rust programmers, our error types don't compose.
- We need to treat errors from different places similarly.

### Networking Example

- The speaker gives an example of connecting to a database using networking with Rust programming language.
- Type inference can be used to ask for an IP address.
- IPv4 address has a cool thing where it can overload methods and extend strings to parse them.

## Writing to a Server and Handling Errors

Section Overview: In this section, the speaker attempts to write code that reads data from a server, stores it in a
cache, and writes it to disk. They encounter errors along the way and discuss how to handle them.

### Attempting to Write Code for Reading Data from a Server

- The speaker attempts to write code for reading data from a server but encounters an error.
- They discuss the normal state of confusion when encountering errors while coding.
- The speaker tries another approach by writing code that does not use the server or file yet.

### Understanding Formatting Strings and IPv4 Addresses

- The speaker explains that the `write` macro takes a formatting string as an argument.
- They mention that IPv4 addresses know how to write themselves to the console.
- The speaker discusses how they need a reason for certain code elements to be present.

### Handling Errors in Rust Programming

- The speaker encounters errors related to missing imports and typos in their code.
- They encounter another error message suggesting they consider importing something they believe they already imported.
- The speaker realizes there was a typo in their code causing an error message.
- They explain that two types of errors are not friends because they are independent types with identical syntax.

### Conceptualizing Reading Data from a Server

- The speaker explains their goal of reading data from a server, storing it in cache, and writing it to disk.
- They demonstrate how they will pretend to read data from a server and write it to disk.
- The speaker acknowledges a question about the trait for pass and promises to answer it soon.

### Understanding Traits in Rust Programming

- The speaker explains that they rewrote a chapter on traits because they are fundamental to Rust programming.
- They demonstrate how to find where a trait is implemented by searching for it in the code.
- The speaker answers a question about the trait for pass.

## Implementing the "From" Trait

Section Overview: In this section, the speaker explains how to implement the "From" trait in Rust and provides
strategies for handling errors.

### Strategies for Implementing the "From" Trait

- To implement the "From" trait, find the method you're interested in and look up its trait bounds.
    - [](t=0:48:37)
- Use a trait object to implement a particular trait.
    - [](t=0:52:42)

### Handling Errors

- Rust has added implicit behavior that automatically attempts to convert things to the type defined at the return
  statement. This can cause confusion when dealing with different types of errors.
    - [](t=0:49:57)
- When dealing with I/O and other things, it's important to be able to join them together.
    - [](t=0:51:21)
- One strategy for handling errors is using a trait object.
    - [](t=0:52:42)

## Running Cargo and Address Parser

Section Overview: In this section, the speaker runs cargo and encounters an error with the address parser.

### Running Cargo

- The speaker runs cargo.
- An error occurs with the address parser when running cargo.

### Troubleshooting the Error

- The speaker realizes they need to include different syntax.
- Despite making changes, the same warning persists.
- The speaker removes a compiler warning by ignoring a parameter.
- Another warning is silenced using code.

### Parsing Issues

- The speaker tries to figure out why parsing is not working.
- They try a different approach that results in a different error message.
- Adding a box resolves the issue of parsing errors.

## Understanding Box Pointers and Custom Error Types

Section Overview: This section covers box pointers and custom error types in Rust programming.

### Box Pointers

- A box pointer is defined as a pointer to an implementation of an error type.
- In this case, control flow needs to be sent to three different places depending on what gets called, which requires
  using pointers.

### Custom Error Types

- Using string as a way to send warning messages is discussed.
- Implementing trait objects can result in losing the ability for the compiler to detect concrete types.

## Creating a Struct for Dirty Data

Section Overview: In this section, the speaker creates a struct called "dirty data" and explains why they are doing so.

### Creating the "Dirty Data" Struct

- The speaker creates a struct called "dirty data" that will return data as dirty.
- They explain that they will automatically implement it and will explain why later.

## Adding Debug Trait to the Struct

Section Overview: In this section, the speaker adds the debug trait to the previously created "dirty data" struct.

### Adding Debug Trait

- The speaker derives debug for the "dirty data" struct.
- This allows it to print to the screen.

## Distinguishing Error Types

Section Overview: In this section, the speaker distinguishes one error type from others by creating an address parser
error in their file.

### Creating Address Parser Error

- The speaker adds standard net and address parser error in their file.
- They want to distinguish one error type from others.

## Converting Between Different Error Types

Section Overview: In this section, the speaker discusses how they need to convert between different error types and how
they plan on doing so using Rust's "from" trait.

### Using Rust's "From" Trait for Conversion

- The speaker explains that Rust's "from" trait enables one type to be used as another type and Rust will automatically
  convert them when needed.
- They add code involving implementing these traits for conversion purposes.
- They mention that strings are not ideal for error types because of potential memory allocation issues.

## Using Zero-Sized Types for Internal Errors

Section Overview: In this section, the speaker discusses using zero-sized types for internal errors and the potential
issues that may arise.

### Using Zero-Sized Types

- The speaker suggests using zero-sized types for internal errors to minimize memory usage.
- They mention the potential issue of making a mistake and causing a crash.
- They express uncertainty about their decision.

## Using Rust to Handle Errors

Section Overview: In this section, the speaker discusses how to handle errors in Rust and demonstrates different
strategies for dealing with them.

### Bringing Modules into Local Scope

- To save time typing later on, the speaker likes to bring modules into local scope using `use std::net`.
- Using the full path is more explicit and makes the compiler happy, but it's not necessary.
- The speaker chooses to keep everything quite full and somewhat ugly.

### Understanding Function Pointers

- A function pointer is just a function definition.
- If you see `fn` as a main or a standalone function, it is a function pointer.
- An error message about an unexpected function pointer can be confusing if you don't know what a function pointer is.

### Strategies for Handling Errors

#### Building Up Rapid Types

- The speaker has built up the rapid type around all of their types.
- There is a lot of bureaucracy involved with these rapper types.
- A third-party crate called "Jane" has made it much simpler to create these rapper types.

#### Using Trait Objects

- There are some gotchas with trait objects that can be resolved with another crate.
- The speaker demonstrates why Rust does what it does and provides options for either doing things yourself or
  leveraging third-party crates.

#### Unwrapping Errors

- Unwrapping errors is not really a strategy for dealing with errors; it causes panics in runtime code.
- Sometimes crashing when there's an error is what we want, but this should be used sparingly.

### Mixing Options and Results

- Mixing options and results semantically can be difficult because they are different. Be mindful of doing so.

## Error Conditions and Conversions

Section Overview: In this section, the speaker discusses error conditions and conversions between Option and Result
types in Rust.

### Error Conditions

- An empty Option is not an error condition but rather an expected thing that might happen.
- Unexpected errors can occur due to memory corruption or device failure when retrieving data from disk.
- Mixing up Option and Result types is often a sign that something needs to change in the code.

### Conversions

- There are methods available in the standard library for converting between Option and Result types.
- Applying these conversions in your own code may indicate that you have an Option that should have started as a Result
  or vice versa.

## Using Options and Results Together

Section Overview: In this section, the speaker discusses using Options and Results together in Rust.

### Example Scenario

- The speaker creates an example scenario where a function returns an Option type while another function returns a
  Result type.
- When trying to return both values together, there is no available type for combining them.
- The speaker suggests rethinking the design of the functions to avoid mixing Option and Result types.

## Converting Option to Result

Section Overview: In this section, the speaker discusses how to convert an option to a result in Rust.

### Converting Option i32 to Result

- To convert an option i32 to a result, we can use the question mark operator.
- We can also match on the option and return either an Ok or Err variant of the result.
- The orphan rule in Rust requires us to bring things into local scope when converting between types.

### Understanding Rust's Orphan Rule

- Rust's orphan rule requires that we make unambiguous decisions when writing import blocks that overlap.
- The compiler does not like ambiguity and wants to force the programmer to be able to make clear decisions about what
  they actually want to do.

## Defining Traits for Arbitrary Types

Section Overview: In this section, the speaker discusses how to define traits for arbitrary types and the limitations of
using conversion mechanisms like `?` operator with options and results.

### Wrapping Option in a New Type

- To use conversion mechanisms like `?` operator with options and results, we need to define a new type by wrapping
  option in it.
- This is because the end result, option, and error are defined outside of this particular crate.
- We cannot tinker with any of that since it's already decided what it wants to do.

### Limitations of Using Conversion Mechanisms

- The reason why using conversion mechanisms like `?` operator with options and results is failing is that we're pulling
  these from third parties or all from the standard library.
- We can't tinker with any of that since it's already decided what it wants to do.
- This problem isn't seen in real-world use cases because normally you wouldn't need to use the question mark operator
  with options and results defined in standard libraries.

## Conclusion

Section Overview: In this section, the speaker concludes the session by apologizing for not being able to get a little
thing working but assures viewers that this isn't really a problem encountered in real-life work use cases.

- The speaker enjoyed hanging out and hopes viewers have enjoyed watching him get unstuck from trying to compose
  multiple error types together.
- The speaker apologizes for not being able to get a little thing working but assures viewers that this isn't really a
  problem encountered in real-life work use cases.

## Generated by Video Highlight

https://videohighlight.com/video/summary/K_NO5wJHFys