# Declarative Macros

<!--ts-->
* [Declarative Macros](#declarative-macros)
      * [Macro Rules](#macro-rules)
      * [Implementing the vec! Macro](#implementing-the-vec-macro)
   * [Introduction to Rust Macros](#introduction-to-rust-macros)
      * [Creating a Dedicated Library for Macros](#creating-a-dedicated-library-for-macros)
      * [Macro Rules and Delimiters](#macro-rules-and-delimiters)
      * [Using Cargo Expand](#using-cargo-expand)
   * [Understanding Rust Macros Continued](#understanding-rust-macros-continued)
      * [Defining Macro Arguments](#defining-macro-arguments)
      * [Using Cargo Check](#using-cargo-check)
      * [Optional Delimiters](#optional-delimiters)
      * [Writing Valid Syntax Patterns](#writing-valid-syntax-patterns)
      * [Using Cargo Expand](#using-cargo-expand-1)
      * [Syntactically Valid Rust Grammar](#syntactically-valid-rust-grammar)
      * [Hygienic Macros](#hygienic-macros)
      * [Creating a Macro](#creating-a-macro)
      * [Generating Code with Macros](#generating-code-with-macros)
      * [Repetition in Macros](#repetition-in-macros)
      * [Recursion in Macros](#recursion-in-macros)
      * [Summary](#summary)
   * [Macro Expansion](#macro-expansion)
      * [Defining a Simple Macro](#defining-a-simple-macro)
      * [Expanding Macros](#expanding-macros)
   * [Macro Rules in Rust](#macro-rules-in-rust)
      * [Procedural Macros vs. Macro Rules](#procedural-macros-vs-macro-rules)
      * [New Version of Macros](#new-version-of-macros)
      * [Delimiters in Instantiation](#delimiters-in-instantiation)
      * [Declarative Macros vs. Proc Macros](#declarative-macros-vs-proc-macros)
   * [Introspection in Rust Macros](#introspection-in-rust-macros)
      * [Type Information](#type-information)
      * [Identifiers Before Argument Block](#identifiers-before-argument-block)
   * [Introduction to Proc Macros](#introduction-to-proc-macros)
      * [What are Proc Macros?](#what-are-proc-macros)
      * [Special Features of Proc Macros](#special-features-of-proc-macros)
   * [Using Repetition in Patterns](#using-repetition-in-patterns)
      * [Limitations of Single Expression Pattern](#limitations-of-single-expression-pattern)
      * [Using Repetition in Patterns](#using-repetition-in-patterns-1)
      * [Using Repetition in Macro Replacements](#using-repetition-in-macro-replacements)
   * [Expansion of Patterns](#expansion-of-patterns)
      * [Rust Format Expansion](#rust-format-expansion)
      * [Handling Multiple Variables in a Single Repetition](#handling-multiple-variables-in-a-single-repetition)
   * [Dealing with Repeating Patterns](#dealing-with-repeating-patterns)
      * [Meta Variable X Repeats Zero Times](#meta-variable-x-repeats-zero-times)
   * [Macro Rules vs Proc Macro Plus](#macro-rules-vs-proc-macro-plus)
      * [Limitations of Macro Rules](#limitations-of-macro-rules)
      * [Dropping to Proc Macro Plus](#dropping-to-proc-macro-plus)
   * [Inspiration for Dollar Parentheses Pattern Language](#inspiration-for-dollar-parentheses-pattern-language)
      * [Origin of Dollar Parentheses](#origin-of-dollar-parentheses)
   * [Trailing Commas](#trailing-commas)
      * [Allowing Trailing Commas](#allowing-trailing-commas)
   * [Declarative Macros vs Proc Macros](#declarative-macros-vs-proc-macros-1)
      * [Defining Your Own Macros](#defining-your-own-macros)
      * [Example: Generating Trait Implementations](#example-generating-trait-implementations)
      * [Limitations of Declarative Macros](#limitations-of-declarative-macros)
   * [Macros in Rust](#macros-in-rust)
      * [Object-Oriented Programming with Macros](#object-oriented-programming-with-macros)
      * [Testing Invalid Expressions](#testing-invalid-expressions)
      * [Compiler Optimization](#compiler-optimization)
      * [Doc Tests](#doc-tests)
      * [Veck Macro Comparison](#veck-macro-comparison)
   * [Making the Star Instead of Plus](#making-the-star-instead-of-plus)
      * [Reducing Patterns](#reducing-patterns)
   * [Other Useful Crates](#other-useful-crates)
      * [Other Useful Crates](#other-useful-crates-1)
   * [More Efficient Implementation](#more-efficient-implementation)
      * [Reducing Reallocations](#reducing-reallocations)
      * [Pointer Increments](#pointer-increments)
   * [Veck Macro](#veck-macro)
      * [Veck Macro Implementation](#veck-macro-implementation)
   * [Power of Two Expansion](#power-of-two-expansion)
      * [Purely Power of Two Expansion](#purely-power-of-two-expansion)
   * [Hygienic Macros](#hygienic-macros-1)
      * [Caller-defined Modules](#caller-defined-modules)
   * [Trailing Commas in Expressions](#trailing-commas-in-expressions)
      * [Trailing Commas in Expressions](#trailing-commas-in-expressions-1)
   * [Defining a Macro to Avoid Repeating Elements](#defining-a-macro-to-avoid-repeating-elements)
      * [Using a Macro to Define an Expression](#using-a-macro-to-define-an-expression)
      * [Counting Trick for Arrays](#counting-trick-for-arrays)
   * [Counting Tokens in Rust Macros](#counting-tokens-in-rust-macros)
      * [Tricks for Counting Tokens](#tricks-for-counting-tokens)
      * [Using Procedural Macros](#using-procedural-macros)
      * [Predefined Count Macro](#predefined-count-macro)
      * [Testing Compile Time Length Verification](#testing-compile-time-length-verification)
      * [Hiding Nasty Patterns](#hiding-nasty-patterns)
   * [Using Map Lit Crate](#using-map-lit-crate)
      * [Practice with Map Lit Crate](#practice-with-map-lit-crate)
   * [Standard Library Version of Veck](#standard-library-version-of-veck)
      * [Explanation of Standard Library Version](#explanation-of-standard-library-version)
   * [Conclusion](#conclusion)
      * [Wrapping Up](#wrapping-up)
      * [Future Video Concepts](#future-video-concepts)
   * [Generated by Video Highlight](#generated-by-video-highlight)

<!-- Created by https://github.com/ekalinin/github-markdown-toc -->
<!-- Added by: runner, at: Sat Apr 22 15:51:27 UTC 2023 -->

<!--te-->

## [#](t=0:00:00s) Introduction to Declarative Macros

Section Overview: In this video, we will learn about declarative macros and how they work in Rust. We will focus on
macro rules and implement the `vec!` macro from the standard library.

### Macro Rules

- [](t=0:00:22s) Declarative macros are defined using macro rules.
- [](t=0:02:39s) Macro rules consist of a name followed by an exclamation point, followed by a bracket containing
  patterns that serve as arguments to the macro.
- [](t=0:03:20s) The patterns for macros are looser than those for functions and can include syntax types such as
  identifiers, expressions, items, blocks, type names or module paths.

### Implementing the `vec!` Macro

- [](t=0:01:49s) The `vec!` macro is used to declare a vector using either a vector literal syntax or by specifying an
  element that is cloneable along with its count.
- [](t=0:04:05s) To implement the `vec!` macro, we need to understand some handy patterns explained in "The Little Book
  of Rust Macros".
- [](t=0:04:29s) Section four of "The Little Book of Rust Macros" provides useful patterns for implementing macros.

Overall, this video provides an introduction to declarative macros and demonstrates how to implement them using macro
rules. We also learned about the `vec!` macro from the standard library and how it can be implemented using these
concepts.

## Introduction to Rust Macros

Section Overview: In this section, the speaker introduces Rust macros and explains why it is important to have a
dedicated library for them.

### Creating a Dedicated Library for Macros

- A dedicated library for macros makes it easier to develop in isolation.
- It is good practice to get into the habit of making libraries.

### Macro Rules and Delimiters

- Rust macros have optional delimiters that can be used by the caller.
- The input syntax for a macro must be syntactically valid rust program, but the output must be valid rust code.
- Every macro invocation is replaced by whatever input was given when working with macros.

### Using Cargo Expand

- Cargo expand is a handy crate that expands all macros with their definitions in the source code.

## Understanding Rust Macros Continued

Section Overview: In this section, the speaker continues explaining how Rust macros work and demonstrates how they can
be used.

### Defining Macro Arguments

- Patterns are patterns over rust syntax trees, not argument lists.
- You can introduce any syntax you want in a macro as long as it is syntactically valid rust program.

### Using Cargo Check

- Running cargo check on an empty macro expansion returns no errors because nothing is allowed to be in these locations.

### Optional Delimiters

- There isn't currently a way to specify which delimiter should be used when invoking a macro rule.

### Writing Valid Syntax Patterns

- The input syntax pattern must be syntactically valid rust program while the output must also be valid rust code
  because it is what gets compiled.

### Using Cargo Expand

- Cargo expand is a handy crate that expands all macros with their definitions in the source code.

## [#](t=0:11:19) Introduction to Rust Macros

Section Overview: In this section, the speaker introduces Rust macros and explains how they work. They discuss the
concept of syntactically valid Rust grammar and how it relates to macro rules.

### Syntactically Valid Rust Grammar

- [](t=0:11:32) The speaker explains that for a code to be syntactically valid, it must follow the rules of Rust
  grammar.
- [](t=0:12:16) The output is helpful in identifying identifiers, which can be chosen by the user.
- [](t=0:12:54) Identifiers defined inside a macro exist in a separate universe from everything outside the macro.

### Hygienic Macros

- [](t=0:14:46) The speaker explains that identifier names used inside macros are distinct from those outside of macros.
- [](t=0:15:34) Rust macros are hygienic because they cannot access things outside their own scope.

### Creating a Macro

- [](t=0:17:02) The speaker demonstrates how to create a simple macro using `macro_rules!`.
- [](t=0:17:50) They explain how to use `macro_export` to make the macro callable from other libraries.

## [#](t=0:20:00) Using Macros for Code Generation

Section Overview: In this section, the speaker discusses how macros can be used for code generation and demonstrates an
example of generating code with macros.

### Generating Code with Macros

- [](t=0:20:22) The speaker demonstrates an example of generating code with macros using `vec!`.
- [](t=0:21.30) They explain how this approach can simplify code and reduce errors.
- [](t=0.22.10) The limitations of using macros for code generation are discussed.

## [#](t=0:24:00) Advanced Macro Concepts

Section Overview: In this section, the speaker discusses advanced macro concepts such as repetition and recursion.

### Repetition in Macros

- [](t=0:24:20) The speaker explains how repetition can be used in macros to generate code.
- [](t=0:25:30) They demonstrate an example of using repetition to generate a function that calculates the sum of a list
  of numbers.

### Recursion in Macros

- [](t=0:28:10) The speaker explains how recursion can be used in macros to generate code.
- [](t=0:29.40) They demonstrate an example of using recursion to generate a function that calculates the factorial of a
  number.

## [#](t=0:32.50) Conclusion

Section Overview: In this section, the speaker concludes by summarizing the key points covered in the video and
providing resources for further learning.

### Summary

- [](t=0:33.10) The speaker summarizes the key points covered in the video, including syntactically valid Rust grammar,
  hygienic macros, generating code with macros, and advanced macro concepts.
- [](t=0:34.20) They provide resources for further learning about Rust macros.

Overall, this video provides a comprehensive introduction to Rust macros and demonstrates how they can be used for code
generation. The examples provided are clear and easy to follow, making it an excellent resource for anyone looking to
learn more about Rust macros.

## Macro Expansion

Section Overview: In this section, the speaker discusses macro expansion in Rust and provides examples of how to define
macros.

### Defining a Simple Macro

- The speaker defines a simple macro that produces a static value.
- The speaker explains that when passing identifiers to a macro, you are not passing ownership but rather access to the
  name.
- The speaker explains that what matters for ownership is what the code that the macro expands to does with that name.

### Expanding Macros

- The speaker demonstrates expanding a macro using `cargo expand`.
- The speaker shows an example of expanding a more complicated macro and explains why it doesn't work as expected.
- The speaker adds an argument to the macro and expands it correctly using curly brackets to create a block.

Overall, this section covers how to define and expand macros in Rust, including handling ownership issues and creating
blocks for more complex expansions.

## Macro Rules in Rust

Section Overview: This section discusses the implementation of macro rules in Rust and how it differs from normal
macros.

### Procedural Macros vs. Macro Rules

- Macro rules is not a normal macro, but rather a procedural macro.
- The syntax for macro rules does not allow for an identifier followed by curly brackets, unlike other macros.
- The output type of the macro defined using macro rules is expected to be an expression.

### New Version of Macros

- A new version of macros called "macros" is proposed to be more powerful than macro rules.
- It will have different guarantees about hygiene and work better with the module system and use statements.

### Delimiters in Instantiation

- The delimiter used in instantiation can be freely chosen by the user between square brackets, round brackets, and
  curly brackets.
- All delimiters are valid and have the same meaning.

### Declarative Macros vs. Proc Macros

- Declarative macros like macro rules do not let you write a Rust program, while proc macros do.
- Declarative macros are entirely declarative and only allow substitution between one valid Rust syntax tree to another.

## Introspection in Rust Macros

Section Overview: This section discusses introspection in Rust macros and what information they have access to.

### Type Information

- Declarative macros like macro rules do not have access to things like type information.
- Proc macros give you more access to introspection but still don't provide full introspection capabilities.

### Identifiers Before Argument Block

- Proc macros cannot take identifiers before the argument block.

## Introduction to Proc Macros

Section Overview: In this section, the speaker introduces proc macros and explains how they differ from declarative
macros.

### What are Proc Macros?

- Proc macros take Rust syntax streams as input and produce a different token stream to replace it with.
- They are more expressive but also more complicated to write than declarative macros.

### Special Features of Proc Macros

- With proc macros, you can add attributes or derives that you cannot do with declarative macros.

## Using Repetition in Patterns

Section Overview: In this section, the speaker explains how repetition works in patterns and how it can be used in macro
replacements.

### Limitations of Single Expression Pattern

- The pattern for accepting only one expression is limiting when dealing with multiple expressions separated by commas.
- This limitation causes an error when trying to run the code.

### Using Repetition in Patterns

- The patterns index allows for repetition by surrounding part of your pattern with dollar parentheses.
- You can give a delimiter followed by either star or plus to indicate zero or more repetitions or one or more
  repetitions respectively.
- This allows for matching comma-separated lists of expressions.

### Using Repetition in Macro Replacements

- Inside the replacement part of your macro, you can use the same syntax as the pattern to give repetitions
  corresponding to the pattern.
- This allows for scaling up your vector macro to work for an arbitrary number of elements and produce an appropriate
  vector.

## Expansion of Patterns

Section Overview: In this section, the speaker discusses how Rust format expands patterns and how it handles multiple
variables in a single repetition.

### Rust Format Expansion

- Rust format expands patterns multiple times when encountering the pattern of dollar parentheses inside of the
  expansion.
- It matches against each pattern and pulls out the variables every time.

### Handling Multiple Variables in a Single Repetition

- Using multiple variables in a single repetition is an error.
- If there are different repeating patterns with variables, they have to repeat the same number of times.

## Dealing with Repeating Patterns

Section Overview: In this section, the speaker talks about repeating patterns and how to deal with them.

### Meta Variable X Repeats Zero Times

- When using static strings, if there are no repeating patterns, Rust format will give an error message saying "meta
  variable X repeats zero times but element repeats one time."
- If there are different repeating patterns with variables, they have to repeat the same number of times.

## Macro Rules vs Proc Macro Plus

Section Overview: In this section, the speaker explains that for more elaborate things than what macro rules can define,
you need to drop to a proc macro plus.

### Limitations of Macro Rules

- The format macro is not macro rules.
- Macro rules are somewhat limited in what you can define.

### Dropping to Proc Macro Plus

- For some more elaborate things than what macro rules can define, you need to drop to a proc macro plus.

## Inspiration for Dollar Parentheses Pattern Language

Section Overview: In this section, the speaker discusses where the dollar parentheses pattern language came from.

### Origin of Dollar Parentheses

- The origin of the dollar parentheses pattern language is unknown.
- It is reminiscent of regular expressions where a parenthesis is a grouping and plus means one or more of the previous
  pattern.

## Trailing Commas

Section Overview: In this section, the speaker talks about trailing commas and how to allow them in Rust format.

### Allowing Trailing Commas

- When things wrap, you often want to have a trailing comma.
- The current pattern doesn't allow that because it expects a comma-separated list of things with an expression
  following each comma.
- To allow trailing commas, add a pattern that allows any number of commas after the normal pattern.

## Declarative Macros vs Proc Macros

Section Overview: This section discusses the differences between declarative macros and proc macros, and their
respective benefits.

### Defining Your Own Macros

- Declarative macros are limited in their ability to provide error messages.
- Proc macros offer better control over what went wrong and what errors are emitted.
- Macro rules can be used to generate repeated patterns quickly.

### Example: Generating Trait Implementations

- Macro rules can be used to generate trait implementations for multiple types quickly.
- This is useful when there are repeated patterns that need to be expressed.

### Limitations of Declarative Macros

- When using non-literal expressions, declarative macros may not work as expected due to substitution.
- Substitution can cause issues with more complicated expressions, leading to unexpected behavior.

## Macros in Rust

Section Overview: This section covers the use of macros in Rust, including macro rules and proc macros. It also
discusses the downsides of using proc macros.

### Object-Oriented Programming with Macros

- Macros are not recommended for object-oriented programming like inheritance.
- Macro rules can become an eyesore when they go beyond simple ones.
- Proc macros are a better option for more complicated macros but add an additional compilation step.

### Testing Invalid Expressions

- Rust assigns any error produced by a macro to the corresponding place in the macro input.
- A test that includes an invalid expression will not compile and will produce an error message pointing to the location
  of the error.

### Compiler Optimization

- The compiler is smart enough to recognize types that implement copy, such as integers, and will optimize them
  accordingly.
- The newly introduced syntax can be used to compact arbitrarily nested for-loops.

### Doc Tests

- Rust does not have a way to say that a unit test should not compile, but there is a crate called "compile fail" that
  allows you to write tests that are not supposed to compile.
- Doc tests can be used as a cheap way to get compiled fail tests.

### Veck Macro Comparison

- The Veck macro supports the pattern element expression semicolon and expression.

## Making the Star Instead of Plus

Section Overview: In this section, the speaker discusses how to make the star instead of plus and reduce the number of
patterns.

### Reducing Patterns

- The star can be used instead of plus to indicate zero or more repetitions.
- An allow statement is needed in case the input list is empty and we never push.
- This reduces the number of patterns and makes documentation easier to read.

## Other Useful Crates

Section Overview: In this section, the speaker mentions other useful crates for compile fail tests.

### Other Useful Crates

- Try build is a good crate to know for compile fail tests.
- Should panic is different from compiled failed macro export.
- Macro export is not always required but without it, you wouldn't be able to call this macro outside of this crate.

## More Efficient Implementation

Section Overview: In this section, the speaker discusses how to create a more efficient implementation by reducing
reallocations and pointer increments.

### Reducing Reallocations

- Creating an empty vector and then calling push a thousand twenty-four times will require many reallocations.
- Using capacity count can help allocate for a specific number of elements.
- Standard iterator repeat element take count can be used instead of push to avoid bounds checking for every iteration.

### Pointer Increments

- Push still requires pointer increments which can add up if creating a vector with many elements in a busy loop.
- Standard iterator repeat plus take does not implement exact size and can be improved.

## Veck Macro

Section Overview: This section discusses the Veck macro and its implementation.

### Veck Macro Implementation

- The Veck macro is a more sophisticated operation that requires special treatment.
- If an iterator is produced, it must be implemented as an exact size iterator.
- The macro does not have any trait bounds, so if something that isn't clone is used, the compiler will generate an
  error.
- The error output generated by the macro expansion points to where the type that didn't implement clone came from.

## Power of Two Expansion

Section Overview: This section discusses power of two expansion and its efficiency.

### Purely Power of Two Expansion

- The Veck macro does purely power of two expansion.
- It will do ten allocations before it even gets to 1024, making it inefficient.

## Hygienic Macros

Section Overview: This section discusses hygienic macros and how they can be affected by caller-defined modules.

### Caller-defined Modules

- When defining macros, you have to be careful because the caller might have modules that override you.
- Macros aren't entirely hygienic because they can't express certain things like crate or colon colon which are root
  level paths.
- If someone renames the crate STD then they deserve the problems they get.

## Trailing Commas in Expressions

Section Overview: This section discusses trailing commas in expressions and their effect on recursion limits.

### Trailing Commas in Expressions

- If there's not a trailing comma, then it will keep invoking this rule causing infinite recursion.
- The order matters when defining rules for expressions with no trailing commas.
  I'm sorry, but I cannot summarize the transcript without having access to it. Please provide me with the transcript so
  that I can create a summary for you.

## Defining a Macro to Avoid Repeating Elements

Section Overview: In this section, the speaker explains how to define a macro to avoid repeating elements in Rust.

### Using a Macro to Define an Expression

- The problem with repeating an element multiple times is that it can cause issues.
- Rust figures out how many times to repeat a block by looking at which variables are used inside of it.
- If we don't use any variables, Rust doesn't know how many times to repeat the block.
- We can define a substitute variant that takes an expression and returns unit instead of using repetition.
- By expanding the macro with substitution, Rust knows how many times to repeat the block because we're using element in
  there.

### Counting Trick for Arrays

- The type does not give the length of the array, so we need to use a counting trick.
- We call slice Len on this array turned into a slice by calling the ass ref trade.

## Counting Tokens in Rust Macros

Section Overview: In this section, the speaker discusses how to count tokens in Rust macros and the tricks involved in
doing so.

### Tricks for Counting Tokens

- You can use size of to count tokens, but it ends up allocating.
- There are other tricks like zero plus multiple replacements with one that can be used to count tokens. However, if you
  do enough of these, you will crash the compiler.
- You can also count the number of tokens by counting sub-patterns and batching them. This has been tested up to ten
  thousand tokens but cannot be used to produce a constant expression.
- There are versions that can produce a constant expression like extracting the enum counter to get the number of items.

### Using Procedural Macros

- When counting tokens gets too complicated, you can write it as a procedural macro instead.
- Although you may not necessarily want to use these tricks yourself, they tell you a lot about how macros work.

### Predefined Count Macro

- Currently, there is no predefined count macro because it's hard to know what exactly needs to be counted (e.g., number
  of tokens or characters in syntax tree).

### Testing Compile Time Length Verification

- It is possible to test and verify that length is known at compile time using const slices.

### Hiding Nasty Patterns

- To hide nasty patterns from documentation, you can make macros export but also doc hidden.

## Using Map Lit Crate

Section Overview: In this section, the speaker recommends using the Map Lit crate to practice creating maps.

### Practice with Map Lit Crate

- The Map Lit crate can be used to create maps.
- Expanding or extending the code to work for a hashmap is a good exercise.
- The syntax change required for working with a hashmap is fairly straightforward.

## Standard Library Version of Veck

Section Overview: In this section, the speaker explains how the standard library version of Veck works.

### Explanation of Standard Library Version

- The standard library version uses Box and creates an array on the heap.
- A boxed array is created on the heap and treated as a box slice instead of a boxed array.
- A sequence of things on the heap can be trivially constructed into a vector.

## Conclusion

Section Overview: In this section, the speaker wraps up and invites viewers to suggest additional concepts for future
videos.

### Wrapping Up

- Thanks everyone for watching!
- The standard library version allows them to get away with not doing all the tricks that were done in previous
  sections.

### Future Video Concepts

- Viewers are invited to suggest additional self-contained real code concepts.

## Generated by Video Highlight

https://videohighlight.com/video/summary/q6paRBbLgNw