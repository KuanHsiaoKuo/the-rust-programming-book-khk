# Subtyping and Variance

<!--ts-->


<!-- Created by https://github.com/ekalinin/github-markdown-toc -->
<!-- Added by: runner, at: Wed Apr 19 08:50:50 UTC 2023 -->

<!--te-->

## Introduction

Section Overview: In this stream, the speaker will cover subtyping and variance in Rust. They will refer to official
Rust language references and educational code files to explain these concepts.

## Understanding Subtyping and Variance

Section Overview: The speaker explains that subtyping and variance are niche topics that can be hard to understand. They
recommend reading through the Rust reference and Rust nomicon for more information.

- The Rust reference is not great for understanding subtyping and variance.
- The Rust nomicon has a chapter on subtyping and variance that is very helpful.
- There is an educational code file available that provides a thorough example of how lifetime variance works in
  practice.

## Implementing Stir Talk Function

Section Overview: The speaker will implement the stir talk function from C++ and NC, which takes a string and delimiter
as input, returns the string up to the next delimiter, then changes the original string by removing the prefix returned.

- This function is similar to strip prefix in the Rust standard library but uses a delimiter instead of a string.
- The function mutates the string in place so it can be called repeatedly.
- Implementing this function straightforwardly runs into issues with variance, making it difficult to use.

## Challenges with Variance

Section Overview: The speaker acknowledges that they have limited experience with variance but will attempt to explain
its importance in this context. They encourage viewers to ask questions if they don't understand why something matters
or why an error occurs.

- Variance can be confusing due to technical terminology and lifetimes involved.
- Viewers are encouraged to ask questions if they don't understand something.

## Introduction to Stir Talk

Section Overview: In this section, the speaker introduces the concept of stir talk and explains its basic signature.
They also discuss how stir talk modifies a pointer to a string.

### Basic Signature of Stir Talk

- Stir talk takes a string and a delimiter as input.
- It takes a mutable reference to a string because it modifies the pointer to the string.
- The function returns everything up to the delimiter.

### Example Usage of Stir Talk

- To use stir talk, pass in a mutable reference to a variable holding the string and delimiter.
- The function will modify the pointer so that it points after the delimiter and return everything before it.

### Implementation of Stir Talk

- The implementation is not complicated. Find the first occurrence of the delimiter in s using `find()`.
- Split s at that location into prefix and suffix.
- Set s to be suffix and return prefix.
- If there is no occurrence of delimiter, set s to empty string, have prefix be entirety of s, and return prefix.

## Understanding How Stir Talk Works

Section Overview: In this section, we dive deeper into how stir talk works by discussing its internals. We also mention
that we will test our implementation later on.

### Internal Mechanics of Stir Talk

- The function changes its input argument in place; it becomes an in-out argument.
- It finds the limiter, returns what comes before it, and sets input argument to what follows it.

### Testing Our Implementation

We will test our implementation later on.

## [#](t=0:12:17s) String Slicing and UTF-8

Section Overview: In this section, the speaker discusses how to slice strings in Rust and how it works with UTF-8
characters.

### Slicing Strings

- [](t=0:12:17s) The length of the delimiter is important when slicing strings.
- [](t=0:12:30s) The function should work with UTF-8 strings as well since it finds the start of the delimiter and
  slices the string up to that point.
- [](t=0:12:51s) After finding the delimiter, we skip past it and return everything after it.

## [#](t=0:13:01s) Borrowing Mutable References

Section Overview: This section covers borrowing mutable references in Rust and how they can cause issues.

### Testing stir_talk Function

- [](t=0:13:23s) We test our function by passing a mutable string pointer into `stir_talk` with a whitespace delimiter.
  We assign the return value to be "hello".
- [](t=0:13:30s) We assert that "hello" is equal to "hello" and then we assert that x is equal to "world".

### Compiler Error Message

- [](t=0:13:58s) When running `cargo check`, everything runs fine. However, when running `cargo test`, it fails due to
  an error message about borrowing mutable references.
- [](t=0:14:26s) The issue arises because hello has the same lifetime as the mutable reference given in, so as long as
  hello exists, x continues to be immutably borrowed.

## [#](t=0:15:s19) Variance Issues

Section Overview: This section covers variance issues in Rust.

### Extra Scope Added

- [](t=0:15:13s) We add an extra scope around the code to drop hello so that the mutable reference to x should no longer
  be in use.
- [](t=0:15:19s) However, when running `cargo test`, it still fails with the same error message about borrowing mutable
  references.

### Simplified Example

- [](t=0:16:27s) We simplify the code by having `stir_talk` return a static string instead of a mutable one.
- [](t=0:17:00s) Even with this change, it still fails to compile due to variance issues.

## Understanding Static Borrowing

