# How to implement the IntoIterator trait

<!--ts-->
<!--te-->

## [#](t=0:00:00s) Introduction

Section Overview: The speaker introduces themselves and welcomes viewers to the stream. They explain that they will be
working on a Rust project and encourage viewers to ask questions throughout the stream.

## [#](t=0:00:56s) Creating a Struct

Section Overview: The speaker begins by creating a main function and a simple struct called "Tree". They then create
another struct called "Forest" which contains an array of trees. The speaker explains how to use macros in Rust and
demonstrates how to print out the contents of the forest.

- [](t=0:00:56s) Create a main function
- [](t=0:01:11s) Create a struct called "Tree"
- [](t=0:02:03s) Create a struct called "Forest" containing an array of trees
- [](t=0:04:23s) Use macro to print out contents of forest

## [#](t=0:05:24s) Iterating Through Structs

Section Overview: The speaker discusses how to iterate through structs in Rust, using the example of iterating through
trees in a forest. They demonstrate two methods for doing this, one using loops and one using iterators.

- [](t=0:05:24s) Discuss iterating through structs in Rust
- [](t=0:06:15s) Demonstrate loop method for iterating through trees in forest
- [](t=0:07:33s) Demonstrate iterator method for iterating through trees in forest

## [0:08:20s](t=500s) Read-Only References

Section Overview: The speaker explains the concept of borrowing and ownership, and recommends using read-only references
to values when possible.

### Benefits of Read-Only References

- Using read-only references allows for multiple read-only accesses to values without needing to modify anything.
- The reference operator (&) is used to create a read-only reference.

## [0:08:49s](t=529s) Delegating with Wrapper Types

Section Overview: The speaker discusses how wrapper types can be used for delegation, and provides an example of
implementing into iterator.

### Implementing Into Iterator

- Implementing into iterator allows your type to work with Rust's for loop syntax.
- Delegation can be achieved by implementing one method that delegates from the wrapper type into its own method.
- When implementing a trait for a type, it must be brought into local scope to avoid name clashes.

## [0:09:28s](t=568s) Motivation for Iterative Thing

Section Overview: The speaker provides motivation for why iterative thing is useful, and discusses the trait called "
into iterator".

### Motivation for Iterative Thing

- Implementing into iterator has benefits such as working with Rust's for loop syntax.
- A wrapper type can be used for delegation by implementing one method that delegates from the wrapper type into its own
  method.

## [0:10:20s](t=620s) Type Variables in Traits

Section Overview: The speaker discusses creating type variables inside traits, bringing types into local scope when
implementing traits, and encourages questions from viewers.

### Creating Type Variables in Traits

- Inside a trait, you can create type variables that are referenced inside the rest of the trait.
- When implementing a trait for a type, it must be brought into local scope to avoid name clashes.

### Encouraging Questions

- Viewers are encouraged to ask questions at any stage, and the speaker is able to respond in real-time.

## [0:11:47s](t=707s) Type Conversions with Into Trait

Section Overview: The speaker discusses the "into" trait and type conversions.

### The "Into" Trait

- When a trait starts with "into", it refers to type conversions.
- Implementing into iterator allows your type to return something that can turn itself into an iterator that returns the
  item or type defined previously.
- Capital self is the type variable of the type being referred to.

## [0:15:34](t=934s) Rust Iterators

Section Overview: In this section, the speaker discusses how Rust iterators work and the different types of iteration
available in Rust.

### Types of Iteration

- There are three types of iteration in Rust: `&`, `mut`, and owned.
    - The `&` type borrows a value without taking ownership.
    - The `mut` type allows for mutation of values.
    - The owned type takes ownership over the value and destroys it when it goes out of scope.

### Implementing an Iterator

- When implementing an iterator, it is important to understand which type of iteration is needed.
- The first type, `&`, can be implemented by implementing the iterator trait.
- If the first type is implemented, then the other two types are automatically implemented as well.

