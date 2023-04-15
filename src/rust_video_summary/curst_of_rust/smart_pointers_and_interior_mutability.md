# Smart Pointers and Interior Mutability

<!--ts-->


<!-- Created by https://github.com/ekalinin/github-markdown-toc -->
<!-- Added by: runner, at: Sat Apr 15 09:30:00 UTC 2023 -->

<!--te-->

## Introduction

Section Overview: In this section, the speaker introduces the topic of smart pointers and interior mutability in Rust.
They discuss some of the types that are commonly used in Rust, such as arc, RC, refs, l mutex L, DRF, as ref traits,
Baro trait, cow and sized.

- The speaker also provides information on where to find recordings of their streams and how to get announcements for
  upcoming streams.
- They introduce a new sub-channel on the Rust Asian Station Discord server called Rotation Station which is intended to
  be a community podcast for Rust.

## Overview of Smart Pointers

Section Overview: In this section, the speaker discusses some of the common smart pointer types in Rust.

### Common Smart Pointer Types

- The speaker mentions several common smart pointer types in Rust: arc, RC, refs, l mutex L, DRF and as ref traits.
- They explain that these types are pervasive in Rust and it's important to have knowledge about them.
- The speaker also mentions other types like cow and sized if they have time to cover them.

## Implementation of Smart Pointers

Section Overview: In this section, the speaker talks about implementing some of the common smart pointer types
themselves.

### Implementing Smart Pointer Types

- The speaker explains that they will be implementing RC ref sell and sell.
- They mention that they may also discuss Arc Mutex Ref Borrow Cow Bull CFD if there is time.
- The implementation will help viewers understand intermediate concepts in Rust.

## Resources for Learning More About Rust

Section Overview: In this section, the speaker provides resources for learning more about Rust.

### Resources for Learning More About Rust

- The speaker mentions that all recordings are posted on YouTube after each stream.
- They encourage viewers to follow them on Twitter for announcements and input on episodes.
- The speaker also mentions that they have longer programming videos available on their YouTube channel.

## Introduction to Cell

Section Overview: In this section, the speaker introduces the Rust standard library module called Cell.

### Introduction to Cell

- The speaker explains that the Cell module is a shareable mutable container.
- They mention that this might sound weird in Rust because of its shared reference and exclusive reference concepts.
- However, the module allows for interior mutability in a controlled fashion under certain constraints.

## Starting with Sell

Section Overview: In this section, the speaker talks about starting with Sell since it has the least amount of
quirkiness among the smart pointer types.

### Starting with Sell

- The speaker explains that they will start with Sell since it's less quirky than other smart pointer types.
- They mention that there are various container types in Rust's standard library that allow for interior mutability.
- These containers are often referred to as shareable mutable containers or interior immutability.

## Interior Mutability and Cell

Section Overview: In this section, the speaker discusses interior mutability and exclusive or shared references. They
then introduce the concept of a cell in Rust.

### Introduction to Cell

- A cell provides interior mutability in Rust.
- You can create a new cell with a value of some type T.
- The set method allows you to change the value inside the cell using an immutable reference.
- The swap method lets you swap values between two cells.
- The replace method replaces the value inside the cell with a new one.
- The into_inner method consumes self, assuming ownership of the cell.

### Restrictions of Cell Types

- Different types of cells have different restrictions on what can be stored inside them and how they can be used.
- As you move towards mutex, there is more freedom to store whatever you want but also more overhead involved in making
  it work.
- Box does not provide interior mutability. If you have a shared reference to a box, you cannot mutate its contents.
- There is no way to tell externally from a type whether it has interior mutability.

### Safety Features of Cell

- With Cell, there is no way to get a reference to what's inside the cell itself. This means that if no one else has a
  pointer to the value stored in the cell, changing it is safe.
- Cell does not implement sync. This means that if you have a reference to a cell, you cannot give away that reference
  to another thread.

## [#](t=0:10:59s) Why can't we borrow Mewtwo more than once for RC?

Section Overview: In this section, the speaker explains why it is not possible to borrow Mewtwo more than once for RC.

### Borrowing and Exclusive Reference

- [](t=0:10:59s) You cannot use an exclusive reference in general with a cell.
- If you have an exclusive reference to the cell, you can get an exclusive reference to the value inside.
- At that point, you cannot get or change the value.