Section Overview: In this section, the speaker explains how the compiler sees a static borrow and why it needs to be
static. They also explore how the compiler can shorten the lifetime of a mutable borrow.

### Compiler's Perspective on Static Borrowing

- The compiler sees a static borrow and assumes that tick a is static.
- If tick a is static, then this must be static, and if that's static, then this mutable borrow has to be a static
  borrow.
- This mutable borrow of x is only valid for the lifetime of x until drop. Therefore, it needs to last until the end of
  the program. However, x is a stack variable so that won't work.

### Shortening Lifetime of Mutable Borrow

- The mutable borrow of x needs to be able to last until the end of the program but since x is not global or heap
  allocated it cannot do so.
- The compiler knows that if you have a static reference, you can use it in place of anything with shorter lifetime
  reference.
- If you have a string with a static lifetime, it's valid in any context that takes non-static lifetime because the
  static lifetime is always longer than any other lifetime.

### Exploring Variance

- The reason why we can provide tick-static-t instead of take-a-tick-t when taking t as an argument is due to variance
  specifically covariance.
- If we have two strings where one has a non-static lifetime and another has a static lifetime, we can still assign them
  because any string with a static lifetime is valid in any context that takes non-static lifetime.
- The compiler knows that it can shorten the borrow of x in some cases, but not in others. This is what we will explore
  next.

### Making X Static

- If we don't have an assertion, the compiler can pretend that the mutable borrow has the same lifetime as x until it's
  dropped.
- When we add an assertion, this is now trying to immutably access x before x is dropped and won't compile again because
  x needs to be static.

## Why Does This Example Work?

Section Overview: In this section, the speaker explains why a specific example works in Rust.

### Subtyping

- The type of `y` is some lifetime tick `a`, but we're assigning something that is a tick static into something that's a
  ticket.
- A subtype is when some type `t` is at least as useful as some type `u`.
- Static is a subtype of any tick `a`.
- You can use subtyping for things that aren't lifetimes in Rust currently variants mostly just affects lifetimes.

### Variance

- Most things are covariant.
- If you have some function foo that takes a t and doesn't return anything, you can use the same argument for if you
  have some x of type t and you want to assign to it.
- Here's what we're going to do: let's say that foo takes a tick `a` stir. You're allowed to provide any type that is a
  subtype of the arguments in this case right so I can call foo with some tick `a` stir but I can also call foo with a
  static stirrer static is a subtype of `a`.

## Covariance

Section Overview: In this section, the speaker explains covariance and how it works in Rust.

### Not Every Type Is Covariant

- There are three types of variance: covariance, contravariance, and invariance.
- Animal is not a subtype of cat because cats can do things that animals can't.

## Covariance and Contravariance

Section Overview: This section discusses covariance and contravariance in Rust, specifically in relation to functions
that take arguments of different types.

### Covariant Types

- A type is covariant if it can be replaced with a subtype.
- In Rust, most types are covariant.
- Examples of covariant types include `&'a T` and `Box<T>`.

### Contravariant Types

- A type is contravariant if it can be replaced with a supertype.
- In Rust, the only contravariant type is `fn(T)`.
- An example of a contravariant function argument is a function that takes an argument of type `fn(&'a str)`.

### Example: Function Arguments

- If a function expects an argument of type `fn(&'a str)` but receives an argument of type `fn(&'static str)`, this will
  not compile because the provided function requires a longer lifetime than what was expected.
- However, if the function expects an argument of type `fn(&'static str)` but receives an argument of
  type `fn(&'a str)`, this will compile because the provided function requires a shorter lifetime than what was
  expected.

## [#](t=0:39:55) Rust Lifetimes

Section Overview: This section covers the concepts of covariance, contravariance, and invariance in Rust lifetimes.

### Covariance and Contravariance

- [](t=0:39:55) Covariant types are more generally applicable than their subtypes.
- [](t=0:40:19) In Rust, function argument types are the only place where we have contravariant types.
- [](t=0:41:03) A static lifetime is a subtype of any other lifetime ticket.
- [](t=0:41:08) A function with a tick a t argument is a subtype of a static t.
- [](t=0:41:28) The subtyping relationship between these two types is flipped, which is what contravariance means.

### Invariance

- [](t=0:41:57) Invariance refers to cases where neither covariance nor contravariance applies.
- [](t=0:42:18) Mutable references to a type T are covariant in the tick a but invariant in the T.
- [](t=0:42:42) If mutable references were covariant, it would be possible to assign an x of type static string
  reference into foo that takes mutable reference to some string pointer. However, this would lead to issues when trying
  to print x since it's pointing into stack local memory that has since been dropped.

### Cell Static

- [](t=0:43.11) The concept of cell static is introduced as an example for explaining why mutable references cannot be
  covariant.

