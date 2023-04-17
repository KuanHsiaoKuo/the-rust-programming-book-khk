# Atomics and Memory Ordering

<!--ts-->
* [Atomics and Memory Ordering](#atomics-and-memory-ordering)
   * [Introduction](#introduction)
      * [Why Do We Need Atomics?](#why-do-we-need-atomics)
      * [What Will Be Covered?](#what-will-be-covered)
   * [Atomic Types](#atomic-types)
      * [Why Not Use Primitive Types?](#why-not-use-primitive-types)
      * [What Are Atomic Types?](#what-are-atomic-types)
      * [Why Do We Need Different APIs for Atomic Types?](#why-do-we-need-different-apis-for-atomic-types)
   * [Conclusion](#conclusion)
      * [Key Takeaways](#key-takeaways)
   * [Rust Memory Model](#rust-memory-model)
      * [Introduction to Memory Model](#introduction-to-memory-model)
      * [Memory Ordering Documentation](#memory-ordering-documentation)
      * [Atomic Types in Rust](#atomic-types-in-rust)
      * [Methods for Atomic Use Size Type](#methods-for-atomic-use-size-type)
   * [Introduction to Memory Ordering](#introduction-to-memory-ordering)
      * [Memory Ordering and Guarantees](#memory-ordering-and-guarantees)
      * [Atomic Operations](#atomic-operations)
      * [Questions from Viewers](#questions-from-viewers)
      * [Atomic Operations and Compiler Semantics](#atomic-operations-and-compiler-semantics)
      * [Unsafe Cell and Atomic Instructions](#unsafe-cell-and-atomic-instructions)
      * [Using Arc for Static Lifetime](#using-arc-for-static-lifetime)
      * [Alternative Approach with Box Leak](#alternative-approach-with-box-leak)
      * [Defining a Mutex Type](#defining-a-mutex-type)
      * [Using with Lock Function](#using-with-lock-function)
   * [Introduction to Mutexes](#introduction-to-mutexes)
      * [Naive Implementation of Mutex](#naive-implementation-of-mutex)
      * [Problems with Naive Implementation](#problems-with-naive-implementation)
      * [Testing Mutex](#testing-mutex)
   * [Implementing a Mutex](#implementing-a-mutex)
      * [Collecting Thread Handles](#collecting-thread-handles)
      * [Sync for Mutex](#sync-for-mutex)
      * [Problems with Mutex Implementation](#problems-with-mutex-implementation)
   * [Understanding Concurrency Issues](#understanding-concurrency-issues)
      * [Concurrency Issues with Multiple Cores](#concurrency-issues-with-multiple-cores)
      * [Race Conditions and Preemption](#race-conditions-and-preemption)
      * [Solving Concurrency Issues with Compare Exchange Operation](#solving-concurrency-issues-with-compare-exchange-operation)
   * [Overview of Compare Exchange](#overview-of-compare-exchange)
      * [How Compare Exchange Works](#how-compare-exchange-works)
      * [Why Compare Exchange Is Useful](#why-compare-exchange-is-useful)
   * [Compare Exchange and Spin Locks](#compare-exchange-and-spin-locks)
      * [Compare Exchange vs. Spin Locks](#compare-exchange-vs-spin-locks)
      * [Optimizing Spin Locks for High Contention](#optimizing-spin-locks-for-high-contention)
   * [Compare Exchange Weak](#compare-exchange-weak)
   * [Load Exclusive and Store Exclusive](#load-exclusive-and-store-exclusive)
      * [Load Exclusive and Store Exclusive](#load-exclusive-and-store-exclusive-1)
   * [Memory Ordering](#memory-ordering)
      * [Ordering Variants](#ordering-variants)
   * [Memory Barriers](#memory-barriers)
      * [Memory Barriers](#memory-barriers-1)
   * [Happens-Before Relationship](#happens-before-relationship)
      * [Happens-Before Relationship](#happens-before-relationship-1)
   * [Lock-Free Programming](#lock-free-programming)
      * [Lock-Free Programming](#lock-free-programming-1)
   * [Relaxed Ordering in Rust](#relaxed-ordering-in-rust)
      * [Relaxed Ordering and Thread Execution](#relaxed-ordering-and-thread-execution)
      * [Explanation of Relaxed Ordering](#explanation-of-relaxed-ordering)
   * [Reordering and Speculative Execution](#reordering-and-speculative-execution)
      * [Reordering in Programming](#reordering-in-programming)
      * [Speculative Execution](#speculative-execution)
      * [Constraints on Reordering](#constraints-on-reordering)
   * [Exclusive Access and Relaxed Memory Ordering](#exclusive-access-and-relaxed-memory-ordering)
      * [Exclusive Access](#exclusive-access)
      * [Relaxed Memory Ordering and Locks](#relaxed-memory-ordering-and-locks)
   * [Memory Orderings and Acquiring/Releasing Resources](#memory-orderings-and-acquiringreleasing-resources)
      * [Store with Release Ordering](#store-with-release-ordering)
      * [Load with Acquire Ordering](#load-with-acquire-ordering)
      * [Additional Restrictions](#additional-restrictions)
   * [Understanding Acquire-Release Semantics](#understanding-acquire-release-semantics)
      * [Acquire-Release Semantics](#acquire-release-semantics)
      * [Compare Exchange Operation](#compare-exchange-operation)
      * [Testing Concurrent Code](#testing-concurrent-code)
   * [Relaxed and Strong Memory Ordering](#relaxed-and-strong-memory-ordering)
      * [Relaxed Memory Ordering](#relaxed-memory-ordering)
      * [Fetch Operations](#fetch-operations)
   * [Fetch Operations and Sequentially Consistent Ordering](#fetch-operations-and-sequentially-consistent-ordering)
      * [Sequentially Consistent Ordering](#sequentially-consistent-ordering)
      * [Fetch Operations with Sequentially Consistent Ordering](#fetch-operations-with-sequentially-consistent-ordering)
   * [Understanding Fetch Operations](#understanding-fetch-operations)
      * [Fetch Update](#fetch-update)
      * [Unique Sequence Numbers](#unique-sequence-numbers)
   * [Sequentially Consistent Ordering](#sequentially-consistent-ordering-1)
      * [Example with Atomic Bools and Usize](#example-with-atomic-bools-and-usize)
   * [Possible Values for Z](#possible-values-for-z)
      * [Two is Possible](#two-is-possible)
      * [One is Possible](#one-is-possible)
      * [Zero May Not Be Possible](#zero-may-not-be-possible)
   * [Mutex Panic Safety](#mutex-panic-safety)
   * [Introduction](#introduction-1)
      * [Thread Execution Order](#thread-execution-order)
   * [Thread Schedules](#thread-schedules)
      * [Thread Schedules](#thread-schedules-1)
   * [Acquire and Release Semantics](#acquire-and-release-semantics)
      * [Acquire and Release Semantics](#acquire-and-release-semantics-1)
   * [Memory Ordering in Rust](#memory-ordering-in-rust)
      * [Happens Before Relationship](#happens-before-relationship-2)
      * [Concurrent Operations](#concurrent-operations)
      * [Sequentially Consistent Operation](#sequentially-consistent-operation)
   * [Memory Ordering and Sequential Consistency](#memory-ordering-and-sequential-consistency)
      * [Memory Ordering](#memory-ordering-1)
      * [Testing Concurrent Code](#testing-concurrent-code-1)
   * [Detecting Concurrency Problems](#detecting-concurrency-problems)
      * [Thread Sanitizer](#thread-sanitizer)
      * [Loom](#loom)
   * [Loom: A Tool for Testing Concurrent Code](#loom-a-tool-for-testing-concurrent-code)
      * [Exploring All Possible Legal Executions](#exploring-all-possible-legal-executions)
      * [Limitations of Loom](#limitations-of-loom)
   * [Examples for Sequentially Consistent Ordering](#examples-for-sequentially-consistent-ordering)
      * [No Less Contrived Example Available](#no-less-contrived-example-available)
      * [Recommendation](#recommendation)
   * [Limitations of Zulu](#limitations-of-zulu)
      * [Limitations of Zulu](#limitations-of-zulu-1)
      * [Example for Relaxed Ordering](#example-for-relaxed-ordering)
   * [Contributing to Loom](#contributing-to-loom)
      * [Benefits of Contributing to Loom](#benefits-of-contributing-to-loom)
      * [Limitations of Loom](#limitations-of-loom-1)
      * [How Contributions Can Help](#how-contributions-can-help)
   * [Testing Sequentially Consistent Ordering with Acquire Release](#testing-sequentially-consistent-ordering-with-acquire-release)
      * [Testing Sequential Consistency with Acquire Release](#testing-sequential-consistency-with-acquire-release)
   * [Overview of Loom and Relaxed Ordering](#overview-of-loom-and-relaxed-ordering)
      * [Atomic Module in Sender Library](#atomic-module-in-sender-library)
      * [Memory Ordering](#memory-ordering-2)
   * [Volatile Keyword](#volatile-keyword)
      * [Volatile Keyword](#volatile-keyword-1)
   * [Understanding Volatile](#understanding-volatile)
      * [What is Volatile?](#what-is-volatile)
   * [Atomic Pointer](#atomic-pointer)
   * [What is Atomic Pointer?](#what-is-atomic-pointer)
* [Generated by Video Highlight](#generated-by-video-highlight)

<!-- Created by https://github.com/ekalinin/github-markdown-toc -->
<!-- Added by: runner, at: Mon Apr 17 15:51:08 UTC 2023 -->

<!--te-->

##  Introduction

Section Overview: In this video, the speaker will explain atomics and memory ordering. The speaker has been hesitant to tackle this topic because of the lack of documentation and complexity of the subject. However, after working with it enough, he feels confident in giving a correct and understandable explanation.

### Why Do We Need Atomics?

-  If you have a primitive type like bool or usize that is shared across thread boundaries, there are only certain ways to interact with that value that are safe.
-  It makes sense to have slightly different APIs for atomic types because you're issuing different instructions to the underlying CPU and placing different limitations on what code the compiler is allowed to generate.
-  When you have shared access to some memory value, you need additional information about that access to let the CPU know when should different threads see the operations that other threads do.
-  Rust's standard sync atomic model has good documentation on high-level ideas of why we have these types and what they're used for.

### What Will Be Covered?

-  The stream will be Rust-specific but most of it translates to any other language that has atomics in this kind of sense.
-  The core focus will be on Rust's atomic types and memory ordering observed in Rust.

##  Atomic Types

Section Overview: This section explains why using primitive types like bool or usize can lead to undefined behavior when shared across thread boundaries. It also explains how atomic types provide additional information about shared access to memory values.

### Why Not Use Primitive Types?

-  There are only certain ways to interact with primitive types safely when they are shared across thread boundaries.
-  Data races are undefined behavior, so if you don't control how different threads interoperate on the same memory location, you run into undefined behavior.
-  Atomic types provide additional information about shared access to memory values.

### What Are Atomic Types?

-  When operating on atomic versions of primitive types, you're issuing different instructions to the underlying CPU and placing different limitations on what code the compiler is allowed to generate.
-  You need additional information about shared access to memory values to let the CPU know when should different threads see the operations that other threads do.

### Why Do We Need Different APIs for Atomic Types?

-  It makes sense to have slightly different APIs for atomic types because they place different limitations on what code the compiler is allowed to generate.

##  Conclusion

Section Overview: The speaker concludes by summarizing why atomics are necessary and what will be covered in this video. He also emphasizes that understanding memory ordering is useful regardless of what language you're working in.

### Key Takeaways

-  If you have a primitive type like bool or usize that is shared across thread boundaries, there are only certain ways to interact with that value that are safe.
-  Rust's standard sync atomic model has good documentation on high-level ideas of why we have these types and what they're used for.
-  Understanding memory ordering is useful regardless of what language you're working in.
##  Rust Memory Model

Section Overview: In this section, the speaker talks about the Rust memory model and how it is not yet fully defined. They also introduce the Rust reference as a source of information for implementing a Rust compiler.

### Introduction to Memory Model

-  The Rust language does not have a fully defined memory model yet.
- Various proposals are being worked on by academics and industry professionals.
- This can be unhelpful when writing code that uses atomics and concurrency.

### Memory Ordering Documentation

-  The speaker will be using the memory ordering documentation from C++ as it has good explanations and examples of different memory orderings.
- This page will be used for explanations, examples, and guarantees throughout the talk.

### Atomic Types in Rust

-  Atomic types in Rust are not inherently shared and need to be placed on the heap using something like `Box` or `Arc`.
- This allows sharing a pointer or reference to that atomic value which can then be updated.
- The main differentiator between atomic types and regular types is that atomic types can be operated on using shared references to self.

### Methods for Atomic Use Size Type

-  There are several methods available for operating on an atomic use size type, including load, store, swap, compare_and_swap, fetch_add/sub/and/or/xor/nand.
- Load returns the value stored in the atomic use size type while store takes a value and stores it into the type.
- Swap performs both operations at once by returning the old value while storing a new one.
- Compare_and_swap compares the current value with an expected value and if they match, replaces it with a new one. Otherwise, it returns the current value without modifying it.
- Fetch_add/sub/and/or/xor/nand performs the corresponding operation on the value stored in the atomic use size type and returns the old value.
##  Introduction to Memory Ordering

Section Overview: In this section, the speaker introduces memory ordering and its importance in multi-threaded programming. They also introduce some methods for reading and swapping values atomically.

### Memory Ordering and Guarantees

- Memory ordering tells the compiler which set of guarantees you expect for a particular memory access with respect to things that might be happening in other threads at the same time.
- There are different types of guarantees provided by memory ordering, such as acquire, release, and sequentially consistent.

### Atomic Operations

- Atomic operations allow multiple threads to operate on a value at the same time in a well-defined way without locking.
- Compare-and-swap is an atomic operation that reads a value and swaps it out conditionally in one atomic step. It ensures no thread can get between the load and store operations.
- Fetch methods like fetch-add or fetch-sub are similar to compare-and-swap but perform arithmetic operations instead of conditional swaps. They also ensure no thread can execute between load and store operations.

### Questions from Viewers

- Non-exhaustive means that no code is allowed to assume that these are all the variants that will ever be in ordering so if you match on an ordering, you always need to include an underscore branch in an otherwise or else branch because the standard library wants to be able to add additional orderings later if necessary.
- The different orderings are related to what guarantees the operation gives you; how different architectures implement those guarantees will vary from architecture to architecture as we'll see later on.
- Mutex guards a larger section of code while atomic guards just a single memory access if needed but more efficiently than mutexes would.
## [#](t=0:16:28) Understanding Atomic Types in Rust

Section Overview: In this section, the speaker explains how atomic types work in Rust and why they are considered lock-free.

### Atomic Operations and Compiler Semantics

- [](t=0:16:28) Atomic types in Rust are considered lock-free because they do not use mutexes.
- [](t=0:16:51) Atomic operations limit both what the CPU can do and what the compiler can do about a given memory access.
- [](t=0:17:08) Even on architectures like Intel x86 64, it is still recommended to use atomic types because they provide guarantees from the compiler as well.

### Unsafe Cell and Atomic Instructions

- [](t=0:17:45) The store swap and friends all take an immutable reference to self, which means they rely on unsafe cell.
- [](t=0:18:09) All of these atomic instructions contain an unsafe cell that they then call get on to get the pointer to the value.
- [](t=0:18:30) The atomic type macro expands to a struct definition that holds an unsafe cell of the inner type.

## [#](t=0:19:07) Sharing Atomics with Arc

Section Overview: In this section, the speaker discusses why atomics are generally shared via Arc rather than Box.

### Using Arc for Static Lifetime

- [](t=0:19:27) When spawning two threads, both require that the closure you pass in has a static lifetime.
- [](t=0:19:49) With an Arc, you can clone it and give each thread its own individually owned and therefore static Arc to the atomic usize.

### Alternative Approach with Box Leak

- [](t=0:20.08s) Box leak will leak a value on the heap so it will never call the destructor which gives you back a static reference which you can then pass to both threads.

## [#](t=0:20:23) Implementing a Mutex Type

Section Overview: In this section, the speaker demonstrates how to implement a mutex type using atomic types in Rust.

### Defining a Mutex Type

- [](t=0:21:13) A mutex is a combination of a boolean that marks whether or not someone currently holds the lock and then an unsafe cell to the inner type.
- [](t=0:21:43) The implementation requires Sync Atomic and Atomic Bool.

### Using with Lock Function

- [](t=0:22:06) The with_lock function takes an impul f and once that's given immutable value to t returns r.
##  Introduction to Mutexes

Section Overview: In this section, the speaker introduces mutexes and explains why spin locks should not be used.

### Naive Implementation of Mutex

-  The speaker creates a new method for the mutex.
-  The method takes an initial value to create the mutex with.
-  Constants are created to make the code more readable.
-  The speaker explains how the naive implementation of mutex works using a while loop that spins until the lock is no longer held.
-  The function captures return value, sets it back to unlocked, and returns red.

### Problems with Naive Implementation

-  The code does not compile because an argument is missing in the method.
-  Ordering is required in this method but it's not clear what ordering should be used yet. For now, relaxed ordering will be used.

### Testing Mutex

-  A lock mutex is created and five threads are spawned to modify its value at the same time through the lock.
-  Each thread uses l.with_lock() and adds 1 to v a hundred times.
##  Implementing a Mutex

Section Overview: In this section, the speaker explains how to implement a mutex in Rust using atomics and compare-and-swap operations.

### Collecting Thread Handles

- We collect all thread handles so that we can wait for them afterwards.
- We spin up 10 threads, each of which increments the value 100 times.

### Sync for Mutex

- Unsafe cell cannot be shared between threads safely.
- Mutex is sync for mutex t where t is send.
- We implement sync from mutex so that you can concurrently access it from multiple threads as long as the value t is send.

### Problems with Mutex Implementation

- The reason we need atomics and why we need operations like compare and swap is because here we're doing a load and then we're doing a store but in between here maybe another thread runs.
- It's possible for two threads to execute the closure passed to with lock at the same time, leading to undefined behavior.
##  Understanding Concurrency Issues

Section Overview: This section discusses the issues that arise when multiple threads are running concurrently and accessing shared memory.

### Concurrency Issues with Multiple Cores

-  When different threads run on different cores, there is no control over the relative operations of each thread.
-  The operating system limits how long a given thread gets to run for and may forcibly stop a program in the middle to ensure all threads get to run.

### Race Conditions and Preemption

-  Preemption can occur at any point in time, including between a load and store operation.
-  A race condition occurs when two threads observe the mutex being unlocked at the same time.

### Solving Concurrency Issues with Compare Exchange Operation

-  The compare exchange operation is used to avoid race conditions between multiple threads accessing shared memory.
-  Compare exchange is more powerful than compare and swap because it strictly avoids race conditions.
##  Overview of Compare Exchange

Section Overview: In this section, the speaker explains how compare exchange works and why it is useful.

### How Compare Exchange Works

-  `compare_exchange` takes the current value, new value, and two orderings.
- The first line of documentation stores a value into the atomic integer or boolean if the current value is the same as the current value.
- If the CPU looks at the atomic bool and sees that it is unlocked (false), then it sets it to true in such a way that no other thread gets to modify the value in between when we look at it and when we change it.
- Compare exchange is a single operation that is both read and write.
- It can be done in a loop because if the current value is locked, then compare exchange will return an error.

### Why Compare Exchange Is Useful

-  Compare exchange solves this particular problem because there's no space in between load and store; they're just one operation performed on memory location.
- However, compare exchange can be expensive because every CPU spins during compare exchange.
- CPUs need to coordinate exclusive access amongst all these cores which can be inefficient.
##  Compare Exchange and Spin Locks

Section Overview: This section discusses the differences between compare exchange and spin locks, and how to optimize spin locks for high contention.

### Compare Exchange vs. Spin Locks

-  In compare exchange, the CPU requires exclusive access to a location in memory, which requires coordination with everyone else.
-  A given location of memory can be marked as shared, allowing multiple cores to have a value in the shared state at the same time.
-  Compare exchange is much faster than locking a mutex because it will never wait. A single compare exchange call will try to do the operation you told it and then say either "I succeeded" or "I failed". If it fails, it does not block the thread.
-  The biggest difference between compare exchange and a mutex is that a mutex has to wait while a compare exchange will never wait.

### Optimizing Spin Locks for High Contention

-  If we can have a value stay in the shared state while the lock is still held, that's going to avoid all of the ownership bouncing.
-  Certain spin lock implementations use an inner loop that only reads values if they fail to take the lock. This allows values to stay in the shared state until some core takes exclusive access to them.
-  When there is high contention on a particular value with many threads or cores fighting over who gets exclusive access, performance may collapse. Avoiding this collapse by optimizing spin locks can lead to better performance curves.

##  Compare Exchange Weak

Section Overview: This section explains what compare exchange weak is and how it differs from regular compare exchange.

-  Compare exchange weak is allowed to fail spuriously, even if the current value is what you passed in. Regular compare exchange is only allowed to fail if the current value did not match the value you passed in.
-  The difference between these two operations comes down to what operations the CPU supports. On Intel x86, there is a compare and swap operation that effectively implements compare exchange.
##  Load Exclusive and Store Exclusive

Section Overview: This section discusses the difference between compare and swap operations on x86 and ARM processors.

### Load Exclusive and Store Exclusive

-  On ARM processors, instead of a compare and swap operation, there is a load exclusive (ldrex) and store exclusive (strex) operation.
-  Load exclusive takes exclusive ownership of the memory location and loads the value to you.
-  Store exclusive only works if you still have exclusive access to that location. If another thread has taken ownership of the value, it will fail.
-  Compare exchange on ARM is implemented using a loop of ldrex and strex because it needs to implement the semantics of comparexchange which is only fail if the current value stayed the same.
-  Compare exchange weak is allowed to fail spuriously so it can be implemented directly using ldrx and strx.

##  Memory Ordering

Section Overview: This section discusses how memory ordering affects observable behavior when multiple threads interact with a shared memory location.

### Ordering Variants

-  There are different variants for ordering: relaxed, release, acquire, acquire-release, sequential consistency.
-  Relaxed means there are no guarantees about what happens when multiple threads interact with a shared memory location.
-  Release ensures that all previous writes become visible before any subsequent write or read from another thread occurs.
-  Acquire ensures that all previous reads become visible before any subsequent read or write from another thread occurs.
-  Acquire-release combines both acquire and release semantics.
-  Sequential consistency ensures that all operations appear to execute in a total order.

##  Memory Barriers

Section Overview: This section discusses memory barriers and how they can be used to enforce ordering constraints.

### Memory Barriers

-  Memory barriers are instructions that enforce ordering constraints on memory operations.
-  There are two types of memory barriers: compiler barriers and hardware barriers.
-  Compiler barriers prevent the compiler from reordering memory operations across the barrier.
-  Hardware barriers ensure that all previous memory operations have completed before any subsequent operation is executed.
-  The C++11 standard defines several types of hardware barriers, including acquire, release, and full.

##  Happens-Before Relationship

Section Overview: This section discusses the happens-before relationship and how it relates to observable behavior when multiple threads interact with shared memory locations.

### Happens-Before Relationship

-  The happens-before relationship is a partial order between events in a program that determines what values can be observed by other threads.
-  If event A happens before event B, then any value written by A must be visible to B.
-  If there is no happens-before relationship between two events, then their order is undefined.
-  Synchronizes-with is a special case of happens-before where one event synchronizes with another if it releases a mutex or semaphore that another thread acquires later.

##  Lock-Free Programming

Section Overview: This section discusses lock-free programming and its advantages and disadvantages.

### Lock-Free Programming

-  Lock-free programming is a technique for writing concurrent programs without using locks.
-  The advantage of lock-free programming is that it can avoid the overhead of acquiring and releasing locks, which can be expensive.
-  The disadvantage of lock-free programming is that it can be more difficult to reason about and debug than lock-based programming.
-  Lock-free programming requires careful attention to memory ordering constraints and the happens-before relationship.
##  Relaxed Ordering in Rust

Section Overview: In this section, the speaker explains how relaxed ordering works in Rust and why it can lead to unexpected results.

### Relaxed Ordering and Thread Execution

-  The speaker spawns two threads and creates an atomic variable x with size 0.
-  The speaker creates another atomic variable y with size 0.
-  Box leak returns a static mutable reference which cannot be moved into two threads. Therefore, the speaker casts it as an atomic variable.
-  The speaker creates two threads, x and y, both of which are numbers.
-  Thread 1 reads y with relaxed ordering and stores that value into x.
-  Thread 2 loads x and then stores 42 into y.
-  Both threads are joined so that we have the values t2.

### Explanation of Relaxed Ordering

-  It is possible for r1 to be equal to r2 be equal to 42. This is a possible execution of this program.
-  When multiple threads execute concurrently by default there are no guarantees about what values a thread can read from something another thread wrote under ordering relaxed.
-  There is a modification order stored per value in atomic operations. When you load a value with ordering relaxed you can see any value written by any thread to that location there's no restriction on when that has to happen relative to you.
-  For example, when loading a value from x, it is allowed to see any value ever stored to x including 42 because it's in the modification set of x and there are no constraints on which subset of that which range of that modification order is visible to this thread.
-  The compiler is totally allowed to reorder these two operations. Similarly, the CPU is allowed to execute them out of order for optimization purposes.
-  Under either of these conditions, the reverse execution might happen and if the code looks like this it's totally obvious why r2 might be 42. With ordering relaxed you're not guaranteed any ordering between threads.
##  Reordering and Speculative Execution

Section Overview: In this section, the speaker discusses how reordering makes sense in programming and how CPUs can execute operations speculatively. They also explain the constraints on what CPUs and compilers are allowed to reorder.

### Reordering in Programming

-  Reordering lines of code that don't depend on each other can make sense for programmers.
-  There is no downside to swapping two lines of code that don't depend on each other.
-  Observable execution outcomes are different, but in the case of relaxed memory ordering, they're interchangeable as far as the memory ordering specification is concerned.

### Speculative Execution

-  CPUs can run an operation before another one if it knows it's about to be executed. This is called speculative execution.
-  Speculative execution usually comes up when there's a question whether an operation might be executed or not.
-  Today's CPUs are very good at out-of-order execution because it makes programs much faster.

### Constraints on Reordering

-  CPUs and compilers are allowed to reorder anything that doesn't have a clear "happens before" relationship.
-  If two lines of code have a dependence relation (i.e., if one depends on the other), they cannot be reordered.
-  Memory ordering and semantics are not just about the CPU or architecture; they're also about what the compiler is allowed to do.

##  Exclusive Access and Relaxed Memory Ordering

Section Overview: In this section, the speaker explains why relaxed memory ordering matters when taking locks with exclusive access.

### Exclusive Access

-  If we already have exclusive access to a piece of memory, we can do an operation on it efficiently.
-  In the case of relaxed memory ordering, this can lead to shenanigans where operations are executed out of order.

### Relaxed Memory Ordering and Locks

-  Relaxed memory ordering doesn't imply any ordering or that anything happens before anything else.
-  When taking locks with relaxed memory ordering, operations might execute while another thread is holding the lock, violating the exclusivity of immutable reference. This is bad for both the compiler and CPU.
-  The reordering we give determines whether an operation is legal or not.
##  Memory Orderings and Acquiring/Releasing Resources

Section Overview: This section discusses memory orderings in the context of acquiring or releasing a resource. It explains how the release ordering ensures that nothing moves below the lock, and how the acquire ordering guarantees that anything that happened before the store happens before anything that happens after the load.

### Store with Release Ordering

- A reordering is valid with relaxed, but not with release.
- The release ordering ensures that nothing moves below the lock.
- All previous operations become ordered before any load of this value with acquire or stronger ordering.

### Load with Acquire Ordering

- Anything we do in here must be visible after an acquire load of the same value.
- Whoever next takes the lock must see anything that happened before this store.
- The acquire-release pair establishes a happens-before relationship between threads.

### Additional Restrictions

- No reads or writes in the current thread can be reordered after a store operation with memory order release.
- A load operation with memory order acquire performs the acquire operation on the effective memory location.
##  Understanding Acquire-Release Semantics

Section Overview: In this section, the speaker explains acquire-release semantics and how they are used in synchronization operations.

### Acquire-Release Semantics

-  Acquire-release semantics are used in synchronization operations that involve a read and write operation.
-  The acquire-release semantics establish ordering between threads to ensure correct synchronization.
-  Acquire-release is commonly used for single modification operations where there is no critical section.
-  Sequentially consistent ordering is stronger than acquire-release.

### Compare Exchange Operation

-  The compare exchange operation has an extra parameter that specifies the ordering of the load if it indicates that you shouldn't store.
-  Establishing a happens-before relationship between threads is important when taking a lock.
-  Relaxed memory semantics can be used when failing to take a lock does not require coordination with other threads.

### Testing Concurrent Code

-  x86_64 architecture guarantees acquire-release semantics for all operations, but this is not true for all platforms.
-  Testing concurrent code by running it multiple times on current hardware and compiler may not be representative of future executions.
##  Relaxed and Strong Memory Ordering

Section Overview: This section discusses the use of relaxed memory ordering when it doesn't matter what each thread sees. It also introduces fetch operations as a gentler version of compare exchanges.

### Relaxed Memory Ordering

- Use relaxed memory ordering when it doesn't matter what each thread sees.
- For example, if you maintain a counter for statistics, you can generally have it be relaxed because the relative ordering of execution doesn't really matter.
- Relax imposes the least amount of restrictions on the CPU and compiler, allowing them to execute more efficiently.

### Fetch Operations

- Fetch operations are gentler versions of compare exchanges.
- They allow you to tell the CPU how to compute the new value instead of just dictating what the updated value should be.
- Fetch ad is an example where you add one to the current value and store it back without failing.
- Fetch update takes a closure that is given the current value and should return the new value.

##  Fetch Operations and Sequentially Consistent Ordering

Section Overview: This section explains how fetch operations work in combination with sequentially consistent ordering.

### Sequentially Consistent Ordering

- Sequentially consistent ordering ensures that all threads see a consistent order of events.
- It's useful when there are dependencies between threads or instructions.

### Fetch Operations with Sequentially Consistent Ordering

- When using fetch operations with sequentially consistent ordering, they behave like acquire-release semantics.
- The operation has both an acquire effect on any previous memory accesses by other threads and a release effect on any subsequent memory accesses by other threads.
##  Understanding Fetch Operations

Section Overview: In this section, the speaker explains what fetch operations are and how they work.

### Fetch Update

-  `fetch_update` loads the current value and does a compare exchange in a loop.
- It is not the same as other fetch operations because it uses a compare exchange loop.
- Atomic types in Rust's atomic module are guaranteed to be lock-free if available.

### Unique Sequence Numbers

-  Fetch operations are used for things like giving unique sequence numbers to concurrent operations.
- Using `fetch_add` on an atomic usize guarantees that every call to get a sequence number will get a distinct one.

##  Sequentially Consistent Ordering

Section Overview: In this section, the speaker discusses sequentially consistent ordering and provides an example to demonstrate its difference from acquire-release ordering.

### Example with Atomic Bools and Usize

-  The example involves two threads storing values in atomic bools and one thread doing a fetch add on an atomic usize.
- One thread stores true with ordering release while another stores y with no specified ordering.
- A third thread does while not x load, acquires in a loop, checks if y again will acquire, then fetch adds one to z which is relaxed.
##  Possible Values for Z

Section Overview: In this section, the speaker discusses possible values for z in a given code snippet.

### Two is Possible

-  Execution order: t x ty t1 t2
- X is true and Y is true, so Z increments by 1 twice.
- Therefore, Z equals 2.

### One is Possible

-  Execution order: tx t1 t2 ty t2
- X is true but Y is false when T1 runs, so it does not increment Z.
- When Ty runs, it sets Y to true.
- When T2 runs, it sees that both X and Y are true and increments Z by 1.
- Therefore, Z equals 1.

### Zero May Not Be Possible

-  To achieve zero as the value of Z:
    - T1 must run after Tx.
    - T2 must run after Ty.
    - There are no other restrictions on execution order.

##  Mutex Panic Safety

Section Overview: In this section, the speaker answers a question about mutex panic safety.

- The mutex is panic safe but will not propagate panics.
##  Introduction

Section Overview: In this section, the speaker discusses a pattern and execution order for threads.

### Thread Execution Order

- The speaker explains that if t2 goes here, then ty must go before that and t1 will increment c because it runs after x and y have both been set to true.
- If t2 goes after tx, ty still has to go before t2 is going to get to do anything useful. In either case, t1 and t2 will increment z because they're both going to go after the things that set x and y.
- If we place t2 at the end, ty could go here but if ty goes there then tx is already run so t2 will increment z. If ty went earlier, t2 would still increment c so there's no place given this restriction where one of them won't increment z. It seems impossible to have a thread schedule where z equals zero.

##  Thread Schedules

Section Overview: In this section, the speaker talks about thread schedules and how computers don't have a single order that things run in.

### Thread Schedules

- The speaker explains that thread schedules are just like the human desire to put things in order but in reality computers don't have a single order that things run in.
- We have multiple cores and those cores can show old values new values all we're subject to are the exact rules of acquire and release semantics which is what we've given here.
- There's no requirement that we see anything that happened in another thread. When you spawn a thread these threads all happen after the main thread that spawned them so we must actually see this false it's not like we could see a value written somewhere else independently but we must see this false or anything that happens later we can't see if imagine that this thread did like x dot store true then the loads down here must see the store because it happened before them.

##  Acquire and Release Semantics

Section Overview: In this section, the speaker talks about acquire and release semantics.

### Acquire and Release Semantics

- The speaker explains that if you observe a value from acquire you will see all operations that happen before the corresponding release store while the corresponding release store is here in this thread there are no operations prior to the store but if there were we would be guaranteed to see them here because we're synchronizing with tx but there are none.
- This load synchronizes with this store so after this load we must see all memory operations that happened before this store. There's no requirement that that's any particular store of why if there had been a store of y in this thread if this did y dot store we must observe that y dot store because it happened strictly before this store which happened before this load because of acquire release so if there was a store here we must see it but there isn't so we're allowed to see any value for y.
##  Memory Ordering in Rust

Section Overview: This section discusses memory ordering in Rust and how it affects concurrent operations.

### Happens Before Relationship

-  When a thread runs, it synchronizes with other threads but there is no requirement that it sees any particular operation that happened to x because there's no happens before relationship between the store here in tx and the load down here.
-  Acquire loads says that you're not allowed to move any operation from below to above an acquire load so the compiler is not allowed to reorder these by the acquire semantics.
-  The load is allowed to see any previous value for x subject to happens before which does not include the operation of t y so therefore uh t2 must see this.

### Concurrent Operations

-  T2 can see either of these and T1 can see either of these. If both threads run, then T1 runs, observes x being true but does not observe y being true even though T1 has run because there's no happens before relationship there.
-  Imagine that it's already in cache in the CPU or something; it just uses that value. It doesn't bother to check again because it's allowed not to. Therefore, this value is false even though Tx Ty ran; therefore, it does not increment one this one for some reason like it spins until y is true.
-  It observes that y is true great. It's no requirement that it observes that x is true even though Tx has run so it does not increment c so therefore z is zero.

### Sequentially Consistent Operation

-  Sexy as t (pronounced "sequentially consistent") means acquire and release and acquire release with the additional guarantee that all threads see all sequentially consistent operations in the same order. If we make these all be sequentially consistent now, zero is no longer possible because if this thread observes that x observes x is true and then y is true, it establishes a happens before relationship.
##  Memory Ordering and Sequential Consistency

Section Overview: This section discusses memory ordering and sequential consistency in concurrent programming.

### Memory Ordering

- In concurrent programming, there must exist some ordering that's consistent across all threads.
- If some thread sees that x happened then y happened, then no thread is allowed to see x not happen even though y has happened.
- If we end up with x is true and then y is true, then here y is true therefore x must be true. It's not allowed to see x being false because that would be inconsistent with the memory ordering that these sequentially consistent operations saw.
- The sequentially consistent ordering is only with relation to all other sequentially consistent operations.
- Sequentially consistent only really interacts with other sequentially consistent acquire release does interact with sequentially consistent so sequentially consistent it is always stronger than acquire release.

### Testing Concurrent Code

- Memory ordering is real subtle and hard to get right.
- Running your code lots of times in a loop or making your computer busy with other tasks can help test for errors but it's not reliable since it might depend on the architecture, operating system scheduler, compiler optimizations, or how likely the error occurs.
- Detecting bugs in concurrent code can be difficult since they may not cause a program crash or may go unnoticed for a long time.
##  Detecting Concurrency Problems

Section Overview: This section discusses the need to detect concurrency problems and some automated systems that can help with this.

### Thread Sanitizer

-  Google has built-in sanitizers in compilers like Clang, GCC, and MSVC.
-  The thread sanitizer is one of these sanitizers.
-  It detects unsynchronized operations on a single memory location by tracking every load and store operation in a program.
-  The algorithm for the thread sanitizer is specified in the documentation.

### Loom

-  Loom is a tool that implements the CDS Checker paper for Rust.
-  It takes a concurrent program, instruments it using loom synchronization and atomic types instead of standard library ones, and runs it multiple times to expose it to different possible executions.
-  Loom executes all possible thread interleavings and memory orderings to ensure that all legal executions of the program are explored.
##  Loom: A Tool for Testing Concurrent Code

Section Overview: In this section, the speaker talks about Loom, a tool used to test concurrent code. The speaker explains that Loom explores all possible legal executions of threads and implements a bunch of implementation things from the paper to reduce the number of possible executions.

### Exploring All Possible Legal Executions

-  Loom explores all possible legal executions of threads.
-  There might be an insane number of such possible executions.
-  Loom has a bunch of implementation things that are from the paper that looks at reducing those.

### Limitations of Loom

-  Ultimately, there's like a limit to how complex your test cases can be when you run them on the loop loom.
-  Even so, loom is the best tool I know of to try to make sure that your concurrent code is actually correct under only the assumptions that the memory model gives you rather than the assumptions that the current compiler or optimization or CPU might give you.

##  Examples for Sequentially Consistent Ordering

Section Overview: In this section, someone asks if there are any toy programs one can think of to try and drill sequentially consistent ordering into their head. The speaker recommends looking at some papers that implement concurrent data structures and notice where they use sequentially consistent ordering as opposed to other orderings.

### No Less Contrived Example Available

-  The speaker doesn't have a less contrived example for sequentially consistent ordering.

### Recommendation

-  Look at some papers that implement concurrent data structures and notice where they use sequentially consistent ordering as opposed to other orderings.
-  Generally, the paper will explain why.

##  Limitations of Zulu

Section Overview: In this section, the speaker talks about some limitations of Zulu and how relaxed ordering is so relaxed that you can't fully model all possible executions even with something like Loom.

### Limitations of Zulu

-  Zulu has some limitations, some of them are known problems, some of them are more fundamental problems with the approach or are just impossible to model.
-  Relaxed ordering is so relaxed that you can't fully model all possible executions even with something like Loom.

### Example for Relaxed Ordering

-  The speaker gives an example where loom has to produce some value for a load but doesn't know about it yet because it hasn't happened yet.
##  Contributing to Loom

Section Overview: This section discusses the benefits of contributing to Loom, a project that deals with concurrency. It also highlights some of the limitations of Loom and how contributions can help improve it.

### Benefits of Contributing to Loom

-  The speaker encourages people interested in concurrency to contribute to Loom.
-  The documentation for Loom has been recently updated and is recommended reading for anyone interested in using or contributing to the project.
-  Tokyo uses Loom and has caught several critical bugs, making it an important tool for detecting issues.

### Limitations of Loom

-  Currently, Loom does not model sequentially consistent ordering correctly.
-  Implementing the additional guarantees required for sequentially consistent ordering is complex.
-  Downgrading every sequentially consistent order to acquire release means that while Loom won't miss any bugs, it might give false positives.

### How Contributions Can Help

-  There is an open issue and test case for modeling sequentially consistent ordering correctly that needs fixing.
-  Contributions towards improving documentation are highly encouraged.

##  Testing Sequentially Consistent Ordering with Acquire Release

Section Overview: This section explains how testing sequential consistency with acquire release works in Loom.

### Testing Sequential Consistency with Acquire Release

-  If code depends on sequential consistency but is tested using acquire release, there may be false positives when using Loom.
-  However, running code through Loom will catch real errors and explore possible cases with acquire release.
##  Overview of Loom and Relaxed Ordering

Section Overview: In this section, the speaker provides an overview of Loom and relaxed ordering.

### Atomic Module in Sender Library

-  The atomic module in the sender library contains atomic types, constants, and three functions: spin loop hint, compiler fence, and fence.
-  Spin loop hint is deprecated and has been moved to the hint module. It is used in atomic contexts such as a spin lock.
-  Compiler fence ensures that the compiler won't move a given load or store operation above or below a fence within that thread's execution. It is mostly used for preventing a thread from racing with itself when using signal handlers.
-  Fence establishes a happens before relationship between two threads without talking about a particular memory location. It synchronizes with all other threads that are doing an offense.

### Memory Ordering

-  Load acquire synchronizes or happens after store release of the same memory location.
-  Fence is basically an atomic operation that establishes a happens before relationship between two threads but without talking about a particular memory location.

##  Volatile Keyword

Section Overview: In this section, the speaker explains why there is no mention of volatile in the atomic module.

### Volatile Keyword

- There is no mention of volatile in the atomic module because it is just unrelated to atomics.
##  Understanding Volatile

Section Overview: In this section, the speaker explains what volatile is and its purpose.

### What is Volatile?

-  Volatile is used when interacting with memory-mapped devices.
-  It ensures that operations cannot be reordered relative to other operations.
-  If you read twice from a given variable in memory map memory without using read volatile, the compiler would cache the first read into register and just read from the register for the second read. However, both reads need to go to memory to have a side effect on device map memory.

##  Atomic Pointer

Section Overview: In this section, the speaker explains atomic pointer and how it differs from other types.

## What is Atomic Pointer?

-  Atomic pointer is an atomic usize where methods are specialized to the underlying type being a pointer.
-  It provides load, store, compare exchange and fetch update which is like having a nicer interface to doing a compare exchange loop.
   [_CUTOFF_LIMIT_]

# Generated by Video Highlight
https://videohighlight.com/video/summary/rMGWeSjctlY