## [0:20:34](t=1234s) Bonus Content: Digging into Trait Implementation

Section Overview: In this section, the speaker discusses how to dig into trait implementation in Rust.

### Open Source Implementation

- One benefit of Rust being open source is that users can dig into trait implementation themselves.
- This can be useful when trying to understand what a certain type means or how to implement a specific trait.

### Double-ended Iterator Trait

- The double-ended iterator trait has few required methods because of Rust's generics capability.
- It has several provided methods that allow for easy implementation.

## Understanding Rust's Standard Library

Section Overview: In this section, the speaker is trying to understand how Rust's standard library works by looking at
the source code of a specific type called "lines".

### Investigating the "lines" Type

- The "lines" type is a wrapper around an internal collection that delegates to it.
- The speaker wants to find out what happens under the hood and what the internal collection is.
- After some searching, they find that the internal collection for "lines" is a map.
- They discover that Rust aggressively inlines and optimizes code, which allows for embedding other types without
  incurring runtime costs.

### Understanding the Map Function

- The speaker looks into the map function and discovers that it takes an iterator and a function as arguments.
- They assume that it takes characters and splits them into multiple lines.
- They note that poking around inside Rust's internals can be fun but also challenging.

### Exploring Further

- The speaker continues to explore Rust's standard library by looking at other types and functions.
- They enjoy digging into the source code and learning more about how things work.

## Finding the Rustling Source Code

Section Overview: In this section, the speaker navigates to Github and pulls up the Rustling source code. They locate a
specific function within the code and explain its purpose.

### Locating the Function

- The speaker goes to Github and pulls up the Rustling source code.
- They locate a function called `MiskaReading` within the code.
- The function is located in `lib core` which is not part of the standard library.

### Understanding `MiskaReading`

- The function is actually a struct that masquerades as a function using closures in Rust.
- It takes a string and returns a string after analyzing whether it's at the end of a line or not.
- This level of control is necessary because it's located in the very guts of Rust, where there may not be all tools
  available.

## Navigating Documentation for Arbitrary Traits

Section Overview: In this section, the speaker explains how they navigated documentation for arbitrary traits by finding
an implementation and digging into its source code.

### Finding Implementation

- The speaker was having trouble understanding documentation for an arbitrary trait.

- They scrolled down to find implementations at the bottom of said trait.

- Clicking on one of these types led them to dig into source code inside Lib core.

### Exploring Byte Slices

- While exploring Lib core, they found themselves looking at how byte slices implement splitting heuristics.

- This was another thing that they may as well explain while they still had an audience.

## Syntax of `main`

Section Overview: In this section, the speaker explains the syntax of `main` and how it works.

### Syntax of `main`

- The syntax for `main` is: `fn main() -> io::Result<()>`.
- This means that it returns a result type which can either be an error or a unit.
- The speaker notes that they feel like they're doing physics in front of an audience.

## [#](t=0:37:42s) Using Generic Types in Rust

Section Overview: In this section, the speaker explains how to use generic types in Rust and their implementation.

### Implementation of Generic Types

- [](t=0:37:42s) The speaker explains that generic types are used to implement all the implementation for us.
- [](t=0:38:21s) The speaker gives an example of how into is implemented for every type T where you implements the
  fromto type.
- [](t=0:39:16s) The speaker talks about converting float 64 and 32 from one another and whether it's safe or not.

## [#](t=0:40:21s) Binary Heap Implementation in Rust

Section Overview: In this section, the speaker discusses binary heap implementation in Rust.

### Implementing Binary Heap

- [](t=0:40:21s) The speaker talks about how binary heap implements from big t so from any couldn't a.
- [](t=0:41:17s) The speaker mentions that bang today we go actually so the implementation of this is super trivial.
- [](t=0:41:55s) The speaker concludes by saying that they enjoyed talking people through iteration and getting the most
  out of the standard library.

## Generated by Video Highlight

https://videohighlight.com/video/summary/OhwnbYshBIo
