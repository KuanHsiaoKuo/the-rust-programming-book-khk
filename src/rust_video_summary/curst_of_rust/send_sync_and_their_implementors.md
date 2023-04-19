# Send, Sync and their implementors

<!--ts-->


<!-- Created by https://github.com/ekalinin/github-markdown-toc -->
<!-- Added by: runner, at: Wed Apr 19 08:50:49 UTC 2023 -->

<!--te-->

## Introduction

Section Overview: In this section, the speaker introduces the topic of thread safety in Rust and explains why they chose
to focus on send and sync.

### Thread Safety in Rust

- Rust has two traits, send and sync, that represent thread safety at the type level.
- These concepts are baked into the type system so that you can check for thread safety just by type checking your
  program.
- The two traits are related but serve different purposes.

## What are Send and Sync?

Section Overview: In this section, the speaker explains what send and sync are and how they differ.

### Marker Traits

- Send and Sync are both marker traits with no methods.
- They only mark that a type meets a given property or guarantee about it but do not confer any additional behavior.
- All marker traits are specifically traits with no methods.

### Auto Traits

- Send and Sync are also auto traits which means that if all members of a type implement the trait, then the compiler
  will automatically implement it for you.
- For example, structs or enums implement Send if all types inside them also implement Send.

## Differences Between Send and Sync

Section Overview: In this section, the speaker discusses how send and sync differ from each other.

### Send Trait

- The Send trait indicates that it is safe to move a value of this type between threads.
- Types like integers, floats, strings etc. are all inherently send because they don't have any interior mutability (
  i.e., they don't contain references to mutable data).
- Types like Mutex<T> or Rc<T> require explicit implementation of Send because they contain interior mutability (i.e.,
  references to mutable data).

### Sync Trait

- The Sync trait indicates that it is safe to share an immutable reference of this type between threads.
- Types like integers, floats, strings etc. are all inherently sync because they don't have any interior mutability (
  i.e., they don't contain references to mutable data).
- Types like Mutex<T> or RwLock<T> require explicit implementation of Sync because they contain interior mutability (
  i.e., references to mutable data).

## Marker Traits

Section Overview: In this section, the speaker explains marker traits in more detail.

### Primitive Traits

- The marker module contains primitive traits and types representing basic properties of types.
- All marker traits are specifically traits with no methods.
- Marker traits only mark that a type meets a given property or guarantee about it but do not confer any additional
  behavior.

### Auto Traits

- Auto traits are implemented automatically by the compiler if all members of a type implement the trait.
- Not all marker traits are auto traits, but all auto traits are marker traits.

## Conclusion

Section Overview: In this section, the speaker concludes the video and summarizes what was covered.

### Summary

- Send and Sync are two important thread safety concepts in Rust that represent thread safety at the type level.
- They differ in their purpose: Send indicates that it is safe to move a value between threads while Sync indicates that
  it is safe to share an immutable reference between threads.
- Both Send and Sync are marker traits with no methods and auto traits which means that if all members of a type
  implement the trait, then the compiler will automatically implement it for you.

## Send and Sync

Section Overview: In this section, the speaker explains the difference between `Send` and `Sync`. He starts by
explaining what `Send` is and how it allows a value to be passed to another thread.

### Send Property

- The `Send` property tells you that it's okay to pass a value to another thread.
- Giving away ownership of a value to another thread confers that the thread can do whatever it wants with that value.
- Most types are `Send`, except for types where passing them to another thread might violate some implicit assumption or
  invariant of the underlying type. Two primary examples of such types are non-atomic reference counted type (`rc`) and
  mutex guards.

### Non-Send Types

- The non-atomic reference counted type (`rc`) is not `Send` because if you write code that passes an object created on
  one thread to another, then there is a risk of violating some internal invariant, which usually doesn't go very well.
- Mutex guards are also not `Send`. This applies both to standard mutexes and rw locks in the standard library backed by
  OS implementation because certain operating systems require that the same thread that gets a lock has to be the same
  thread that releases it.

## Rc Type

Section Overview: In this section, the speaker talks about why Rc (reference count) isn't send.