Overall, this section provides an overview of how covariance and contravariance work in Rust lifetimes and explains why
there are cases where neither applies (invariance). It also introduces the concept of cell static as an example for
explaining why mutable references cannot be covariant.

## Invariance for Mutable References

Section Overview: This section explains why mutable references are invariant in their argument type.

### Why Mutable References are Invariant

- Mutable references are invariant in their argument type.
- Invariance means that you must provide something that is exactly the same as what was specified. You cannot provide
  something more useful or less useful.
- The compiler needs to reconcile the provided arguments with the signature of foo, but it cannot make static be equal
  to take a or take a equal to static because mutable references are invariant in their thing that they reference.
- If mutable references were not invariant, you could downgrade a mutable reference to something less useful and end up
  with something that thinks it's a more useful type but it's actually a less useful type.

## Covariance and Invariance for Immutable References

Section Overview: This section explains why immutable references are covariant in their lifetime but invariant in the t.

### Covariance and Invariance for Immutable References

- Immutable reference to t is covariant in its lifetime but invariant in the t because if we had covariance, we could
  have an immutable reference to b with some lifetime and then stick in something less useful.
- The lifetime of the mutable reference to b is from b until when b gets dropped.
- We can introduce a memory leak using box leak which allows us to get an immutable reference with any lifetime but
  specifically with a static lifetime.
- Here, z is a mutable reference to y and we set z equals x which compiles fine because immutable references are
  covariant in their lifetime but invariant in the t.

## Introduction to Mutable References

Section Overview: In this section, the speaker introduces mutable references and explains how they are invariant in
their type but covariant in their lifetime.

### Mutable References

- Mutable references are introduced as a way to modify variables.
- They are invariant in their type but covariant in their lifetime.
- The compiler reconciles these two properties by allowing for subtyping between static and other lifetimes.
- This allows for the shortening of an immutable borrow's lifetime.

## Covariance and Invariance of Mutable References

Section Overview: In this section, the speaker explains why mutable references are covariant in their lifetime and
invariant in their type.

### Covariance and Invariance

- Mutable references are covariant in their lifetime because it is safe to shorten the lifetime of an immutable borrow.
- They are invariant in their type because it is not safe to assign a less useful type into what's behind the mutable
  reference.

## Understanding Lifetime Issues with Mutable References

Section Overview: In this section, the speaker discusses how mutable references can cause issues with lifetimes.

### Lifetime Issues

- The speaker goes back to a previous example where there was an issue with lifetimes.
- The compiler tells us that x does not live long enough because it needs to figure out how long the mutable borrow will
  be.
- We cannot shorten the lifetime of what's behind the mutable reference due to its invariance, so we must make it static
  instead.

## Introduction

Section Overview: In this section, the speaker explains that they will be discussing why a certain code does not compile
and how to fix it. They emphasize the importance of understanding why the code doesn't work before jumping into a
solution.

## Adding an Additional Lifetime

Section Overview: The speaker introduces an additional lifetime to fix a code that previously did not compile.

- The new signature for `stir_talk` now has two lifetimes: one for mutable borrow and one for the string being pointed
  to.
- The compiler can now choose these two lifetimes independently, allowing us to choose one as static and one as
  non-static.
- With this new signature, the previously non-compiling code now compiles successfully.

## Testing the Fixed Code

Section Overview: The speaker tests the fixed code with an earlier example to ensure that it works properly.

- The earlier example now compiles successfully with the fixed code.
- An unnecessary line is removed from the earlier example.
- The test passes, indicating that our implementation was correct.

## Explanation of Lifetime Assumptions

Section Overview: The speaker explains why a tick bee bound is not needed in this case.

- Rust assumes that any generic lifetime argument implies at least for as long as the duration of the function. This
  assumption is all we need for our mutable reference.
- Tick b does not depend on tick a; we are only returning a string.

## Borrow Checker Rules

Section Overview: This section covers the borrow checker rules in Rust.

### Mutable Reference Lifetime

- Modifying a variable after a mutable borrow would cause an error.
- Non-lexical lifetimes (NLL) enabled in Rust 2015 edition, but before NLL, this code would not compile.

### Borrowing Rules

- You can have a mutable reference to something that lives longer than the mutable reference.
- You cannot have a tick `a` is longer than tick `b`.
- The compiler implicitly adds a bound that tick `b` must be shorter than or equal to tick `a`.

### Phantom Data

- Phantom data is used when you have a type that is generic over `T`, but doesn't directly contain it.
- A phantom data field is added to satisfy the Rust compiler's requirement for using unused type parameters.

## The Drop Check

Section Overview: This section discusses the drop check in Rust and how it determines whether generic types might be
dropped as well.

### The Drop Check