### Benefits of Using Cell

- [](t=0:11:19s) The benefit of using cell is that you can have multiple shared references to a thing.
- Cell is usually used with something like RC where you want the cell to be stored in multiple places or pointers to
  it'd be stored in multiple places like in some data structure like imagine a graph where some of the things might
  share a value right then you might have multiple references to a thing.

### Usage of Cell

- [](t=0:12:02s) Cell should usually just be used for small copy types.
- You can only get the value out of a cell either if you have a mutable reference to it or if the value is copy.

## [#](t=0:13:18s) Implementing Sell Ourselves

Section Overview: In this section, the speaker demonstrates how to implement sell ourselves.

### Basic API

- We need a pub struct called "cell" which holds a T.
- We will implement:
    - A new function which takes a value of type T and returns self. This gives us a cell that contains the given value.
    - A set function which takes an immutable reference to self and a value T. It sets self.value equal to value but
      currently will not work because we are trying to assign to self.value which is behind a shared reference and so we
      can't modify it.
    - A get function which returns a T. This is like the basic API we are going for.

### Unsafe Cell

- [](t=0:15:06s) At the core of almost all of these types to provide interior mutability is a special cell type called
  unsafe cell.
- We will use cell unsafe cell so that the value here is an unsafe cell. That's the only way that we can actually mutate
  something through a shared reference.
- Cell is usually used for smaller values like numbers or flags that need to be mutated from multiple different places.

## Introduction to Unsafe Cell

Section Overview: In this section, the speaker introduces the concept of unsafe cell and explains how it can be used to
dereference a raw pointer.

### Using Unsafe Cell

- The value of unsafe cell is just unsafe sell new values.
- Trying to dereference a raw pointer is currently incorrect because the compiler doesn't know if anyone else is
  mutating that value.
- Writing "unsafe" here tells the compiler that we have checked that no one else is currently mutating this value.
  However, this code will be rejected if two threads try to write to a value at the same time.

### Implementing Not Sync for Cell T

- We need to implement not sync for cell T and tell the compiler that you can never share a cell across threads.
- Unsafe cell itself is not sync, which means that cell is not sync. Therefore, this unsafe code will be rejected.

## Recap of Unsafe Cell

Section Overview: In this section, the speaker summarizes what was covered in the previous section about unsafe cells.

### Summary of Unsafe Cell

- The cell type allows you to modify a value through a shared reference because no other threads have a reference to it.
- Get returns a copy of the value stored inside so even if we change the value, we don't have to invalidate any
  references because there are no references outside.

## [#](t=0:24:13s) Cell and Sync

Section Overview: In this section, the speaker discusses how to use cell and sync in Rust.

### Using Cell to Get a Reference

- Use `Cell` to get a reference out.
- Get a reference to the first thing inside the vector.
- Do not allow this code because once you call set that vector is gone as a first should be invalidated.

[](t=0:24:13s)

### Disallowing Unsafe Implementation of Sync

- Never give out a reference which means a set is always safe.
- Disallow unsafe implementation of sync by getting not returning a reference but only return to copy.

[](t=0:25:06s)

### Demonstrating Broken Code with Multiple Threads

- It's hard to write tests that fail when there are multiple threads.
- Have one thread that tries to set the whole value, and one that sets it differently.
- Wait for both threads to finish and then print out the value that ends up being stored in there.

[](t=0:30:00s)

## The Problem with Interleaving Threads

Section Overview: In this section, the speaker discusses the problem of interleaving threads and how it can lead to
corrupted arrays.

### Interleaving Threads

- When two threads are modifying the same bit of memory at the same time, there is no guarantee that they won't step on
  each other.
- If one thread writes its value and then goes to sleep while another thread runs for a while and then goes to sleep,
  we'll see an interleaving of values.
- This can result in a corrupted array that contains some values that were never set by either thread.
- The underlying memory system being fast enough can prevent this from happening in practice, but it's still a potential
  issue.

### Demonstrating the Problem

- By running a test where two threads increment a shared value 100,000 times each, we can demonstrate how interleaving
  threads can cause lost modifications.