### Why Rc Isn't Send

- The reason why Rc isn't send is because if you create an object on one thread using its own local state and then try
  sending it over to another thread, when dropping it on the other end, there's no guarantee which threads' local state
  will be accessed leading to a violation of internal invariants.
- Rc is a little bit of a different beast because it's possible to write code that creates an object on one thread and
  then passes it to another thread, which can lead to problems.

## Introduction to Rc

Section Overview: In this section, the speaker introduces Rc and explains how it works.

### Construction of Rc

- An Rc is a reference-counted smart pointer in Rust.
- When an Rc is dropped, if the current count is one, then the inner thing from raw self.inner is dropped.
- Implement ops dref for rct. This will be just unsafe self.inner.value.
- The construction of an Rc involves keeping both the actual value that you're protecting and a count of the number of
  Rcs that there are.

### Safety of Unsafe Code

- The reason why this access is safe is because Rc is not send and not sync.
- We know that even though we have an immutable reference there are no other threads there's no other concurrent
  execution that is accessing count and that is actually getting inner at all they might like there are there might be
  other Rcs that have a reference to inner but none of them are currently accessing it because rc is not allowed to
  leave the thread.
- The reason this needs to be boxed from raw is because self.inter is just a raw pointer so dropping it does nothing we
  have to turn it into an owned box again so the box drop gets to run to deallocate the heap allocation and also run the
  destructor for the inner type t.

### Send Trait

- If Rc was send then anything that closure moves would be allowed to be sent to another thread. This operation would
  allow us to spawn a thread which requires send.

## Introduction to Send and Sync

Section Overview: In this section, the speaker introduces the concepts of Send and Sync in Rust programming language.
The speaker explains how these traits are used to ensure thread safety in Rust.

### Send Trait

- The `Send` trait is used to indicate that a type can be safely transferred between threads.
- If a type does not implement `Send`, it cannot be shared across thread boundaries because bad things will happen.
- The `thread::spawn` function requires that the type it's passed implements `Send`. If it doesn't, the compiler will
  throw an error.

### Sync Trait

- The `Sync` trait is defined in terms of the `Send` trait. A type T is sync if and only if a reference to T is send.
- If you have a type where a reference to that type is allowed to be shared across threads, then that type is sync even
  if the type itself cannot be passed to some other thread.

### Conclusion

- Thread safety is important in Rust programming language.
- The `Send` and `Sync` traits are used to ensure thread safety by indicating whether or not types can be safely
  transferred or shared between threads.

## Rust Types: Sync and Send

Section Overview: This section discusses the concepts of `Sync` and `Send` in Rust types. It explains how some types can
be `Send` but not `Sync`, while others can be both or neither.

### Understanding Rc and MutexGuard

- Rc is a type that allows multiple ownership of the same data, but it cannot be used across threads because it is
  not `Sync`.
- MutexGuard is another type that cannot be used across threads because it is not `Send`. However, it can be used within
  a single thread because it is `Sync`.

### Interior Mutability Types

- Interior mutability types like Cell and RefCell are examples of types that are `Send` but not necessarily `Sync`.
- Cell does not give out references to its inner value, so as long as there is only one thread accessing the value at
  any given time, it's safe to mutate the value inside.
- RefCell allows for mutable borrowing of its inner value, but only if there are no other active borrows at the same
  time.

### Requirements on Inner Types

- The requirements on the inner type of a type like Cell determine whether or not the outer type can be
  considered `Send`.
- For example, Cell can only implement Send if its inner type also implements Send.

## Rust Cell and Negative Implementations

Section Overview: In this section, the speaker discusses Rust's `Cell` type and negative implementations.

### Using Rust's `Cell`

- `Cell` is not super useful on its own because you could just have an owned value and take references to it.
- `Cell` comes in handy when doing something like graph traversal where you might have cycles and need shared access to
  all nodes in the tree at that point in time.
- You can modify a value through a shared reference with `Cell`.
- If the inner type of a `Cell` is copy, you can get the inner value even with just a reference to the cell.

### Negative Implementations

- Negative implementations are an unstable feature in Rust that allow you to write "not" implementations for traits
  like `Send`.
