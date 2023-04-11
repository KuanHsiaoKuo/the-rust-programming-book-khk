# Dispatch and Fat Pointers

<!--ts-->
* [Dispatch and Fat Pointers](#dispatch-and-fat-pointers)
   * [Introduction](#introduction)
   * [Topics Covered](#topics-covered)
      * [Traits and Dispatch](#traits-and-dispatch)
      * [Size Trait](#size-trait)
      * [Wide or Fat Pointers](#wide-or-fat-pointers)
      * [V Tables Coherence](#v-tables-coherence)
      * [Monomorphization](#monomorphization)
   * [Rust Book](#rust-book)
   * [Monomorphization and Generics in Rust](#monomorphization-and-generics-in-rust)
      * [Generics in Rust](#generics-in-rust)
      * [Benefits of Monomorphization](#benefits-of-monomorphization)
      * [Downsides of Monomorphization](#downsides-of-monomorphization)
      * [Monomorphization and Binary Size](#monomorphization-and-binary-size)
      * [Debug Symbols and Stripping Binaries](#debug-symbols-and-stripping-binaries)
      * [Dynamic Libraries and Generics](#dynamic-libraries-and-generics)
      * [Implementing a Trait](#implementing-a-trait)
      * [Dispatching Methods](#dispatching-methods)
   * [Static Dispatch vs Dynamic Dispatch](#static-dispatch-vs-dynamic-dispatch)
      * [Static Dispatch](#static-dispatch)
      * [Dynamic Dispatch](#dynamic-dispatch)
   * [Understanding Trait Objects and the Sized Trait](#understanding-trait-objects-and-the-sized-trait)
      * [Trait Objects](#trait-objects)
      * [The Sized Trait](#the-sized-trait)
   * [Dynamic Dispatch and Object Safety](#dynamic-dispatch-and-object-safety)
      * [Dynamic Dispatch](#dynamic-dispatch-1)
      * [Object Safety](#object-safety)
   * [Implementing Traits for Trait Objects](#implementing-traits-for-trait-objects)
      * [Implementing Traits](#implementing-traits)
   * [Downcasting Trait Objects](#downcasting-trait-objects)
      * [Downcasting](#downcasting)
   * [Conclusion](#conclusion)
   * [Introduction to the Problem](#introduction-to-the-problem)
      * [The Problem with Din High](#the-problem-with-din-high)
   * [Understanding the Issue](#understanding-the-issue)
      * [Types of Different Sizes](#types-of-different-sizes)
   * [Implementing Sized Trait](#implementing-sized-trait)
      * [Auto Implementation of Sized Trait](#auto-implementation-of-sized-trait)
   * [Types That Are Not Sized](#types-that-are-not-sized)
      * [Traits That Are Not Sized](#traits-that-are-not-sized)
   * [Indirection through Sized Types](#indirection-through-sized-types)
      * [Making an Unsized Type Sized](#making-an-unsized-type-sized)
   * [Using Box to Make an Unsized Type Sized](#using-box-to-make-an-unsized-type-sized)
      * [Using Box](#using-box)
      * [Creating a Trait Object](#creating-a-trait-object)
      * [Passing References](#passing-references)
      * [Boxed Pointer Type](#boxed-pointer-type)
      * [Static vs Dynamic Dispatch](#static-vs-dynamic-dispatch)
      * [Virtual Dispatch Tables](#virtual-dispatch-tables)
      * [Conclusion](#conclusion-1)
   * [Dynamically Sized Types and V-Tables](#dynamically-sized-types-and-v-tables)
      * [Dynamically Sized Types](#dynamically-sized-types)
      * [V-Tables](#v-tables)
      * [Pointers to String References](#pointers-to-string-references)
      * [Vtables and Duplication](#vtables-and-duplication)
      * [Box Din](#box-din)
      * [Two Different Traits](#two-different-traits)
      * [Workaround](#workaround)
   * [Trait Objects and Associated Types](#trait-objects-and-associated-types)
      * [Combining Traits](#combining-traits)
      * [Object Safety](#object-safety-1)
      * [Associated Functions](#associated-functions)
   * [Opting Out of Trait Object Safety](#opting-out-of-trait-object-safety)
      * [Weird Function and Trait Objects](#weird-function-and-trait-objects)
      * [Disallowing Trait Objects](#disallowing-trait-objects)
      * [Associated Type Problem with Static Dispatch](#associated-type-problem-with-static-dispatch)
   * [Opting Out of Trait Object Safety with Static Dispatch](#opting-out-of-trait-object-safety-with-static-dispatch)
      * [Opting Out with Sized Requirements](#opting-out-with-sized-requirements)
   * [Implementing Traits for Concrete DSTs](#implementing-traits-for-concrete-dsts)
      * [Implementing Traits for Concrete DSTs](#implementing-traits-for-concrete-dsts-1)
   * [Type Erasure in Trait Objects](#type-erasure-in-trait-objects)
      * [Associated Types and Vtables](#associated-types-and-vtables)
      * [Finagling Around Associated Types](#finagling-around-associated-types)
   * [The FromIterator Trait and Trait Objects](#the-fromiterator-trait-and-trait-objects)
      * [The FromIterator Trait](#the-fromiterator-trait)
   * [Extending with a Single Bool Add True](#extending-with-a-single-bool-add-true)
      * [Extending with a Single Bool Add True](#extending-with-a-single-bool-add-true-1)
   * [Could Rusty Add a Monomorphized Version of Extend?](#could-rusty-add-a-monomorphized-version-of-extend)
      * [Possibility and Problems](#possibility-and-problems)
   * [Object Safety in Rust](#object-safety-in-rust)
      * [Traits and Object Safety](#traits-and-object-safety)
      * [Iterator Trait Example](#iterator-trait-example)
      * [Receiver Requirements](#receiver-requirements)
   * [Object Safe Traits](#object-safe-traits)
      * [Dispatchable Functions](#dispatchable-functions)
      * [Non-Dispatchable Functions](#non-dispatchable-functions)
      * [Iterator Last](#iterator-last)
   * [Trait Objects and Drop Function](#trait-objects-and-drop-function)
      * [Drop Function](#drop-function)
   * [Unsized Types and Fat Pointers](#unsized-types-and-fat-pointers)
      * [Unsized Types](#unsized-types)
      * [Making Unsized Types Sized](#making-unsized-types-sized)
      * [Manipulating Fat Pointers](#manipulating-fat-pointers)
   * [Understanding Waker and Raw Waker](#understanding-waker-and-raw-waker)
      * [Structure of Waker and Raw Waker](#structure-of-waker-and-raw-waker)
      * [Manually Constructed V Table](#manually-constructed-v-table)
   * [Questions on References, Metadata Types, Dynamically Sized Types, Box U8 vs Vec U8, Din Fn vs Fn](#questions-on-references-metadata-types-dynamically-sized-types-box-u8-vs-vec-u8-din-fn-vs-fn)
      * [Reference to U8](#reference-to-u8)
      * [Metadata Types](#metadata-types)
      * [Dynamically Sized Types](#dynamically-sized-types-1)
      * [Box U8 vs Vec U8](#box-u8-vs-vec-u8)
      * [Din Fn vs Fn](#din-fn-vs-fn)
   * [Static Dispatch vs Dynamic Dispatch](#static-dispatch-vs-dynamic-dispatch-1)
      * [Calling Functions with Closures](#calling-functions-with-closures)
      * [Using dyn Fn over impl Fn](#using-dyn-fn-over-impl-fn)
      * [Object Safety](#object-safety-2)
   * [Coherence](#coherence)
   * [Understanding V-Tables and Dynamic Dispatch](#understanding-v-tables-and-dynamic-dispatch)
      * [V-Tables and Trait Objects](#v-tables-and-trait-objects)
      * [Double D Reference](#double-d-reference)
      * [Comparing V-Tables](#comparing-v-tables)
   * [Working with Slices and Vec of Dins](#working-with-slices-and-vec-of-dins)
      * [Example Code](#example-code)
   * [Rust Traits and the Any Trait](#rust-traits-and-the-any-trait)
      * [Rust Traits](#rust-traits)
      * [The Any Trait](#the-any-trait)
   * [Conclusion](#conclusion-2)
   * [Generated by Video Highlight](#generated-by-video-highlight)

<!-- Created by https://github.com/ekalinin/github-markdown-toc -->
<!-- Added by: runner, at: Tue Apr 11 15:55:34 UTC 2023 -->

<!--te-->

## Introduction

Section Overview: In this video, the speaker covers a range of topics related to Rust programming language. The topics
include traits, dynamic and static dispatch, size trait, wide or fat pointers, v tables coherence and monomorphization.

## Topics Covered

Section Overview: This section covers the various topics that will be discussed in the video.

### Traits and Dispatch

- The video covers traits and dynamic and static dispatch.
- Dynamic dispatch is when you have a pointer to an object on the heap.
- Static dispatch is when you have a concrete type.
- Traits are used to define shared behavior between types.

### Size Trait

- [](t=1m10s) The size trait is used to determine the size of a type at compile time.
- [](t=1m20s) It can be used for optimization purposes.

### Wide or Fat Pointers

- [](t=1m25s) Wide or fat pointers are pointers that contain additional information about their referent.
- [](t=1m35s) They are useful for dynamically sized types like slices.

### V Tables Coherence

- [](t=1m45s) V tables coherence refers to how Rust ensures that all implementations of a trait are consistent with each
  other.
- [](t=2m0s) It ensures that there is only one implementation of a trait for any given type.

### Monomorphization

- [](t=2m8s) Monomorphization is the process of turning generic code into specific code by filling in the concrete types
  that are used when compiled.
- [](t=2m23s) It enables more efficient code generation by allowing the compiler to generate specialized code for each
  concrete type.

## Rust Book

Section Overview: This section covers a chapter in the Rust book on generic types, traits, and lifetimes.

- [](t=1m23s) The Rust book has a chapter on generic types and traits.
- [](t=1m44s) The chapter discusses the performance of code using generics and introduces the concept of
  monomorphization.
- [](t=2m23s) Monomorphization is the process of turning generic code into specific code by filling in the concrete
  types that are used when compiled.

## Monomorphization and Generics in Rust

Section Overview: In this section, the speaker explains how generics work in Rust and the process of monomorphization.

### Generics in Rust

- Generics in Rust are turned by the compiler into multiple non-generic implementations through a process called
  monomorphization.
- Monomorphization generates a full copy of the entire struct and all its methods for each type that it is used by.
- The generation of copies happens on demand determined at compile time when code needs a function for a given type.
- Rust generates distinct copies of functions for each type, which makes it hard to ship people a binary that they can
  use as a library.

### Benefits of Monomorphization

- Monomorphization produces potentially much more efficient code because the compiler gets to see the concrete
  written-out code for particular types that are used, which lets it optimize better.
- For hashmap, it only generates methods you actually end up calling, so it amortizes the cost.

### Downsides of Monomorphization

- One downside is that your binary ends up being larger because you need to generate a copy of the type or function for
  every type that it's used with.
- Another downside is that rather than having one function called sterling, you end up with more code in the final
  binary.

## [#](t=0:12:09s) Rust Binary Size and Monomorphization

Section Overview: This section discusses the reasons why Rust binaries are larger than C ones, and how monomorphization
works.

### Monomorphization and Binary Size

- [](t=0:12:09s) The compiler generates copies of functions during monomorphization, but this is usually not too costly
  unless instantiated with a large number of types.
- [](t=0:13:23s) Monomorphization allows for inlining and optimizing based on the specific type used, which can result
  in larger binary sizes due to more static compilation.
- [](t=0:14:43s) The size of the standard library used also affects binary size, especially if using many generics from
  it.

### Debug Symbols and Stripping Binaries

- [](t=0:12:32s) Rust binaries statically compile more stuff, including debug symbols that can make the binary larger.
  Stripping the binary can help reduce its size.

## [#](t=0:15:00s) Generics in Dynamic Libraries

Section Overview: This section discusses how dynamic libraries handle generics in Rust.

### Dynamic Libraries and Generics

- [](t=0:15:34s) Dynamic libraries do not handle generics in Rust. This makes it challenging to distribute Rust
  libraries in binary form or use them for dynamically linked libraries.
- [](t=0:16:14s) Swift has been trying to stabilize something similar to dynamic linking for Rust, but it is still
  unclear how it will work.

## [#](t=0:16:44s) Dispatching Methods with Traits

Section Overview: This section explains how dispatching methods with traits works in Rust.

### Implementing a Trait

- [](t=0:16:49s) Defining a trait involves creating a method that can be implemented for different types.
- [](t=0:17:17s) Implementing the trait for a specific type involves defining the method for that type.

### Dispatching Methods

- [](t=0:17:26s) When generating code for a trait, the compiler determines the type and looks up which methods are
  available.
- [](t=0:18:00s) The dispatch process involves selecting the appropriate method based on the type used.

## Static Dispatch vs Dynamic Dispatch

Section Overview: This section discusses the difference between static dispatch and dynamic dispatch.

### Static Dispatch

- When the compiler generates machine code, it knows the type of h and can call the method directly. This is known as
  static dispatch.
- The method is trivial to figure out where it lives because the compiler knows what the actual type is.

### Dynamic Dispatch

- If we don't want to generate multiple copies, we use dynamically sized types or generics.
- We can treat things that are different concrete types as the same type using trait objects.
- Dynamic dispatch allows us to treat different concrete types as if they were the same type.

## Understanding Trait Objects and the Sized Trait

Section Overview: In this section, the speaker discusses trait objects and the sized trait. They explain how trait
objects work and why we need to use them. They also discuss the sized trait and its importance in Rust.

### Trait Objects

- Bar is generic over one type which has to be either a stir reference or a string.
- When taking a slice or an iterator, there is nowhere to put h2 as there is only one iterator that only has items of
  one type.
- Type erasure is used to take a collection of things where we only care about implementing a specific trait.

### The Sized Trait

- The size trait has no methods; it's just a marker trait that requires types with constant sizes known at compile time.
- Every trait bound requires that the type is sized implicitly.
- The compiler needs to know how large an argument is so it can allocate space on the stack for it when calling
  functions.
- The size of an argument can be determined by knowing its concrete implementation.

## Dynamic Dispatch and Object Safety

Section Overview: In this section, the speaker discusses dynamic dispatch and object safety in Rust. They explain what
dynamic dispatch means, why it's important, and how it works in Rust. They also discuss object safety and what makes a
trait object safe or unsafe.

### Dynamic Dispatch

- Dynamic dispatch allows us to call methods on objects whose concrete types are unknown at compile time.
- Virtual method tables (VMTs), also known as vtables, are used to implement dynamic dispatch in Rust.
- VMTs contain pointers to the actual functions that will be called at runtime.

### Object Safety

- A trait is object-safe if it can be safely used as a trait object.
- A trait is not object-safe if it has associated types or self-referential methods.
- The compiler will generate an error if we try to use a non-object-safe trait as a trait object.

## Implementing Traits for Trait Objects

Section Overview: In this section, the speaker discusses how to implement traits for trait objects. They explain what it
means to implement a trait for a type and how this differs from implementing a trait for a trait object. They also
discuss how to use the `dyn` keyword when working with trait objects.

### Implementing Traits

- To implement a trait for a type, we simply define the methods of the trait on that type.
- To implement a trait for a trait object, we need to use dynamic dispatch and vtables.
- We can use the `dyn` keyword when working with trait objects to indicate that we want dynamic dispatch.

## Downcasting Trait Objects

Section Overview: In this section, the speaker discusses downcasting and how it works with Rust's type system. They
explain what downcasting means and why it's useful. They also discuss how to downcast using Rust's built-in `as`
operator.

### Downcasting

- Downcasting allows us to convert from one type of pointer (e.g., `&T`) to another (e.g., `&U`) when `T` is a supertype
  of `U`.
- Downcasting can be useful when working with trait objects, as it allows us to access methods that are specific to the
  concrete type of the object.
- We can use Rust's built-in `as` operator to downcast trait objects.

## Conclusion

Section Overview: In this section, the speaker concludes their discussion on trait objects in Rust. They summarize the
key points covered in the video and provide some final thoughts.

- Trait objects allow us to work with collections of objects that implement a specific trait without knowing their
  concrete types at compile time.
- The sized trait is important because it allows the compiler to determine how much space an argument takes up on the
  stack.
- Dynamic dispatch and vtables are used to implement trait objects and allow us to call methods on unknown types at
  runtime.
- Object safety is

## Introduction to the Problem

Section Overview: In this section, the speaker introduces a problem with Rust's type system and explains why it is
important to understand.

### The Problem with Din High

- Rust allows for traits to be used as types.
- However, when using a trait as a type, there can be issues with size.
- This is because different types that implement the same trait may have different sizes.
- This can cause problems when trying to use these types in arrays or slices.

## Understanding the Issue

Section Overview: In this section, the speaker goes into more detail about why the issue occurs and how it affects code.

### Types of Different Sizes

- Strings and stirs are examples of types that have different sizes but implement the same trait.
- A slice is just a contiguous piece of memory where each chunk is the same size.
- If we don't know the size of each element in a slice, we can't guarantee that they are all the same size.
- This makes it impossible to generate code for functions that require knowing how much space to allocate for variables.

## Implementing Sized Trait

Section Overview: In this section, the speaker discusses how Rust implements sizing for types and what exceptions there
may be.

### Auto Implementation of Sized Trait

- The `Sized` trait is auto-implemented for any type that can implement it.
- For example, if you create a struct with no fields or add a string field to it, then that struct will be sized
  automatically.
- You never need to implement `Sized` yourself; it is only ever used as an auto-trait.

## Types That Are Not Sized

Section Overview: In this section, the speaker discusses some examples of traits that are not sized and what issues they
can cause.

### Traits That Are Not Sized

- Traits like `High` are not sized because different types that implement the trait may have different sizes.
- This makes it impossible to generate code for functions that require knowing how much space to allocate for variables.
- The compiler needs to know the size of a type in order to generate efficient code.

## Indirection through Sized Types

Section Overview: In this section, the speaker explains how to make a type that is not sized into a type that is always
sized by indirecting it through some type that is itself sized.

### Making an Unsized Type Sized

- An example of an unsized type is `dyn Trait` or `str`.
- To make an unsized type like `dyn Trait` or `str` into a sized type, we need to indirect it through some other type
  that is itself sized.
- Examples of such types include references (`&T`) and boxes (`Box<T>`).
- By indirection through these types, the size of the argument becomes known and relevant code can be generated.

## Using Box to Make an Unsized Type Sized

Section Overview: In this section, the speaker explains how to use `Box<T>` to make an unsized type like `str` into a
sized type.

### Using Box<T>

- A box has a known size because it's just a pointer.
- When you create a box initially, you allocate space for whatever you're going to put inside of it.
- The box is still of a known size even if what it points to might have an arbitrary size.
- You can create a box of something that itself is not sized by opting out of the auto-bound that gets added saying for
  box the T does not have to be sized.
- Once you do this, you can pass the boxed value as an argument without getting monomorphization errors.

## [#](t=0:44:44s) Passing References to Functions

Section Overview: In this section, the speaker discusses how to pass a reference to the stack and create a trait object.

### Creating a Trait Object

- [](t=0:45:59s) A trait object is an object that only has the property of representing a trait.
- [](t=0:46:35s) When you turn an object into a trait object, you erase all knowledge about what type it used to be.
- [](t=0:47:20s) You can technically go back through unsafe transmutes and any traits, but in general, you should ignore
  that.
- [](t=0:47:57s) When you create a boxed in astrif stir or any other trait object, you only retain the ability to use
  this trait.

### Passing References

- [](t=0:44:44s) You can give the function a reference to the stack by using `&`.
- [](t=0:45:30s) You can pass something that's on the stack by creating a pointer in direction here and then passing it
  to something that takes a reference din.

### Boxed Pointer Type

- [](t=0:46:44s) Box is kind of a pointer type.
- [](t=0:50.29s) The trick to constructing a trait object is that it carries extra information about the type pointed
  to.

## [#](t=0:48.17s) Generating Machine Code for Dynamic Dispatch

Section Overview: In this section, the speaker explains how dynamic dispatch works when generating machine code.

### Static vs Dynamic Dispatch

- [](t=0.49.24s) In the generic case, generating machine code is trivial because it's just like an assembly call.
- [](t=0:50.00s) In the dynamic case, we don't know what type it is, so we don't know whether to generate the address of
  this or something else entirely.

### Virtual Dispatch Tables

- [](t=0:50.29s) The trick to constructing a trait object is that it carries extra information about the type pointed
  to.
- [](t=0:50.55s) The reference of the box is not just one pointer wide; it carries a little bit of extra information
  about the type pointed to.
- [](t=0:51.45s) A virtual dispatch table (vtable) is an array of function pointers used in implementing polymorphism in
  object-oriented programming languages.

### Conclusion

- Dynamic dispatch and vtables are used when generating machine code for trait objects.
- Trait objects only retain the ability to use their underlying trait, erasing all other knowledge about their concrete
  types.

## Dynamically Sized Types and V-Tables

Section Overview: This section discusses dynamically sized types, which are only known at runtime, and v-tables, which
are little data structures that have pointers to each of the methods for a trait for a type.

### Dynamically Sized Types

- Slices and trait objects are two examples of dynamically sized types.
- At compile time, we don't know what type this is going to be. It's only at runtime that we'll know.
- The reason why this is determined at runtime is because some input can't be predicted at compile time.

### V-Tables

- A v-table or virtual dispatch table is a little data structure that has pointers to each of the methods for the trait
  for the type.
- When you have a trait object like `dyn Hi`, what actually gets stored in the reference is one pointer to the actual
  concrete implementing type and two a pointer to a v-table for the referenced trait.
- A different v-table ends up being constructed for each concrete type turned into a trait object.
- The second pointer in there will always be known statically because it's determined by the original construction of
  the trait object.

## [#](t=0:59:03s) Trait Objects

Section Overview: This section covers the basics of trait objects, including how they work and their limitations.

### Pointers to String References

- [](t=0:59:03s) Trait objects are pointers to string references, not pointers to actual strings.
- [](t=0:59:23s) The vtable struct for a trait object contains a member for each method of the corresponding trait that
  this is a trait object for.
- [](t=0:59:48s) The value in the vtable struct is the pointer to the implementation of that method for the concrete
  type.

### Vtables and Duplication

- [](t=1:00:12s) The vtable is generally statically constructed for the type and does not get constructed dynamically.
- [](t=1:00:32s) Identical vtables are not detected, and they are guaranteed to not be duplicates even if they contain
  the same code.
- [](t=1:01:00s) If you implement a trait for two different types, then the implementations are still distinct locations
  in the source code and resulting binary.

### Box Din

- [](t=1:01:09s) Box din is a thin pointer that points to a wide pointer that points to an object.
- [](t=1:01.33s) Pointer types are special because if they are trait objects, they have two pointers instead of one.
- [](t=1.02.05s) Nightly APIs exist for trade objects too.

## [#](t=1.02.14s) Limitations of Trait Objects

Section Overview: This section discusses some limitations of trait objects compared to fully generic types.

### Two Different Traits

- [](t=1.02.38s) A trade object cannot be created for two different traits because the vtables for each trait are
  contained in two different locations.
- [](t=1.04.11s) To create a trade object for two different traits, you would need a three-wide pointer: one to the
  data, one to the vtable for high, and one to the vtable for asref.

### Workaround

- [](t=1.04.55s) A workaround is to create a new trait that requires both traits with no methods of its own.
- [](t=1.05.13s) The rust compiler does not currently generate a vtable for the combination of multiple traits, but it
  is possible in theory.

## Trait Objects and Associated Types

Section Overview: In this section, the speaker discusses trait objects and associated types in Rust.

### Combining Traits

- When combining traits, adding a trait to a larger v table tells the developer that they can do something else instead.
- Developers can generate their own trait here.
- Only auto traits like Send and Sync are fine for marker traits because they don't have any methods.

### Object Safety

- If an associated type is added to a trait, it cannot be turned into a trait object directly. Instead, developers must
  specify the associated type when taking in the trait.
- For a trait to be object safe, it needs to allow building a v table to allow the call to be resolvable dynamically.

### Associated Functions

- The high trait cannot be made into an object because its associated function weird has no self parameter.
- A self parameter is necessary for dynamic dispatching of functions in Rust.

## Opting Out of Trait Object Safety

Section Overview: In this section, the speaker discusses how to opt out of trait object safety in Rust.

### Weird Function and Trait Objects

- The `weird` function cannot be called through a trait object because it requires a valid instance of the type that
  implements the trait.
- If there were multiple implementations of `weird`, it would not be clear which one to call through a trait object.
- To make sure that the trait remains object safe, you can opt out of including certain functions in the vtable by
  requiring that `self` is sized. This means that these functions cannot be called through a trait object.

### Disallowing Trait Objects

- You can also disallow using an entire trait as a trait object by specifying `where self is sized`.
- This is rare and usually only done for backwards compatibility reasons or if non-object safe methods may be added
  later.

### Associated Type Problem with Static Dispatch

- The associated type problem does not occur with static dispatch because concrete types are known at compile time.
- It would be possible to include the `weird` method in the vtable for a specific type if special syntax was added to
  specify which implementation to use.

## Opting Out of Trait Object Safety with Static Dispatch

Section Overview: In this section, the speaker discusses how opting out of trait object safety works with static
dispatch in Rust.

### Opting Out with Sized Requirements

- Requiring that `self` is sized allows you to opt out of including certain functions in the vtable and ensures that
  they cannot be called through a trait object.
- With static dispatch, it is possible to include specific implementations of functions like `weird` in the vtable for a
  given type.

## Implementing Traits for Concrete DSTs

Section Overview: In this section, the speaker discusses whether it is possible to implement a trait for concrete
dynamically sized types (DSTs), such as `Box<dyn AsRef<str>>`. They also explain why there are restrictions on
implementing traits for DSTs.

### Implementing Traits for Concrete DSTs

- It is possible to implement a trait for concrete DSTs if the `where Self: Sized` constraint is not present.
- The reason this should be possible is because when you wrap a type behind a pointer, it now has a size and is no
  longer dynamically sized.
- However, there are some restrictions on implementing traits for DSTs. For example, methods cannot be generic.
- If `Self: Sized` constraint is present, then it disallows implementing the trait for concrete DSTs.

## Type Erasure in Trait Objects

Section Overview: In this section, the speaker explains how type erasure works in trait objects and why associated types
pose a problem.

### Associated Types and Vtables

- The point of associated types is that only one exists for any given concrete type. However, the vtable does not know
  what the associated type should be because the type is erased.
- Trade objects are type erased. You don't get to keep information about the type it used to be.

### Finagling Around Associated Types

- There are some ways to finagle around this limitation. For example, the `Any` trait has a method that returns a
  descriptor of the type of the concrete type that it used to be.

## The `FromIterator` Trait and Trait Objects

Section Overview: In this section, the speaker discusses the `FromIterator` trait and how it poses a problem for trait
objects.

### The `FromIterator` Trait

- The `FromIterator` trait is implemented by types like `Vec`. It allows you to collect items from an iterator into a
  collection.
- The type of the trait is the type of the items of the iterator. However, the `FromIterator` type itself takes a
  generic parameter that represents the type of the iterator.
- This poses a problem for trait objects because they cannot have methods that are generic.

## Extending with a Single Bool Add True

Section Overview: In this section, the speaker explains how to extend an iterator of bools and why the trait extend
cannot be made into an object.

### Extending with a Single Bool Add True

- The function extends anything that can be extended with an iterator of bools and then tries to extend it with an
  iterator that yields a single bool by adding true.
- The implementation of extend is generic over the type of items in the iterator.
- The implementation of extend is generic over the type of the iterator, which leads to multiple copies of this method
  at compile time.
- There is no pointer to the appropriate implementation of extend, so there cannot be a v table for din extend.
  Therefore, din extend cannot exist.

## Could Rusty Add a Monomorphized Version of Extend?

Section Overview: In this section, the speaker discusses whether Rusty could add a monomorphized version of extend for
each t it's called with to each type that implements extend.

### Possibility and Problems

- It is tempting to add a monomorphized version of extend for each t it's called with to each type that implements
  extend.
- However, it is not always possible because crates that depend on another crate might call extend with even more types,
  leading to different v tables for din extends in different crates.
- This would result in lots of different v table implementations or lots of different v tables for din extends and make
  passing din extends from one crate to another impossible.

## Object Safety in Rust

Section Overview: This section discusses the requirements for a trait to be object safe and why some traits cannot be
made into objects.

### Traits and Object Safety

- A trait needs to have all of its methods include a receiver that includes self to be object safe.
- The trait cannot have a type that returns self, as this would make the return value not sized, which is required for
  generating code.
- Some traits have methods that are object safe but also have other methods that aren't. These traits can still be
  useful on their own without being made into objects.

### Iterator Trait Example

- The iterator trait has some methods that are object safe, such as next, but also has other methods like chain and
  enumerate that return self or are generic, making them non-object safe.
- To make the iterator trait object safe, certain methods like chain have where clauses specifying that self is sized,
  allowing them to be ignored when using the trait as an object. This allows for a mix of object-safe and
  non-object-safe methods in the same trait.

### Receiver Requirements

- The receiver for an object-safe method can include anything that includes self or &self/mut self. There are no
  additional restrictions beyond this requirement.

## Object Safe Traits

Section Overview: This section discusses object safe traits and their requirements.

### Dispatchable Functions

- All associative functions must either be dispatchable from a trait object or be explicitly non-dispatchable.
- Dispatchable functions require that they don't have type parameters so they're not generic.
- They have to be a method that does not use the self type right the concrete type except in the type of receiver.

### Non-Dispatchable Functions

- Non-dispatchable functions should library writers always consider adding where self is size to non-object safe methods
  just in case someone downstream wants to use it as a trait object.
- If you have a trait where it is useful even if you could only call the objective methods then it might make sense to
  opt-out for the other one so that the trait overall is objective.

### Iterator Last

- Iterator last has the size restriction because its receiver doesn't go behind a reference, it consumes self which
  means that it takes self and self is a din iterator which is not sized.
- Function arguments must be sized therefore last can't be called through a trade object.

## Trait Objects and Drop Function

Section Overview: This section discusses how drop function works with trait objects.

### Drop Function

- The v table for any trait object includes a pointer to the drop function for the concrete type because it's necessary.
- Every v table includes drop, which technically includes extra information such as size and alignment of the concrete
  type.
- For something like a box where you have to deallocate memory, that information is necessary to pass to the allocator.

## Unsized Types and Fat Pointers

Section Overview: In this section, the speaker discusses unsized types such as din trait, u8, and stir. They explain
that these types are not sized and cannot be used as function arguments or return types. The speaker also explains how
to make these types sized by placing them behind a reference or a pointer.

### Unsized Types

- Din trait, u8, and stir are examples of unsized types.
- When placed behind a reference or a pointer, unsized types become a tuple of a pointer to the data and a pointer to
  the v table.
- U8 becomes sized when placed behind a type that can mask its unsizedness such as box or raw pointers.

### Making Unsized Types Sized

- To make an unsized type sized, place it behind a type that can mask its unsizedness such as references or boxes.
- Box can be used to make an unsized type sized by turning it into a wide pointer where one part is the data and the
  other part is the length of the slice.
- Dynamically-sized types are special in Rust because they require knowledge about whether their pointers are wide or
  not.

### Manipulating Fat Pointers

- RFC 2580 adds generic APIs for manipulating metadata of fat pointers.
- A din trait becomes pointy where metadata is din metadata which contains information such as type size, alignment,
  drop-in-place pointer, and methods for implementing traits.
- Trait alias thin refers to any type that implements pointy where metadata is empty. It has methods for introspecting
  the metadata of a pointer.
- Waker trait in Rust's standard library is an example of dynamically constructing v tables at runtime.

## Understanding Waker and Raw Waker

Section Overview: This section explains the structure of a waker and raw waker, which is a manually constructed v table
in the standard library.

### Structure of Waker and Raw Waker

- A waker is a struct that has the methods wake, wake by ref will awaken from raw, and you can drop it and clone it.
- Inside of a waker is a raw waker. A raw waker if you look inside of it is a data pointer and a v table pointer so it
  really is dynamic dispatch but in sort of a hidden way.
- The v table is a raw waker v table and you construct by giving the function pointers for the clone method for the wake
  method for the wake by ref and for drop.

### Manually Constructed V Table

- It's basically a manually constructed v table that gives you dynamic dispatch through a type rather than a trait.
- This section concludes with no further information provided.

## Questions on References, Metadata Types, Dynamically Sized Types, Box U8 vs Vec U8, Din Fn vs Fn

Section Overview: This section answers various questions related to Rust programming language.

### Reference to U8

- A reference to u8 had length not start pointer or endpoint.

### Metadata Types

- RFC deprecates trait object stuff that existed nightly because this is replacement pointy trade thin metadata from raw
  parts where is the definition of oh it doesn't actually give.

### Dynamically Sized Types

- You can create your own dynamically sized types by adding fields to a struct. If it's the last field, then it's fine
  you can statically know the offset of f the offset of x and the offset of t.

### Box U8 vs Vec U8

- Box u8 is not the same thing as a vec u8. Evacuate can grow so evacuate first of all has um it's three three words it'
  s the pointer to the vector on the heap uh it's the length of the vector and it's the capacity of the vector.
- With a box u8, you can't do that this will never grow or shrink you can't push to it.

### Din Fn vs Fn

- A din fn is different from an fn because din fn is really a v table which means that it both has a sort of v table.
  This has to be a function; it can't be a closure.

## Static Dispatch vs Dynamic Dispatch

Section Overview: This section discusses the difference between static dispatch and dynamic dispatch.

### Calling Functions with Closures

- `fn main` can call `foo` with a closure, but not `bar`.
- The reason is that closures capture data from their environment, so when calling the closure, the address of the
  captured data needs to be passed in as well.
- For `bar`, only a function pointer is required, which means there's nowhere to pass in the captured data.

### Using `dyn Fn` over `impl Fn`

- When using `dyn Fn` over `impl Fn`, we can use closures with functions like `baz`.
- However, using `impl Fn` generates a copy of the function for each closure type passed in.
- Sometimes it's better to use `dyn Fn` to clean up your interface or make traits object-safe.

### Object Safety

- Making a trait take an `impl Fn` makes it not object-safe because there could be multiple implementations of that
  function.
- Using a boxed trait object (`Box<dyn Trait>`) instead of making it generic cleans up your interface and makes it
  easier to use.

## Coherence

Section Overview: This section briefly mentions coherence but will not go into detail about it.

Coherence is different enough from previous topics that it will require its own separate stream.

## Understanding V-Tables and Dynamic Dispatch

Section Overview: This section discusses how Rust generates a V-table for each trait object, and how there are many
V-tables for each din trait. It also explains that dynamic detection of which traits something implements is not
possible.

### V-Tables and Trait Objects

- Rust generates a V-table for each trait object.
- There are many V-tables for each din trait.
- Dynamic detection of which traits something implements is not possible.

### Double D Reference

- Calling a din function involves a double d reference.
- The first reference is for the V-table pointer, while the second one is for the actual function pointer within.

### Comparing V-Tables

- With the new pointer v table dynamic metadata RFC, it's possible to compare v tables by checking if s.v_table equals
  string as din asref stir dot v table.
- However, this approach may not be guaranteed since there could be different v tables in different compilation units.

## Working with Slices and Vec of Dins

Section Overview: This section covers working with slices and vec of dins in Rust.

### Example Code

- An example code was provided to demonstrate working with slices and vec of dins in Rust.
- To make the code sized correctly, you have to use Box or Arc for the inner type.

## Rust Traits and the Any Trait

Section Overview: In this section, the speaker talks about Rust traits and how they work. They also introduce the any
trait and explain its purpose.

### Rust Traits

- The speaker explains that Rust traits are similar to interfaces in other programming languages.
- They mention that there is an invariant in Rust that states that if two types implement the same trait, then they
  should have the same behavior for that trait. However, this invariant may not hold true in some cases.

### The Any Trait

- The speaker introduces the any trait, which is a super magical trait in Rust.
- They explain that any is just a trait with a function that returns a unique identifier for a type guaranteed by the
  compiler.
- Using type id on a trait object over any can give you a unique type identifier for that value, which can be used to
  downcast from din any to the concrete type.
- The speaker recommends reading the standard any module documentation to learn more about why this is safe.

## Conclusion

Section Overview: In this section, the speaker concludes their talk and thanks their audience.

- The speaker ends their talk after covering many topics related to Rust traits and introducing the any trait.

## Generated by Video Highlight

https://videohighlight.com/video/summary/xcygqF5LVmM