- Because both threads are modifying the same value at the same time, some modifications end up being lost.
- This results in an incorrect final value for the shared variable.

## Non-Copy Types and Trait Bounds

Section Overview: In this section, the speaker discusses why non-copy types don't require trait bounds and why putting
trait bounds only where they're needed is more idiomatic in Rust.

### Copy Trait Requirement

- A question is asked about why non-copy types are allowed if only the get method requires copy types.
- The speaker explains that generally only putting trait bounds where they're needed is more idiomatic in Rust.
- Putting trait bounds on every type that contains a cell would result in extraneous bounds all over the place.

### Constrained Trait Bounds

- By putting trait bounds only where they're needed (i.e. on methods like get), callers only have to worry about those
  constraints when they're actually using those methods.
- This is usually the most constrained space and makes for cleaner code.

## Unsafe Cell and Ref Cell

Section Overview: In this section, the speaker explains what Unsafe Cell and Ref Cell are in Rust programming language.

### Unsafe Cell

- Unsafe cell is a special type in Rust that allows you to mutate data even if it's shared.
- You cannot cast a shared reference into an exclusive reference without going through unsafe cell because the Rust
  compiler optimizes your code and how it interacts with LLVM.

### Ref Cell

- Ref cell is a type of immutable memory location with dynamically checked borrow rules.
- It lets you check at runtime whether anyone else is mutating this value, which is useful for traversing graphs or
  trees where there might be cycles.
- The ref cell has a method called borrow which takes a shared reference to self and gives you an option reference to T.
  It also has a borrow mute method that initially makes both methods return None.

## [#](t=0:46:55s) RefCell and Cell

Section Overview: In this section, the speaker explains how to use RefCell and Cell in Rust.

### Borrowing with RefCell and Cell

- [](t=0:47:24s) The borrow and borrow_mut methods are not suitable for this case.
- [](t=0:48:02s) If the state is unshared, we can give out a value. Otherwise, we cannot.
- [](t=0:48:49s) If it's exclusively borrowed out, it's not okay to give out a shared reference.
- [](t=0:49:28s) We need to set self.state to be exclusive if we give out an exclusive reference.

### Using Cell instead of RefCell

- [](t=0:50:14s) Modifying ref state here is not thread-safe.
- [](t=0:51:29s) We can make this a cell because it gives us exactly what we need.
- [](t=0:52:18s) Mutex is basically a thread-safe version of RefCell.

### Safety Argument

- [](t=0:53:05s) No exclusive references have been given out since state would be exclusive or shared when using
  something like Rayon.

## Ref Type

Section Overview: In this section, the speaker discusses the implementation of a ref type and a ref mute type to solve
the problem of shared and exclusive references.

### Implementation of Ref Type

- A ref type is created with a lifetime that points to the ref cell.
- The ref cell contains a reference to the inner value.
- When a ref is dropped, the reference count is decremented.
- If there are no shared references left, then it becomes unshared.

### Deref Trait

- The Deref trait allows for automatic dereferencing into an inner type.
- Given a reference to self, it returns a reference to the target type T.
- This allows calling any method that requires a reference of T on it.

### Safety Argument

- The safety argument is updated to say that the ref is only created if no exclusive references have been given out.
- Dereferencing into a shared reference is fine since no exclusive references are given out.

## RefCell and Rc

Section Overview: In this section, the speaker explains RefCell and Rc in Rust programming language.

### RefCell

- RefCell allows for mutable data to be shared between multiple owners.
- RefMut is only created if no other references have been given out once it is given out state is set to exclusive so no
  future references are given out so we have an exclusive lease on the inner value so dereferencing is fine mutable or
  immutable EDD referencing is fine.
- Safety here is see safety 4d refute.
- It's common practice to write safety comments for every unsafe use.

### RC

- RC provides shared ownership of a value of type T allocated in the heap.
- Shared references in rust disallow mutation by default and RC is no exception.
- RC never provides mutability all it does is allow you to have multiple shared references to a thing and only
  deallocate it when the last one goes away.
- RC is not thread safe, but even on a single thread, this can be useful often again in the context of data structures
  or things like graphs.

## RC and Smart Pointers

Section Overview: In this section, the speaker explains the difference between weak and strong pointers. They then
introduce RC (Reference Counted) smart pointers and explain how they work.