- When a type is dropped, Rust needs to know whether the generic types might be dropped as well.
- If a vector of `T`s is dropped, then when you drop the vector, you're also going to drop some `T`s.
- Rust needs to know this because dropping a reference that still exists is not okay if it's going to try to access the
  reference later on.
- However, if the vector is never accessed again after being dropped, then dropping the reference does not access the
  inner type on drop and is okay.
- The compiler knows this through something called the drop check which looks at whether the type contains a `T` and a
  phantom data `T`.
- Vec will only drop inner types if they implement drop themselves.

## Touch Drop

Section Overview: This section discusses touch drops in Rust and how they relate to variance.

### Touch Drop

- Touch drops are used for debugging purposes in Rust.
- They require that their type is format debug and we're going to impl drop for touch drop.
- If you make this a touch drop instead of just a reference, then this code no longer compiles because when z gets
  dropped right down here, we're going to call this implementation of touch drop which is going to access the inner type
  which means it's going to access this reference so dropping x here is not okay because by when z gets dropped it's
  going to try to access the reference but when we drop the vector dropping the reference does not access the inner type
  on drop.
- Dropping x here would cause an error because there's still a reference that lives down here.

## Understanding Phantom Data

Section Overview: In this section, the speaker talks about phantom data and its use in Rust programming. They explain
how it is used to communicate ownership and drop semantics.

### Phantom Data

- `self.0` is a tuple struct with one element.
- The compiler assumes that it's going to drop a `T`.
- Phantom data `T` communicates that you are owning a `T` and might drop one.
- Phantom data `Fnt` is a marker of contravariance.

### Invariance

- If you want your type to be invariant in `T`, you can use phantom data.
- The compiler concludes that the `T` in d serializer four must be invariant in T.

### Covariance

- Star const T is covariant.
- Unsafe cell has to be invariant.

## Raw Pointers vs. Phantom Data

Section Overview: In this section, the speaker compares raw pointers and phantom data, explaining why people prefer
using phantom data over raw pointers.

### Raw Pointers

- Once you introduce raw pointers, there are a couple of things you don't get like send and sync auto-implemented for
  your types assuming all the members are.

### Auto Implementation

- People prefer using phantom data over raw pointers because if you use this type and a phantom data, it would get
  auto-implemented for send and sync.

## Covariance and Invariance in Rust

Section Overview: This section discusses the importance of understanding covariance and invariance in Rust, particularly
when dealing with generic types.

### Understanding Covariance and Invariance

- It is important to consider whether a type is covariant or invariant when working with generic types.
- If a compiler concludes that a type is invariant over T, but you need it to be covariant, you must ensure that your
  code is not vulnerable to an "invariance attack."
- In unsafe code, if the safety of your code relies on a generic parameter being invariant but the compiler concludes
  it's covariant, you must ensure that your type is actually invariant.
- In order to mutate through a pointer, you can only use `&mut T`, `*mut T`, or `UnsafeCell<T>`. All of these are
  invariant.

### Syntax for Lifetime Bounds

- The syntax `for<'a>` specifies that a bound holds for any lifetime `'a'`.
- This syntax does not affect variance or subtyping.

### Mutating Through Star Const T

- You can technically mutate through `*const T` by casting it without undefined behavior. However, this should be
  avoided as it requires contorting yourself into using an immutable reference as a mutable one.

### Unique Types and Non-null Pointers

- Vectored types and box types contain unique pointers which are non-null. The nominal pointer type is covariant over T.
- If this covariance is incorrect for your use case, include phantom data in your type to provide invariants.

## Rust Lifetimes: Variance and Subtyping

Section Overview: In this video, the speaker covers variance and subtyping in Rust lifetimes. They start with a simple
function and show how adding lifetimes can cause issues. The speaker then explains how to fix these issues using
variance and subtyping.

### Understanding Lifetime Specifiers

- A simple function like `stir_talk` can become complicated when lifetime specifiers are added.
- Adding lifetime specifiers to every reference is not enough to solve the problem.
- Using tick marks (`'a`) on all references can lead to problems that are difficult to diagnose.

### Solving Lifetime Issues with Variance and Subtyping

- Adding a second lifetime specifier solves the issue of calling `stir_talk`.
- The solution involves using variance and subtyping.
- By the end of the video, viewers should have a better understanding of variance and subtyping in Rust lifetimes.

### Writing Explicit Code

- It's possible to write code without explicit lifetime specifiers by relying on Rust's automatic generation of unique
  lifetimes for each reference.
- However, it's recommended to be explicit about different lifetimes for clarity.

### Conclusion

- The speaker thanks viewers for joining them in this journey through Rust lifetimes.
- They acknowledge that explaining type inference can be challenging but hope that viewers landed in a good place.
- The next stream will likely involve live coding, possibly porting some code.

## Generated by Video Highlight

https://videohighlight.com/video/summary/iVYWDIW71jk