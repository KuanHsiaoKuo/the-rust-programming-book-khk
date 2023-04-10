# Async/Await in Rust

<!--ts-->
* [Async/Await in Rust](#asyncawait-in-rust)
   * [Introduction](#introduction)
      * [Speaker's Introduction](#speakers-introduction)
   * [Async Await in Rust](#async-await-in-rust)
      * [Understanding Async Await](#understanding-async-await)
   * [Additional Resources](#additional-resources)
      * [Recommended Books](#recommended-books)
      * [Other Resources](#other-resources)
   * [Focusing on Tokio Executor](#focusing-on-tokio-executor)
      * [Using Tokio Executor](#using-tokio-executor)
   * [Conclusion](#conclusion)
      * [Final Thoughts](#final-thoughts)
   * [Introduction to Async Functions](#introduction-to-async-functions)
      * [What is an Async Function?](#what-is-an-async-function)
      * [How Async Functions Work](#how-async-functions-work)
   * [Example of Using Async Functions](#example-of-using-async-functions)
      * [Handling Long-Running Operations](#handling-long-running-operations)
      * [Example Code](#example-code)
   * [Async Blocks](#async-blocks)
      * [Async Blocks](#async-blocks-1)
   * [Mental Model of Async](#mental-model-of-async)
      * [Mental Model of Async](#mental-model-of-async-1)
   * [Promises vs Futures](#promises-vs-futures)
      * [Promises vs Futures](#promises-vs-futures-1)
   * [Understanding Async Programming in Rust](#understanding-async-programming-in-rust)
      * [How Async Programming Works](#how-async-programming-works)
      * [Why Use Async Programming?](#why-use-async-programming)
      * [Conclusion](#conclusion-1)
   * [Introduction to Select Macro](#introduction-to-select-macro)
      * [Using Select Macro with Futures](#using-select-macro-with-futures)
   * [Benefits of Async Await](#benefits-of-async-await)
      * [Benefits of Async Await](#benefits-of-async-await-1)
   * [Cooperative Scheduling and Cancellation](#cooperative-scheduling-and-cancellation)
      * [Cooperative Scheduling](#cooperative-scheduling)
      * [Cancellation](#cancellation)
   * [Understanding Async in Rust](#understanding-async-in-rust)
      * [Cancellation](#cancellation-1)
      * [Racing Futures Against Each Other](#racing-futures-against-each-other)
      * [Understanding Executors](#understanding-executors)
      * [Epoll Loop](#epoll-loop)
      * [Tokyo's Use of Event Loops](#tokyos-use-of-event-loops)
      * [Selecting Among Options in a Loop](#selecting-among-options-in-a-loop)
      * [Abandoned Select Arm Side Effects](#abandoned-select-arm-side-effects)
   * [Select and Cooperatively Scheduled World](#select-and-cooperatively-scheduled-world)
      * [Potential Issues with Select](#potential-issues-with-select)
      * [Importance of Cooperatively Scheduling Tasks](#importance-of-cooperatively-scheduling-tasks)
   * [Future Polling](#future-polling)
      * [Future Polling and Select](#future-polling-and-select)
   * [Select Macro and its Implementation](#select-macro-and-its-implementation)
      * [Smart Select](#smart-select)
      * [Fairness of Select Macro](#fairness-of-select-macro)
      * [Fuse](#fuse)
   * [Asynchronous IO and the Benefits of Async](#asynchronous-io-and-the-benefits-of-async)
      * [Async Keyword](#async-keyword)
      * [Overhead of Asynchronous IO Read](#overhead-of-asynchronous-io-read)
      * [Benefits of Async](#benefits-of-async)
   * [Joining Futures with Join Operations](#joining-futures-with-join-operations)
      * [Example: Waiting for Multiple Files](#example-waiting-for-multiple-files)
      * [Join Macro](#join-macro)
      * [Wrapping into Future Type](#wrapping-into-future-type)
      * [Return Value](#return-value)
   * [Joining Files with the Join Macro](#joining-files-with-the-join-macro)
      * [Using Join Macro](#using-join-macro)
      * [Benefits of Using Join Macro](#benefits-of-using-join-macro)
      * [Downsides of Not Using Join Macro](#downsides-of-not-using-join-macro)
   * [Overview of Futures in Rust](#overview-of-futures-in-rust)
      * [How Futures Work](#how-futures-work)
      * [Concurrency vs Parallelism](#concurrency-vs-parallelism)
      * [Example: Writing a Web Server](#example-writing-a-web-server)
   * [Handling Multiple Connections](#handling-multiple-connections)
      * [Using Futures Stream and Select](#using-futures-stream-and-select)
      * [Awaiting on Futures](#awaiting-on-futures)
      * [Introducing Parallelism with Spawn](#introducing-parallelism-with-spawn)
   * [Introduction to Parallelism in Asynchronous Programs](#introduction-to-parallelism-in-asynchronous-programs)
      * [Communicating Futures for Parallel Execution](#communicating-futures-for-parallel-execution)
   * [Importance of Spawning and Performance Issues](#importance-of-spawning-and-performance-issues)
      * [Remembering to Spawn and Program Performance](#remembering-to-spawn-and-program-performance)
   * [Best Practices for Passing Data and Handling Errors with Tokyo Spawn](#best-practices-for-passing-data-and-handling-errors-with-tokyo-spawn)
      * [Passing Data with Tokyo Spawn](#passing-data-with-tokyo-spawn)
      * [Handling Errors with Tokyo Spawn](#handling-errors-with-tokyo-spawn)
   * [Spawning and Running Futures](#spawning-and-running-futures)
      * [Advantages of Spawning Futures](#advantages-of-spawning-futures)
      * [How to Spawn Futures in Tokio](#how-to-spawn-futures-in-tokio)
      * [Blocking Operations](#blocking-operations)
   * [Rust Futures and Thread Locals](#rust-futures-and-thread-locals)
      * [Thread Locals in Tokio](#thread-locals-in-tokio)
      * [Rust Futures without Thread Locals](#rust-futures-without-thread-locals)
   * [Introduction](#introduction-1)
   * [What is a Future?](#what-is-a-future)
   * [Example Async Function](#example-async-function)
   * [Understanding Impul Futures](#understanding-impul-futures)
      * [Impul Future Type](#impul-future-type)
      * [State Machine Type](#state-machine-type)
      * [Passing Futures Around](#passing-futures-around)
      * [Using Tokyo Spawn](#using-tokyo-spawn)
   * [Introduction](#introduction-2)
      * [Key Points](#key-points)
   * [Async Trait](#async-trait)
      * [Key Points](#key-points-1)
   * [Async Trait and Associated Types](#async-trait-and-associated-types)
      * [Async Trait vs. Associated Types](#async-trait-vs-associated-types)
      * [Naming Associated Types](#naming-associated-types)
   * [Challenges with Implementing Async Functions in Traits](#challenges-with-implementing-async-functions-in-traits)
      * [Compiler Code Generation](#compiler-code-generation)
   * [Asynchronous Programming in Rust](#asynchronous-programming-in-rust)
      * [Key Points](#key-points-2)
   * [Standard Library Mutexes vs Asynchronously Enabled Mutexes](#standard-library-mutexes-vs-asynchronously-enabled-mutexes)
      * [Key Points](#key-points-3)
   * [Using Async-Aware Mutexes](#using-async-aware-mutexes)
      * [Standard Library Mutex vs Async-Aware Mutex](#standard-library-mutex-vs-async-aware-mutex)
      * [Deadlock Potential](#deadlock-potential)
      * [Detecting Problems](#detecting-problems)
   * [Thread Spawn vs Tokyo Spawn](#thread-spawn-vs-tokyo-spawn)
      * [Thread Spawn](#thread-spawn)
      * [Tokyo Spawn](#tokyo-spawn)
   * [Introduction](#introduction-3)
      * [Async and Await](#async-and-await)
   * [Scheduling Futures](#scheduling-futures)
      * [Awaiting Inside a Future](#awaiting-inside-a-future)
      * [Single Threaded Executors](#single-threaded-executors)
      * [Enforcing Static Guarantees](#enforcing-static-guarantees)
   * [Async Stack Trace](#async-stack-trace)
      * [Getting an Async Stack Trace](#getting-an-async-stack-trace)
   * [Tracing Futures](#tracing-futures)
      * [Using instrument to Trace Futures](#using-instrument-to-trace-futures)
      * [Best Practices for Calling Async Code from Synchronous Code](#best-practices-for-calling-async-code-from-synchronous-code)
      * [Use Cases for Threads Instead of Futures](#use-cases-for-threads-instead-of-futures)
      * [When Not To Use Async](#when-not-to-use-async)
   * [Generated by Video Highlight](#generated-by-video-highlight)

<!-- Created by https://github.com/ekalinin/github-markdown-toc -->
<!-- Added by: runner, at: Mon Apr 10 14:43:17 UTC 2023 -->

<!--te-->

## Introduction

Section Overview: In this section, the speaker introduces himself and explains that he will be discussing async await in
Rust.

### Speaker's Introduction

- The speaker welcomes viewers back after a long break.
- He mentions that he has been on vacation and briefly talks about it.
- The speaker announces that this will be another "Crust of Rust" stream.
- He states that he will be discussing async await in Rust.

## Async Await in Rust

Section Overview: In this section, the speaker discusses async await in Rust from a usability standpoint.

### Understanding Async Await

- The speaker acknowledges that he has covered async await before but only from a technical standpoint.
- He explains that he wants to discuss how to use async await and what it looks like for developers.
- The speaker emphasizes the importance of understanding the intuition and mental model behind async await.

## Additional Resources

Section Overview: In this section, the speaker provides additional resources for learning about async programming in
Rust.

### Recommended Books

- The speaker recommends reading "The Rust Programming Language" book if you haven't already.
- He mentions that there is no chapter on async programming in the book but one may be added soon.
- The speaker also recommends his own book which covers async programming at a deeper level than basic tutorials.

### Other Resources

- The speaker recommends checking out "The Rust Async Book" which explains asynchronous programming in Rust but still
  has some work to do.
- He also suggests looking into Mini Redis, a project by Tokyo Project which demonstrates how to structure an
  asynchronous application client-side and server-side.

## Focusing on Tokio Executor

Section Overview: In this section, the speaker focuses on using Tokio executor for asynchronous code.

### Using Tokio Executor

- The speaker mentions that there are many different executors for asynchronous code but he will be focusing on Tokio
  because it is the most widely used and actively maintained one.
- He clarifies that most of what he will be discussing is independent of the executor.

## Conclusion

Section Overview: In this section, the speaker concludes his discussion on async await in Rust.

### Final Thoughts

- The speaker fixes an error in the video title.
- He thanks viewers for watching and encourages them to follow him on Twitter for updates on upcoming videos.
- The speaker emphasizes the importance of understanding async programming in Rust and provides resources for further
  learning.

## Introduction to Async Functions

Section Overview: In this section, the speaker introduces async functions and explains how they work.

### What is an Async Function?

- An async function is a function that returns a future.
- The future trait signifies a value that's not ready yet but will eventually be of a certain type.
- Awaiting on a future means not running the following instructions until it resolves into its output type.

### How Async Functions Work

- A future does nothing until it is awaited.
- When awaited, the future describes a series of steps that will be executed at some point in the future.
- If an async function calls another async function, it awaits the returned future before continuing with its own
  execution.

## Example of Using Async Functions

Section Overview: In this section, the speaker provides an example of using async functions and explains how they can be
used to handle long-running operations.

### Handling Long-Running Operations

- Reading from a file can take time and block other operations.
- By using async functions, we can avoid blocking other operations while waiting for long-running tasks to complete.
- We can use await to pause execution until the task completes without blocking other operations.

### Example Code

- The speaker provides an example code snippet that demonstrates how to use async functions to read from a file without
  blocking other operations.

## Async Blocks

Section Overview: This section explains how async blocks execute in chunks and yield when they have to wait.

### Async Blocks

- Async blocks execute in chunks.
- They yield when they have to wait, allowing other things to run.
- A future is checked for progress towards completion.
- When a future makes progress, it continues executing from the last point it yielded until the next point where it
  might fail to make progress.

## Mental Model of Async

Section Overview: This section provides a mental model of async that helps understand how async works.

### Mental Model of Async

- The mental model is that async lets you chunk computation.
- It runs until it has to wait and then yields, letting other things run.
- Once an await resolves, execution continues as if there were no async until another future is created and awaited.

## Promises vs Futures

Section Overview: This section compares promises and futures.

### Promises vs Futures

- A future can be thought of as a promise in terms of the word "promise," not in terms of the actual JavaScript type
  Promise.

## Understanding Async Programming in Rust

Section Overview: In this section, the speaker explains how async programming works and why it is useful.

### How Async Programming Works

- Async programming involves waiting for multiple operations to complete without blocking the main thread.
- Yielding allows you to return all the way up to where a future was first awaited.
- This approach is useful when you have many futures that you want to wait on simultaneously.

### Why Use Async Programming?

- Traditional threading systems require one thread for each operation, which can be difficult to manage.
- Async programming allows you to handle multiple operations with a single thread, making it easier to manage and more
  efficient.
- Using async programming also allows you to handle slow connections without getting stuck writing to them.

### Conclusion

- Async programming is a powerful tool for managing multiple operations efficiently and effectively. By using async
  programming, developers can avoid the pitfalls of traditional threading systems and create more robust applications.

## Introduction to Select Macro

Section Overview: In this section, the speaker introduces the select macro and how it can be used with futures.

### Using Select Macro with Futures

- The select macro waits on multiple futures and tells you which one finished first.
- If progress is made on network, then a stream is given and the code inside is called. If not, progress is tried on the
  terminal.
- When neither of them make progress, it yields and retries later using some operating system primitive under the hood
  to be smarter about when it tries again.
- Async allows you to express waiting for something else to happen while giving up thread's time.

## Benefits of Async Await

Section Overview: In this section, the speaker explains how async await can be used as a state machine and its benefits.

### Benefits of Async Await

- Async allows something else to run instead while waiting for disk or network operations.
- Async await can be thought of as a big state machine where there are multiple paths forward depending on what becomes
  available on sockets.
- Select operation allows branching execution but in this case, there's no real branching.

## Cooperative Scheduling and Cancellation

Section Overview: This section discusses cooperative scheduling and cancellation in Rust.

### Cooperative Scheduling

- `async` functions allow for cooperative scheduling, where execution is yielded to other tasks when a function
  encounters an `await` statement.
- The `select!` macro allows for non-blocking I/O operations by selecting the first future that completes.
- When a future is awaited, it does not execute until it is explicitly called with an `await` statement.
- If there are no other tasks to run, the current task will continue executing until it reaches another `await`
  statement or yields control back to the scheduler.

### Cancellation

- Futures can be cancelled by dropping them if they have not been awaited yet.
- Once a future has been awaited, its execution cannot be cancelled directly.
- To cancel a future's execution, you can use the `select!` macro with a timeout value or a cancellation token.

## Understanding Async in Rust

Section Overview: In this section, the speaker explains how async works in Rust and describes the mechanisms for
cooperatively scheduling a bunch of computation by describing when under what circumstances code can make progress and
under what circumstances code can yield.

### Cancellation

- The speaker explains that cancellation is done by describing the circumstances under which you should cancel the
  operation.
- A good example of cancellation is given as a cancellation channel which is like a Tokyo sync mpsc receiver.
- The cancel function is defined as cancel.08.
- If we get a weight, we fall through to print line below.

### Racing Futures Against Each Other

- The speaker proposes another name for select, which is race.
- Whichever await can produce its result first gets to run while the other one does not.
- If we get a weight, we return zero and don't execute any of the remainder.

### Understanding Executors

- An executor provides both the lowest level resources like network sockets and timers and also the executor loop at the
  top.
- At some point in your program, you're going to have this giant future that holds the entire execution of your program
  but it's a future so nothing runs yet right?
- You're only allowed to await in async functions and async blocks so this wouldn't actually compile if this was an
  async fn but then you see compiler complaints main function is not allowed to be async because ultimately at some
  point in your program you're going to have this giant future that holds the entire execution of your program.
- The executor crate provides both the lowest level resources like network sockets and timers and also the executor loop
  at the top.

### Epoll Loop

- An executor is a sort of primitive executor that just pulls futures in a loop and does nothing else it just keeps
  retrying aggressively instead.
- Imagine that you're doing a read from the network and you call dot await what's going to actually happen is Tokyo's
  tcp stream is going to do a read from the network realize that I can't read yet and then it's going to sort of store
  its file descriptor store the socket id if you will into the executor's sort of internal state.
- The main executor takes all those resources that it knows it needs to wait for, sends them to the operating system,
  puts itself to sleep but wakes up if any of these change because it has more work to do.
  I'm sorry, but I cannot provide a summary of the transcript without having access to it. Please provide me with the
  transcript so that I can create a summary for you.

## [#](t=0:47:15s) Tokyo's Use of Event Loops

Section Overview: This section discusses how Tokyo uses event loops and the Mayo crate to abstract over kq, epoll, and
other operating system event loops.

### Tokyo's Use of Event Loops

- [](t=0:47:15s) Tokyo does not re-implement kq but instead uses the Mayo crate to abstract over kq and other operating
  system event loops.
- [](t=0:47:45s) The Mayo crate allows you to register events with the operating system and go to sleep until any of
  those events occur. The operating system will then wake you up when one of those events happens.
- [](t=0:48:49s) You can use the Futures channel with Tokyo because they use the same underlying mechanisms that Rust
  provides.

## [#](t=0:49:27s) Selecting Among Options in a Loop

Section Overview: This section discusses how select works in a loop and what happens if an abandoned select arm has side
effects.

### Selecting Among Options in a Loop

- [](t=0:49:27s) When using select, it selects among all given options every time. It doesn't remember anything about
  having been run in the past.
- [](t=0:50:18s) To reuse network across multiple iterations of the loop, you need to do something like this:

```
let mut network = network;
select! {
    _ = terminal => { ... },
    _ = foo => { ... },
    _ = &mut network => { ... },
}
```

### Abandoned Select Arm Side Effects

- [](t=0:51:09s) If an abandoned select arm has side effects, such as file copying, it will still execute those side
  effects even if it is not selected.

## Select and Cooperatively Scheduled World

Section Overview: This section discusses the potential issues that arise when using select in an asynchronous context.
It also explains the importance of cooperatively scheduling tasks to avoid blocking other futures.

### Potential Issues with Select

- When using select, it's possible to be in a state where some bytes have been copied from one future to another but not
  all, which can lead to errors if not handled properly.
- If a future is partially completed and then dropped, it won't get to finish its work and will be cancelled. This means
  that your program might be in an intermediate state that needs to be considered.
- You need to be careful about what happens if one branch runs and then another branch completes so the first branch
  didn't run to completion but the second branch did where does that leave you? This is an error case that you need to
  be concerned about when you're using select.

### Importance of Cooperatively Scheduling Tasks

- In an asynchronous context, it's important for tasks to yield occasionally so that other things can run as well. If a
  future never yields, it can block other futures from making progress.
- Using blocking operations (operations that aren't aware of async yielding or very compute-intensive operations) can
  cause threads to block and prevent other futures from making progress. It's important to use methods
  like `spawn_blocking` or `block_in_place` in Tokyo for long-blocking operations so that the executor knows how to
  handle them appropriately.

## Future Polling

Section Overview: This section discusses whether select with a huge number of cases will potentially slow down by trying
all options out every time.

### Future Polling and Select

- The implementation of select determines whether a select with a huge number of cases will be slowed down by trying all
  options out every time.
- In general, if you have a select with a million cases, it seems like it would be slow. However, the answer depends on
  the implementation of select.

## Select Macro and its Implementation

Section Overview: In this section, the speaker talks about the select macro and how it can be implemented to optimize
for few cases rather than many cases. The speaker also explains how select can integrate with Rust machinery for dealing
with futures and wakeups.

### Smart Select

- The select macro can be smart in the sense of only polling the ones that might be able to make progress.
- When a future becomes runnable through whatever mechanism like a file descriptor was ready or a send happened on an
  in-memory channel, there's a way for the select macro to sort of inject itself into that notification that this future
  is ready and sort of get a signal to update its own state when that happens.
- The select keeps almost like a bit mask across all of its branches, and when that notification comes in, it flips the
  bit for the appropriate branch. Then, next time the select is awaited, it'll only check the ones where the bit is set.

### Fairness of Select Macro

- It depends on which implementation you use. For example, if you look at the futures crate, it has a select biased
  variant that always runs if multiple are ready; it runs the first one so that would not be fair.

### Fuse

- Fuse is a way to say that it's safe to pull or await a future even if that future has already completed in the past.
- In general, you will want to select on mutable references to futures rather than on futures themselves because if you
  await on futures directly then they would just be dropped at end of selection.

## Asynchronous IO and the Benefits of Async

Section Overview: This section discusses the benefits of using async in Rust, particularly for IO-bound tasks.

### Async Keyword

- Adding the `async` keyword to a function that implements matrix multiplication does not make it more expensive.
- The code runs the same way but is wrapped in a future type, which requires waiting for results with `await`.
- There is no overhead to marking something as async since it doesn't change generated code.

### Overhead of Asynchronous IO Read

- There is some overhead when doing an asynchronous IO read compared to synchronous IO read.
- Additional system calls get amortized across all resources, so there's not much added.
- Fewer threads run by the executor and cooperatively scheduled are usually faster than having many threads that cross
  operating system boundaries.

### Benefits of Async

- Async leads to more efficient use of resources for IO-bound tasks like web servers.
- It also makes code easier to read but not necessarily easier to reason about.
- User space threads are one way to think about async.

## Joining Futures with Join Operations

Section Overview: This section discusses how join operations work in Rust and their usefulness when waiting for multiple
futures to complete.

### Example: Waiting for Multiple Files

- An example scenario where join operations are useful is when waiting for multiple files to complete before continuing
  with a program.
- The join operation waits until all futures complete before continuing with the program.

### Join Macro

- In the futures crate, there is a join macro that lists all futures you want to wait on without specifying branches.
- It looks similar to select macro except you don't specify branches.

### Wrapping into Future Type

- You can construct a select manually too but it's usually more hassle.
- Select expands into basically a loop with a match or a bunch of ifs.
- You could make select a future by wrapping it in an async block and assigning that to a variable.

### Return Value

- Join doesn't return anything meaningful apart from a future.
- It's possible to construct the select future for a block yourself, but it's not quite true that select generates a
  future.

## Joining Files with the Join Macro

Section Overview: In this section, the speaker explains how to use the join macro to read multiple files concurrently
and efficiently.

### Using Join Macro

- The join macro allows operations to continue concurrently, which is more efficient.
- It lets you overlap compute and I/O.
- You need to explicitly enumerate all the things you're joining.
- There are multiple functions like join which joins two things joint three joint four joint five etc.
- Try join all takes an iterator over things that are futures whose output is a result. It will make sure that the
  result output is in the same order as the input even if they completed out of order.

### Benefits of Using Join Macro

- Allows operations to continue concurrently, which is more efficient.
- Lets you overlap compute and I/O.

### Downsides of Not Using Join Macro

- Sequential execution does not allow for concurrent operations.
- Does not give the operating system all read operations at once, making it less efficient.

## Overview of Futures in Rust

Section Overview: In this section, the speaker discusses how futures work in Rust and how they can be used to run
operations concurrently.

### How Futures Work

- The output of a future describes which input it was from, making it more efficient.
- Join operations like futures unordered implement a hook into the runtime system to only check the futures that might
  be ready.

### Concurrency vs Parallelism

- Only one thread can await one future at any time because awaiting a future requires an immutable reference to it.
  Multiple threads cannot do it.
- Even if you use multiple threads, all operations are still happening concurrently on one thread, which is not what you
  want for optimal performance.

### Example: Writing a Web Server

- If you're writing a web server and have a loop over accepting TCP connections, using futures unordered will not give
  optimal performance since all operations are still happening concurrently on one thread.

## Handling Multiple Connections

Section Overview: In this section, the speaker discusses how to handle multiple connections in Rust using Tokio.

### Using Futures Stream and Select

- The speaker creates a `let mute connections` variable that is a `futures stream future maybe futures unordered new`.
- The speaker then uses `select` to either get a new stream or call an async function called `handle connection`.
- If there are new streams, the speaker pushes them into the `connections` variable
  using `connections.push(handle connection of stream)`.

### Awaiting on Futures

- The async function `handle connection` takes a TCP stream and does something with it.
- To ensure all futures run concurrently, the speaker needs to await on both the accepting and client connections.
- However, having only one top-level future means that even if there are many threads, only one thread can run at a
  time.

### Introducing Parallelism with Spawn

- To introduce parallelism, the speaker gets rid of the `connections` variable and replaces it with a while loop.
- The speaker then uses Tokio's provided function called spawn which gives additional futures to work on.
- This allows for two separate futures which means that two threads can run them at the same time.

## Introduction to Parallelism in Asynchronous Programs

Section Overview: This section covers how to introduce parallelism into asynchronous programs by communicating futures
that can run in parallel to the executor. The importance of using this pattern is highlighted, as it allows for futures
to run concurrently on multiple threads.

### Communicating Futures for Parallel Execution

- To introduce parallelism into asynchronous programs, communicate futures that can run in parallel to the executor.
- If the executor doesn't have many threads, there may not be much parallelism.
- Using this pattern allows for futures to run concurrently on multiple threads.

## Importance of Spawning and Performance Issues

Section Overview: This section discusses why it's important to remember to spawn when writing async await code. It also
highlights how program performance can drop significantly if nothing is spawned, causing the entire application to run
on one thread.

### Remembering to Spawn and Program Performance

- When writing async await code, it's important to remember to spawn.
- If nothing is spawned, program performance can drop significantly.
- The entire application will run on one thread without spawning anything.
- Running an entire application on one thread is slower than running it on multiple threads because nothing gets to run
  in parallel.

## Best Practices for Passing Data and Handling Errors with Tokyo Spawn

Section Overview: This section covers best practices for passing data and handling errors with Tokyo Spawn. Techniques
such as using mutexes or channels are discussed, along with how errors should be handled when they occur.

### Passing Data with Tokyo Spawn

- To share data between two things that are spawned and want them both accessing a vector, use arc new mutex new.
- Each thing gets its own arc and has a mutex regarding the underlying value.
- Communication over a channel or static memory is also possible.

### Handling Errors with Tokyo Spawn

- If an error occurs in a spawned future and there is nowhere to communicate it, print it to standard out or log it to a
  file.
- Use an event distribution tool like tracing to emit an error event that gets handled somewhere else.

## Spawning and Running Futures

Section Overview: This section discusses the advantages of spawning futures and how to do it in Tokio.

### Advantages of Spawning Futures

- Spawning a future allows other tasks on the current thread to keep running while that operation is running elsewhere.
- This can be valuable when you have an expensive operation like deserialization or hashing a password that you don't
  want to block async execution of a thread.

### How to Spawn Futures in Tokio

- Use `tokio::spawn` to spawn a future.
- Thread locals are used by Tokio to find the current runtime and executor.
- Multiple runtimes can be created, each with its own thread count, allowing for prioritized traffic handling.

### Blocking Operations

- Use `tokio::task::spawn_blocking` or `tokio::task::block_in_place` for blocking operations like hashing passwords.

## Rust Futures and Thread Locals

Section Overview: This section explains how Rust futures work with thread locals and why they are used in Tokio.

### Thread Locals in Tokio

- Thread locals are used by Tokio to make the interface slightly nicer.
- Without thread locals, `spawn` would not be a free function, requiring the handle to the runtime throughout your
  entire application.

### Rust Futures without Thread Locals

- There is nothing in Rust's async support that requires thread locals.

## Introduction

Section Overview: In this section, the speaker introduces the topic of creating a runtime and explains that it does not
allocate a lot of memory.

## What is a Future?

Section Overview: In this section, the speaker explains what a future is and how it works in an async function.

- A future is a chunked computation with weights in between each chunk.
- Each chunk contains the state for that part of the state machine.
- Local variables that need to be kept across await points are saved as part of the state.
- The compiler generates an enum-like structure to represent the state machine.
- The future itself needs to be stored somewhere because it holds a reference into x.

## Example Async Function

Section Overview: In this section, the speaker provides an example async function and explains how it gets divided up
into chunks.

- The async function has one chunk that starts at its beginning and ends before any await points.
- Between each chunk is a weight operation represented by an await point.
- Local variables that need to be kept across await points are saved as part of the state.

## Understanding Impul Futures

Section Overview: In this section, the speaker explains how Impul futures work and how they are implemented in Rust.

### Impul Future Type

- An `Impul Future` is a type that returns an `Impul Future`.
- The `Impul Future` is actually a type of state machine.
- The state machine contains all the values kept across await points.
- When we await an `Impul Future`, we are invoking a method on the state machine type that gets an exclusive reference
  immutable reference to that type so it can access the future and continue to wait.

### State Machine Type

- The generated state machine type is a trait with a name but no specified name.
- It contains internal local variables and references to other futures.
- This conversion from async fn rewrites these variables as references into the state machine struct that we pass
  around.

### Passing Futures Around

- Futures can get really large because they contain all of the futures they in turn await.
- Passing them around requires mem copying which can be expensive for large futures.
- Boxing your features solves this problem by placing them on the heap or using box pin.

### Using Tokyo Spawn

- Using something like Tokyo spawn is useful because it stores only a pointer to the future instead of storing the
  entire future itself.
- This reduces mem copies and avoids having futures grow indefinitely.

## Introduction

Section Overview: In this section, the speaker introduces the topic of futures and async functions in Rust.

### Key Points

- The speaker briefly discusses why futures can be moved even though they may need to be pinned.
- The speaker explains that it is possible to create a vector of futures recursively within an async function by using a
  join handle.
- However, the speaker notes that using a vector is not enough because an array needs to know the size of its elements,
  which may not be known for all futures.
- The speaker mentions that there is an async trait crate available for writing asynchronous functions within traits.

## Async Trait

Section Overview: In this section, the speaker discusses the limitations of writing asynchronous functions within traits
in Rust.

### Key Points

- The speaker explains that currently, it is not possible to define an async function within a trait in Rust.
- This is because the size of the future returned by the function cannot be determined at compile time.
- The speaker provides an example where defining an async function within a trait would result in an error due to
  unknown future size.
- The speaker mentions that there are two ways around this limitation: using existential types or using the async trait
  crate.

## Async Trait and Associated Types

Section Overview: In this section, the speaker discusses the use of async trait and associated types in Rust
programming. They explain how async trait works for higher-level functions but may not work well at the bottom of a
stack due to heap allocation and dynamic dispatch.

### Async Trait vs. Associated Types

- Async trait rewrites signatures and bodies into an async move, which works if you remove async trait.
- Rust already knows how to reason about dynamically dispatched futures, making it easy to await them.
- Heap allocating all futures leads to dynamic dispatch, indirection, and memory allocator pressure.
- Async trade is best suited for higher-level functions as opposed to lower-level ones like reads that require frequent
  heap allocations.
- Declaring an associated type call future can help solve the problem of heap allocation by allowing callers to know how
  large the actual type is.

### Naming Associated Types

- The challenge with using associated types is naming them since there are no clear conventions on how to do so.
- Type alias infiltration could be used to name associated types but would require additional design decisions.

## Challenges with Implementing Async Functions in Traits

Section Overview: In this section, the speaker discusses some of the challenges involved in implementing async functions
in traits. They explain that while it's possible for the compiler to generate code automatically, there are still many
design decisions that need to be made before this can happen.

### Compiler Code Generation

- The rust compiler could theoretically generate code automatically behind the scenes when writing async functions in
  traits.
- There is a lot of magic involved in the process, and it's not always clear how to name types that aren't pin box.
- Async functions and traits are likely to be implemented eventually in Rust, but there are still many design decisions
  that need to be made.

## Asynchronous Programming in Rust

Section Overview: In this section, the speaker talks about various aspects of asynchronous programming in Rust.

### Key Points

- The speaker discusses different topics related to asynchronous programming such as join, futures unordered, spawn,
  number of worker threads, async trait, spawn blocking, send and static futures pin and box futures.
- Blocking code is problematic because it doesn't let other tasks run.
- There is an argument ongoing about whether to use the standard library mutex or an asynchronously enabled mutex. Using
  a standard library mutex can lead to deadlock situations where the executor's one thread is blocked. On the other
  hand, using an asynchronously enabled mutex yields when it fails to take the lock which lets other future run again.

## Standard Library Mutexes vs Asynchronously Enabled Mutexes

Section Overview: This section focuses on the differences between standard library mutexes and asynchronously enabled
mutexes.

### Key Points

- Standard library mutexes can lead to deadlock situations where the executor's one thread is blocked while using an
  asynchronously enabled mutex yields when it fails to take the lock which lets other future run again.
- Async-enabled mutexes are necessary for sharing state between two futures. If two asynchronous threads access the same
  arc and use a standard library mutex then they may end up in deadlock situations.

## Using Async-Aware Mutexes

Section Overview: In this section, the speaker discusses the use of async-aware mutexes and standard library mutexes in
critical sections.

### Standard Library Mutex vs Async-Aware Mutex

- Use a standard library mutex for short critical sections without await points.
- Use an async-aware mutex if the critical section is long or contains await points to avoid blocking other futures on
  other threads.
- Standard library mutexes can be faster to acquire and release than async-aware mutexes.

### Deadlock Potential

- Critical sections with await points are ripe for deadlock potential.
- Holding a lock while running a slow operation can block other futures on other threads, causing them to be unable to
  make progress.

### Detecting Problems

- There are currently no good tools for detecting blocking or unexpected cancellations in futures.
- Tokyo console is being developed as a tool that hooks into the executor and monitors how long it has been since a
  given future yielded or was retried.

## Thread Spawn vs Tokyo Spawn

Section Overview: In this section, the speaker explains the conceptual difference between thread spawn and Tokyo spawn.

### Thread Spawn

- Spawns a new operating system thread that runs in parallel with everything else in your program.
- Does not take a future but takes a closure instead.
- If you want to await a future inside of a spawned thread, you need to create your own executor inside of there.

### Tokyo Spawn

- Does not guarantee having its own thread and requires cooperative scheduling with yield points.
- If you do not have yield points, you might block the executor.

## Introduction

Section Overview: In this section, the speaker introduces the topic of async and await.

### Async and Await

- The speaker introduces async and await as higher-level techniques for working with futures.
- The lower levels, such as the future trait, pin, context, or waker are not discussed in detail.
- This section provides a good survey of what mental models to have when working with async and await.

## Scheduling Futures

Section Overview: In this section, the speaker discusses how futures get scheduled on threads.

### Awaiting Inside a Future

- When a future yields, it just yields to whatever awaited it.
- If inside of f you await and then you end up yielding you just go back up to b and it's up to b what happens at this
  point right b in this case is chosen to await.
- All the way up the stack it's going to be awaits all the way up to the executor which does have the option of
  rescheduling you on a different thread.

### Single Threaded Executors

- If you have a single threaded executor it would execute on that same thread because there are no other threads in
  Tokyo.
- You're not guaranteed that it's the same thread; there are ways to say only run me on one thread in different
  executors like in Tokyo there's like a spawn local which gives you some of these guarantees.

### Enforcing Static Guarantees

- If you really wanted a future not to be sent across threads, you would just not implement send for it. Of course, that
  makes it harder to work with that future but that would be how you enforce that statically.

## Async Stack Trace

Section Overview: In this section, the speaker discusses how to get an async stack trace.

### Getting an Async Stack Trace

- If you print just a regular stack trace inside of an async inside of a future, you will get the call trace going up to
  the executor.
- The problem really stems from spawning. If I spawn this future and then let's say I panicked right here then this
  panic would not show main.
- When that executor thread awaits this future and it panics, the backtrace for that panic will only say the executor
  pulled this future so it'll point you at this future but that trace.

## Tracing Futures

Section Overview: This section discusses tracing futures and how to use them to trace back to where a future was
originally spawned.

### Using `instrument` to Trace Futures

- `instrument` is used with a future import trait.
- It takes a future and returns the same future.
- The whole future gets spawned, allowing for tracing back to where it was originally spawned.

### Best Practices for Calling Async Code from Synchronous Code

- Avoid calling async code from synchronous code as it is difficult to do right.
- Using `futures::block_on` can be problematic as it does not provide cooperative scheduling.
- Enforcing an asynchronous runtime on users can cause issues such as nested asynchronous runtimes, runtime panics, and
  performance problems.
- Expose internal asynchronous operations as asynchronous and leave it up to the user to choose how to make them
  synchronous.

### Use Cases for Threads Instead of Futures

- For compute-heavy tasks, use something like Rayon instead of futures.
- If there is no I/O involved in the program, there is no need for async.

### When Not To Use Async

- Writing non-async code tends to make simple or single execution code easier to read.
- Back traces work better when not in an async context.
- If you don't need the mechanisms that async provides, use normal threading.

## Generated by Video Highlight

https://videohighlight.com/video/summary/ThjvMReOXYM