### Weak vs Strong Pointers

- A weak pointer will not prevent an object from being deleted, whereas a strong pointer will.
- Weak smart pointers need to be upgraded to a real pointer before use, but this upgrade can fail.

### Introduction to RC Smart Pointers

- An RC is a pointer to some type T that is stored on the heap.
- The value needs to be stored on the heap because if multiple functions in the code reference it, it cannot be on the
  stack of any given function.
- The reference count has to be in the value that is shared amongst all copies of the RC.
- An RC inner holds both the value and reference count.

### Implementing Clone for T

- Cloning an RC increases its reference count without copying its inner value.
- DRF (Dereference), implemented for RCT, works similarly for RC by returning a raw pointer to T.
- This implementation requires unsafe blocks because Rust's compiler does not know whether a pointer is still valid.

## Rust Reference Types

Section Overview: In this section, the speaker discusses reference types in Rust and the semantics that must be followed
when using them.

### Reference Types

- Ref Mew T, Star Mew T, Star Consti So Star Mute and Star Const are not references but raw pointers.
- Ampersand symbol means a shared reference. Ampersand mute means an exclusive reference.
- The star versions of these like star constants star mute do not have guarantees. If you have a raw pointer, the only
  thing you can really do to it is use an unsafe block to dereference it and turn it into a reference.
- The difference between star constant and star mute is fuzzy. A star mute is usually something that you might be able
  to mutate something you might have an exclusive exclusive reference to whereas the star constant is intended to
  signify that you will never mutate this.

## Box in Rust

Section Overview: In this section, the speaker discusses what box provides for us in Rust.

### Box Provides Heap Allocation

- Box provides heap allocation which lets us go from our Zener which would otherwise be on the stack to a pointer that
  is on the heap which is what we store here.
- For clone here we're going to increase the reference count but here we have the same problem as we did for ref cell
  right which is we have a shared reference to self but we need to mutate something inside of it and so here lo and
  behold the problem is the answer is the same thing that we've done before it's our friend cell.

## Unsafe Keyword in Rust

Section Overview: In this section, the speaker discusses the unsafe keyword in Rust and its meaning.

### Unsafe Keyword

- The unsafe keyword is a little weird because really what it means is I have checked that the stuff inside the brackets
  is safe. It's like I as the programmer certified that this is safe so it's not really unsafe.
- It's like in some sense saying that I acknowledge that this code seems unsafe but it's actually safe so I agree with
  you it's a little bit of a weird keyword name.

## Smart Pointers in Rust

Section Overview: In this section, the speaker discusses smart pointers in Rust and how to deallocate them.

### Deallocating Smart Pointers

- When an RC goes away we need to make sure that when the last RC goes away then we actually deallocate otherwise there
  will be a memory leak.
- We are going to check with the countess if the count is one we are being dropped for after us there will be no RCS and
  no references to tea otherwise there are other references there are other RCS so don't drop the box.

## Understanding Variance in Rust

Section Overview: In this section, the speaker explains variance in Rust and how it relates to star-mutant-star-const.
They also introduce non-null as a way to optimize code.

### Variance in Rust

- Variance is one of the primary differences between star mutant star const.
- Non-null is used for optimization purposes because it allows the compiler to know that a pointer cannot be null.
- Option non-null can use the null pointer to represent none without any overhead.

### Using Non-Null

- The standard library has a neat thing called non-null that we can use instead of using star mute.
- We give it a storm utage from box from raw and use it to get back this star mutti which is what we need for into raw.

## Using Unsafe Pointers in Rust

Section Overview: In this section, the speaker explains how to use unsafe pointers in Rust and why they are necessary.

### Using Unsafe RF Method on Non Null

- We can use the unsafe RF method on non-null instead of having this star ampersand star thing.
- This is obviously unsafe, but we know that we're keeping the reference count correctly.

### Why Unsafe Pointers are Necessary

- The compiler doesn't know that we have the last pointer and therefore that it's safe to turn this back into a box and
  drop it.
- We know because we know that we're keeping the reference count correctly.

## Lifetime Guarantees in Rust

Section Overview: In this section, the speaker explains lifetime guarantees in Rust and why you cannot store mutable
references outside their scope.