- You can always write positive implementations for traits like `Send`, but negative implementations require unsafe code
  because you are claiming something that the compiler cannot guarantee is true.
- It is rare that you actually need negative implementations. They may be necessary if your code holds only types that
  are send or sync, but accessing them makes it not thread-safe.

### Basic Implementation of Sync

- The basic implementation of sync involves implementing send for a reference to t where t is sync.
- There is also an implementation of send for a mutable reference to t where t is send.

## Memory Management in Rust

Section Overview: This section discusses the memory management in Rust and how it handles mutable references.

### Mutable References and Ownership

- The `mem::replace` function takes a mutable reference to a type `T` and returns the value that was stored behind this
  destination.
- If a mutable reference is sent to an immutable type, it can take ownership of something that was only passed as
  immutable. For example, if a mutex guard is given as a mutable reference, one could take their own lock, swap the two
  mutex guards, and then drop the original guard which would not be okay.

### Manual Implementations for Types

- There are explicit implementations of send for some types like `VecDeque`, but not all types have them.
- Manual implementations are required for types containing raw pointers because they require careful consideration of
  thread safety.
- Negative implementations exist for raw pointer types to prevent accidental sending or syncing when it's not safe.

### Send List

- The send list contains explicit implementations for anything that contains inner pointers.
- Raw pointers including non-null have negative implementations for send.
- Cell and RefCell both implement send but do not implement sync because it would not be safe.
- Atomic pointer implements send because if you're using atomic pointer chances are you've thought about sentencing.

## Procedural Macros and Thread Local State

Section Overview: This section discusses the implementation of procedural macros and how they do not implement send,
indicating the presence of thread local state.

### Procedural Macro Types

- Procedural macro types do not implement send.
- This suggests that there is thread local state when invoking procedural macros.
- The source for these types cannot be accessed as they are compiler implemented.

## Channel Senders and Arc

Section Overview: This section discusses channel senders and arc in Rust.

### Channel Senders

- Sender and receiver are not sync but are send.
- The sender method does not require t to be send because if a sender-receiver pair is created but never moved to other
  threads, there is no requirement for the types sent over it to be themselves send.
- There is only a requirement that t is send if the sender is sent or moved across thread boundaries.

### Arc

- Arc also does not require t to be send or sync.
- It has methods like get_mut which allow mutable references to the inner type if there's only one clone of the arc
  remaining.
- Arc is only send if t is sync and send.
- Only when an arc or reference to an arc is moved across a thread boundary does it require that the inner type be
  thread safe.

## Moving References Across Threads

Section Overview: This section discusses moving references across threads in Rust.

### Sending References

- Moving or sending a reference requires sync, which means it can be accessed by different threads.
- Explicitly implementing send for guards in certain OS's is possible but not recommended as it would make determining
  thread safety dependent on the operating system.
- Requiring mutex guards to not be send allows for more optimizations.

## Why is there no sync implementation for sender?

Section Overview: This section discusses why there is no sync implementation for sender in Rust.

### Possible Reasons

- The way a sender works internally in the standard library is each sender has a slot in the channel. If multiple
  threads were allowed to send through the same sender, they would be contending for that slot, which is not intended to
  be thread safe.
- Sending on a channel takes a reference to self, not an immutable reference. Therefore, you need a separate sender for
  each thread.
- The implementation may have bled into the API. It's not a fundamental requirement of any multi-producer single
  consumer queue.

## When is Send implemented by default?

Section Overview: This section explains when Send is implemented by default and when it's not.

### Auto Trait Implementation

- Whether or not Send is implemented depends entirely on what types it contains.
- If all the inner types of your type are themselves Send, then your type will be Send.
- If your type contains an Rc or MutexGuard or any kind of raw pointer, then your type will not be Send.

## Where can I find more information about Sync and Send?

Section Overview: This section provides resources where you can learn more about Sync and Send in Rust.

### Resources

- The Nomicon entry on Sync and Send talks about many of the things discussed in this video.
- The auto implementations are much less interesting because they are implemented without the programmer of that type
  having sat down and really thought about the descendant Syncness. The manual implementations are usually more
  instructive to look at.