### Why You Cannot Store Mutable References Outside Their Scope

- If you tried to stick the mutable reference you got back from mute somewhere and then dropped the ref mute and then
  tried to use this mutable reference again, the compiler would say no that's not allowed.
- You're trying to use this mutable reference after the lifetime it's tied to has already expired because the refuge has
  gone away.

### Difference Between Mutable Pointer and Mutable Reference

- A mutable pointer is just a pointer with certain semantics and we call it star mute.
- It does not carry the additional implication that it's exclusive which is what allows you to mutate through things.

## Understanding Memory in Rust

Section Overview: In this section, the speaker explains how memory works in Rust and why a cell is necessary even if
there is a mutable pointer.

### Why a Cell is Necessary Even if There is a Mutable Pointer

- We have a mutable pointer but it's not safe for us to mutate through it.
- This is the difference between a mutable pointer and a mutable reference.

# Inner Dog Ref Count

Section Overview: In this section, the speaker explains why it's important to add a marker in Rust when a type owns
another type. They also discuss how accessing something through a pointer that has just been deallocated can cause
issues.

## Importance of Marker in Rust

- It's important to add a marker in Rust when a type owns another type.
- If someone later comes along and writes code that accesses something through the pointer that was just deallocated,
  the compiler won't warn them that this isn't okay.
- This is because they won't know that this is the case if there isn't a marker present.

## Drop Check in Rust

- The speaker introduces drop check in Rust and explains why it's important to implement RC properly.
- Drop check is used by Rust to ensure that types are dropped in the correct order.
- If types are not dropped in the correct order, it can lead to errors and crashes.

## Example of Drop Check Issue

- The speaker provides an example of how drop check can cause issues if not implemented properly.
- They explain how dropping foo calls a method on the string through its drop implementation, but since drop is
  implicit, it can lead to problems if types are not dropped in the correct order.

# Understanding Drop Check

Section Overview: In this section, the speaker goes into more detail about drop check and how it works. They provide
examples of how incorrect use of drop check can lead to errors and crashes.

## How Drop Check Works

- When functions or any type gets dropped, Rust assumes that every use of that type is a test of use for all fields
  contained within it.
- This means that even if drop is not explicitly written, Rust will treat it as if it's accessing every field of the
  type.
- If types are dropped in the wrong order, Rust will catch this as a problem and throw an error.

## Example of Drop Check Issue

- The speaker provides another example of how incorrect use of drop check can lead to errors.
- They explain how dropping foo before T can cause issues because foo holds a reference to T.

# Understanding Phantom Data in Rust

Section Overview: This section explains the concept of phantom data in Rust and how it is used to ensure that the
dropping of an RC is checked at compile time.

## Phantom Data

- Phantom data tells Rust to treat a type as though there is one even though there is only a pointer to it.
- It ensures that when we drop an RC, we treat it as dropping one of these types.
- The marker is needed when T is not static, but we want to allow any T here.
- The wrapper internally in the standard library was added to guard against someone accidentally writing an
  implementation for RC in ER.

## Question Mark Sized Types

- Question mark sized types are used to opt-out of the requirement that every generic argument must be sized.
- Coerce incised trait deals with some of the restrictions of why it's hard for you to implement RC Foley yourself if
  you want to support dynamically sized types.

## Benefits of Using Rust Over C

- Writing convoluted code like this rarely happens in your own code.
- In Rust, these problems are caught at compile time rather than run time.

# RefCell, Mutex and Arc

Section Overview: This section covers the difference between exclamation marks sized and question mark sized. It also
explains how synchronous versions of RefCell, Mutex and Arc work.

## Exclamation Marks Sized vs Question Mark Sized

- Exclamation mark sized means not sized.
- Question mark size means it does not have to be sized.
- Default is that everything has a size bound.
- Opt out of that bound by using exclamation marks or question marks.

## Synchronous Versions of RefCell, Mutex and Arc

- Strategies written so far don't quite work right in the cell case if you have multiple threads to can mutate at the
  same time there just is no equivalent of cell because even though you're not giving out references to things having
  two threads modify the same type at the same value at the same time it's just not okay so actually is no thread-safe
  version of cell refs l is a little interesting so in the ref cell we wrote right you have borrow and borrow mute and
  they return options you could totally implement a thread-safe version of ref cell one that uses an atomic counter
  instead of cell for these numbers.
- Thread-safe version of ref cell uses an atomic counter instead of a cell for these numbers. CPU has built-in
  instructions that can increment and decrement counters in a thread safe way.
- Multi-threaded or synchronized version of RefCell is usually our W lock.
- Reader/writer lock (R/W lock), which is one type in sync, is basically a ref cell where counters are kept using
  Atomics so they are thread-safe but also borrow and borrow mute which in the reader/writer lock are called read and
  write they don't return an option instead they always return the ref for the ref mute but what they do is they block
  the current thread if the borrow can't succeed yet so they block the current thread until the conditions are met.
- Mutex is a simplified version of RefCell where there's only borrow mute as you don't need to keep all these extra
  counts for how many readers or how many shared references there are it's just either it some other thread has a
  reference to it or some of the threads is not and it similarly has blocking behavior where when you call lock on a
  mutex it will block until there are no other references to the inner value and at that point you're given that
  reference and it similarly has a guard the same way ref cell does.
- Arc (Atomic Reference Count) is pretty much exactly the same as our C except that it uses these thread safe operations
  these atomic CPU Atomics for managing the reference count rather than a cell. It cannot be sent because if I sent an
  RC to some other thread and that other thread dropped the RC and I dropped an RC at the same time both of us would try
  to use cell to decrement count but that's obviously not okay because cell is not thread safe. Non-null is also not
  send by default.

# RC vs Art

Section Overview: This section discusses why one might prefer RC over Art.

## Cost of Using Atomics

- RC is cheaper than Art.
- Atomics are more expensive in terms of CPU cycles and coordination overhead between cores.
- Non-thread safe versions are preferred because they have lower overhead.

## Asynchronous Mutexes

- Async Std, Tokyo, Futures crate, and Futures Intrusive crate all have asynchronous mutexes.

# The Cow Type

Section Overview: This section covers the Cow type and its implementation.

## Copy-On-Write

- The Cow type is an enum that is either owned or borrowed.
- If a Cow of T contains a reference to a T, it passes access through. If it owns the thing it contains, it gives you a
  reference to that.
- If you want to modify the value inside of a copy-on-write when it's just holding a reference, you can't modify it
  because it's shared.

## Benefits of Cow Type

- Cow allows for modification only when necessary by cloning the value and turning it into the owned version.
- It's useful when most of the time you don't need a copy because you're only going to read but sometimes you need to
  modify it.

# Example: Escape Function with Cow Type

Section Overview: This section provides an example use case for the Cow type in an escape function.

## Escaping Function with String Input

- An escape function takes a string and returns the string.
- Cow is useful in this case because if you don't have to modify it, you just pass it through. Only if you do have to
  change something do you clone and mutate it.

# From Utf-8 Lossy

Section Overview: This section discusses why from_utf8_lossy returns Cow but other UTF variants don't.

## From Utf-8 Lossy

- The reason from_utf8_lossy returns Cow is that if the given byte string is completely valid utf-8, it can just pass it
  straight through.

# Cow Type and Smart Pointers

Section Overview: This section covers the cow type and smart pointers in Rust.

## Cow Type

- The cow type allows you to avoid allocation if you don't need to modify.
- Other from UTA types usually allocate regardless, but with the cow type, there is no reason to do so.

## Smart Pointers

- Rust has several smart pointer types.
- Cell is for non-thread safe non-reference interior mutability.
- Ref cell is for dynamic interior mutability.
- RC is for dynamically shared references where you don't know how many references are going to be or when the inner
  value will be dropped at runtime.
- There are also thread-safe versions of these types called synchronized versions.
- Cow is not really a smart pointer but kind of a copy-on-write pointer that upgrades when needed.

# Recap and Next Steps

Section Overview: This section provides a recap of what was covered in the previous section and discusses what will be
covered next.

## Recap

- The previous section covered the cow type and smart pointers in Rust.

## Next Steps

- The next stream may cover trade objects, borrow trait, or trade delegation.

## Generated by Video Highlight
https://videohighlight.com/video/summary/8O0Nt9qY_vo