- The feature is called negative impulse, and it should be incredibly rare that you actually have to add such a negative
  implementation to yourself.

## Introduction to Send and Sync

Section Overview: In this section, the speaker introduces the concept of mutex guards and explains that send and sync
are not baked into the compiler. Instead, they rely on auto traits.

### Auto Traits

- The compiler only knows that some types implement send and sync.
- If all inner types of a type implement send, then that type implements send.
- Same for sync.
- Send and sync are not baked into the compiler at all; they rely on auto traits.

### Dynamic Trait Dispatch

- Because send and sync are auto traits, you can include them in dynamic trait dispatch as additional traits.
- Normally, you're only allowed to name one trait when using din syntax.
- You can use iterator plus clone instead of din iterator plus clone.
- However, with auto traits like send and sync, you can write "this type is an iterator and it is also both send and
  sync."

### Futures Code

- You'll see din future output equals something plus send plus sync indicating that this particular future is safe to
  send to another thread to continue execution there.
- Multi-threaded executors like the Tokyo executor require that futures passed to them are sent unless you use a
  specific construct called local set which allows you to run non-send futures.

## Thread Spawn Requirements

Section Overview: In this section, the speaker explains that arc and channels only require sending or syncing because
thread spawn requires it.

### Thread Spawn Requirements

- Arc and channels only require sending or syncing because thread spawn requires it.
- The reason thread spawn requires sending or syncing is because somewhere deep down there they spawn new threads whose
  arguments need to be sent.

## Rust: Send and Sync

Section Overview: In this section, the speaker discusses the implementation of thread spawning in Rust and how it
relates to the Send and Sync traits. They also explain why these traits are not magic and how they work in practice.

### Thread Spawning and Graph Routes

- Thread spawning is a key primitive for concurrency in Rust.
- There are no other routes to this graph than thread spawn.
- Somewhere in the call graph deep down there's a thread spawn.

### Send Trait

- The Send trait ensures that types can be safely sent between threads.
- Thread spawn only requires that the type is static and send, it does not require sync because it doesn't take
  references to anything.

### Static Requirement for Threads

- The reason thread spawn needs to be static is that there's nothing that guarantees that this thread won't outlive the
  current thread.
- If a value on my stack has a reference to my stack then that thread might keep running after I return, making that
  reference invalid.

### Scoped Threads

- There is some work in the standard library now to add back thread scoped which allows scoped threads so you don't need
  static arguments.
- This ensures that the current stack frame can't return until the thread has been joined so you know that the thread
  won't outlive the current stack frame.

### Negative Implementations

- Negative implementations for sort of key things in the standard library are used in thread bounds.
- Currently, negative trade bounds are not yet fully implemented on stable compilers.

### Not Sync + Send Implementation

- You just write those impulses if you want to implement not sync + send.
- On stable compilers, you can use phantom data as a workaround.

## Understanding Phantom Data and Auto Traits

Section Overview: In this section, the speaker explains what phantom data is and how it propagates through auto traits.
They also discuss how implementing sync and send are both unsafe properties.

### Phantom Data and Auto Traits

- Phantom data means that you're not actually storing an rc; the phantom data is just like a pretend this type was here
  but don't actually put one here.
- Auto traits are compiler magic. When it constructs a type, the compiler looks at all the contained types, and if all
  of them meet the trait that's marked as auto, then the outer type also implements that trait.
- Implementing sync and send are both unsafe properties because you would only write these if the compiler's auto trade
  inference says that these types are not sent in sync.

### Unsafe Properties

- The meaning of unsafe is the compiler saying "you prove it if you're so smart." Manually implementing these is unsafe.
- Auto inference is not unsafe. If I remove these, mutex guard will be send and sync, and I didn't have to do anything
  unsafe.

## General Q&A

Section Overview: In this section, the speaker takes questions from viewers on various topics unrelated to sentencing.

No bullet points available for this section as there were no timestamps associated with any specific question or answer.

## Generated by Video Highlight

https://videohighlight.com/video/summary/yOezcP-XaIw