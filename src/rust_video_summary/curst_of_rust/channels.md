# Channels

<!--ts-->
* [Channels](#channels)
   * [Introduction](#introduction)
      * [Introduction to Crust of Rust](#introduction-to-crust-of-rust)
   * [What is Crust of Rust?](#what-is-crust-of-rust)
      * [Description of Crust of Rust](#description-of-crust-of-rust)
   * [Introduction to Standard Sync Mpsc Module](#introduction-to-standard-sync-mpsc-module)
      * [Introduction to Standard Sync Mpsc Module](#introduction-to-standard-sync-mpsc-module-1)
   * [Topics Covered in Past Videos](#topics-covered-in-past-videos)
      * [Topics Covered in Past Videos](#topics-covered-in-past-videos-1)
   * [Overview of Standard Sync Mpsc Module](#overview-of-standard-sync-mpsc-module)
      * [Overview of Standard Sync Mpsc Module](#overview-of-standard-sync-mpsc-module-1)
   * [Implementation of Channel](#implementation-of-channel)
      * [Implementation of Channel](#implementation-of-channel-1)
   * [Notification for Upcoming Streams](#notification-for-upcoming-streams)
      * [Notification for Upcoming Streams](#notification-for-upcoming-streams-1)
   * [Channel Implementation in Rust](#channel-implementation-in-rust)
      * [Sender and Receiver Types](#sender-and-receiver-types)
      * [Sending Data through Channels](#sending-data-through-channels)
      * [Size Constraints for Types Sent through Channels](#size-constraints-for-types-sent-through-channels)
      * [Ownership of Data in Channels](#ownership-of-data-in-channels)
   * [Panama: A Simple Channel Implementation in Rust](#panama-a-simple-channel-implementation-in-rust)
      * [Implementation Details](#implementation-details)
      * [Performance Considerations](#performance-considerations)
      * [Back Pressure](#back-pressure)
   * [Mutex and Conditional Variables](#mutex-and-conditional-variables)
      * [Mutex](#mutex)
      * [Arc and Rc](#arc-and-rc)
      * [Conditional Variables](#conditional-variables)
   * [Channel Implementation in Rust](#channel-implementation-in-rust-1)
      * [Inner Type](#inner-type)
      * [Sender and Receiver Types](#sender-and-receiver-types-1)
      * [Alternative Implementations](#alternative-implementations)
   * [Mutex and Convar](#mutex-and-convar)
      * [Parking Lot Implementations](#parking-lot-implementations)
      * [Default Implementation of Vec](#default-implementation-of-vec)
      * [Arc Mutex vs Boolean Semaphore](#arc-mutex-vs-boolean-semaphore)
   * [Mutex vs Boolean Semaphore Continued](#mutex-vs-boolean-semaphore-continued)
      * [Problem with Boolean Semaphore](#problem-with-boolean-semaphore)
      * [Advantages of Mutex](#advantages-of-mutex)
   * [Why Arc Is Needed](#why-arc-is-needed)
      * [Sharing Inner](#sharing-inner)
   * [Implementing Sender](#implementing-sender)
      * [Send Function](#send-function)
   * [Implementing Receiver](#implementing-receiver)
      * [Receive Method](#receive-method)
   * [Using Vec as a Stack and Ring Buffer](#using-vec-as-a-stack-and-ring-buffer)
      * [Using Vec as a Stack](#using-vec-as-a-stack)
      * [Ring Buffer](#ring-buffer)
   * [Blocking Version of Receive](#blocking-version-of-receive)
      * [Try Receive Method](#try-receive-method)
      * [Blocking Version of Receive](#blocking-version-of-receive-1)
   * [Notification and Waiting Receivers](#notification-and-waiting-receivers)
      * [Using Condvar for Notification](#using-condvar-for-notification)
   * [Vector Double Ended Queue](#vector-double-ended-queue)
      * [Understanding Vector Double Ended Queue](#understanding-vector-double-ended-queue)
   * [Async Wait vs Weight](#async-wait-vs-weight)
      * [Difference Between Async Wait and Weight](#difference-between-async-wait-and-weight)
   * [Convars Spurious Wake Ups](#convars-spurious-wake-ups)
      * [Convars Spurious Wake Ups](#convars-spurious-wake-ups-1)
   * [Dropping Lock Before Notify](#dropping-lock-before-notify)
      * [Dropping Lock Before Notify](#dropping-lock-before-notify-1)
   * [Notify One vs Notify All](#notify-one-vs-notify-all)
      * [Notify One vs Notify All](#notify-one-vs-notify-all-1)
   * [Introduction](#introduction-1)
      * [Current Setup for Senders](#current-setup-for-senders)
   * [Making Sender Cloneable](#making-sender-cloneable)
      * [Deriving Clone for Sender](#deriving-clone-for-sender)
      * [Implementing Clone Manually](#implementing-clone-manually)
   * [Using Our Clone Implementation](#using-our-clone-implementation)
      * [Using Our Clone Method](#using-our-clone-method)
      * [Issues with Dot Operator](#issues-with-dot-operator)
   * [Blocking Send and Receive Methods](#blocking-send-and-receive-methods)
      * [Blocking Send and Receive Methods](#blocking-send-and-receive-methods-1)
   * [Testing Our Implementation](#testing-our-implementation)
      * [Ping Pong Test](#ping-pong-test)
   * [Arc Clone and Channel Implementation](#arc-clone-and-channel-implementation)
      * [Coercing to Trait Objects](#coercing-to-trait-objects)
      * [Naming Channels](#naming-channels)
      * [Problems with Channel Implementation](#problems-with-channel-implementation)
      * [Implementing Shared Channels](#implementing-shared-channels)
   * [Introduction](#introduction-2)
      * [Potential Optimization](#potential-optimization)
   * [Complicated Case](#complicated-case)
      * [Dropping Sender](#dropping-sender)
   * [Atomic Use Size](#atomic-use-size)
      * [Using Atomic Use Size](#using-atomic-use-size)
   * [Notifying All for Drop](#notifying-all-for-drop)
      * [Notifying All for Drop](#notifying-all-for-drop-1)
   * [Vector Default](#vector-default)
      * [Vector Default](#vector-default-1)
   * [Initializing Senders](#initializing-senders)
      * [Initializing Senders](#initializing-senders-1)
   * [Debugging Closed Test](#debugging-closed-test)
      * [Debugging Closed Test](#debugging-closed-test-1)
   * [Debugging Rust Programs with GDB](#debugging-rust-programs-with-gdb)
      * [Using Print Debugging Instead of GDB](#using-print-debugging-instead-of-gdb)
   * [Removing Unnecessary Code](#removing-unnecessary-code)
      * [Removing Unnecessary Code](#removing-unnecessary-code-1)
   * [Identifying a Problem with the Implementation](#identifying-a-problem-with-the-implementation)
      * [Identifying a Problem with Closed RX Channels](#identifying-a-problem-with-closed-rx-channels)
   * [Implementing Closed Flag for Failed Sends](#implementing-closed-flag-for-failed-sends)
      * [Implementing Closed Flag for Failed Sends](#implementing-closed-flag-for-failed-sends-1)
   * [Multi-Producer Multiple Consumer](#multi-producer-multiple-consumer)
      * [Resurrecting a Drop Channel and Multi-Producer Multiple Consumer Channels](#resurrecting-a-drop-channel-and-multi-producer-multiple-consumer-channels)
   * [Synchronous and Asynchronous Channels](#synchronous-and-asynchronous-channels)
      * [Synchronous vs. Asynchronous Channels](#synchronous-vs-asynchronous-channels)
   * [Weak Counters and ConVars](#weak-counters-and-convars)
      * [Using Weak Counters](#using-weak-counters)
      * [ConVars Without Mutexes](#convars-without-mutexes)
   * [Blocking Sends on Vector Resizing](#blocking-sends-on-vector-resizing)
      * [Blocking Sends During Vector Resizing](#blocking-sends-during-vector-resizing)
   * [Resizing Insights](#resizing-insights)
      * [Optimization for Channels](#optimization-for-channels)
      * [Benefits of Optimization](#benefits-of-optimization)
      * [Implementation Source](#implementation-source)
   * [Branch Predictor and Channel Implementations](#branch-predictor-and-channel-implementations)
      * [Branch Predictor](#branch-predictor)
      * [Channel Implementations](#channel-implementations)
         * [Synchronous Channels](#synchronous-channels)
         * [Asynchronous Channels](#asynchronous-channels)
         * [Rendezvous Channels](#rendezvous-channels)
         * [One-Shot Channels](#one-shot-channels)
   * [Flavors of Channels](#flavors-of-channels)
      * [Two-way Synchronization](#two-way-synchronization)
      * [One-shot Channels](#one-shot-channels-1)
      * [Bounded and Unbounded Channels](#bounded-and-unbounded-channels)
      * [Rendezvous Channel](#rendezvous-channel)
   * [Different Implementations of Channels](#different-implementations-of-channels)
      * [Synchronous Channel with Mutex Plus Convar](#synchronous-channel-with-mutex-plus-convar)
      * [Upgrading Channel Type at Runtime](#upgrading-channel-type-at-runtime)
   * [Async Await in Futures](#async-await-in-futures)
      * [Async-Await](#async-await)
   * [Implementing Channels in Rust](#implementing-channels-in-rust)
      * [Atomic Vectec or Queue](#atomic-vectec-or-queue)
      * [Signaling Mechanism](#signaling-mechanism)
      * [Linked List Implementation](#linked-list-implementation)
      * [Atomic Linked List](#atomic-linked-list)
   * [Optimizing Channel Implementations](#optimizing-channel-implementations)
      * [Using Wake-Up Primitive Instead of Linked List](#using-wake-up-primitive-instead-of-linked-list)
      * [Specialized Implementations](#specialized-implementations)
   * [Memory Overhead vs. Memory Allocator Performance](#memory-overhead-vs-memory-allocator-performance)
      * [Linked Lists and Allocation/Deallocation](#linked-lists-and-allocationdeallocation)
      * [Reusing Pool of Nodes](#reusing-pool-of-nodes)
   * [Implementing Channels for Async Await](#implementing-channels-for-async-await)
      * [Differences in Primitives](#differences-in-primitives)
      * [Internal Implementation](#internal-implementation)
   * [Channel Implementations](#channel-implementations-1)
      * [Blocking vs Async Worlds](#blocking-vs-async-worlds)
      * [Memory Allocation](#memory-allocation)
      * [Use of Channels in Thesis](#use-of-channels-in-thesis)
   * [Real Implementation Examples](#real-implementation-examples)
      * [Standard Library Implementation](#standard-library-implementation)
      * [Crossbeam Channel Implementation](#crossbeam-channel-implementation)
      * [Benchmarking Channel Implementations](#benchmarking-channel-implementations)
   * [Benchmarking Possible Configurations](#benchmarking-possible-configurations)
      * [Creating a Grid of Configurations](#creating-a-grid-of-configurations)
   * [Async Await Ecosystem](#async-await-ecosystem)
      * [Supporting Async Without Tying It To A Specific Executor](#supporting-async-without-tying-it-to-a-specific-executor)
   * [Sleeping Center Thread](#sleeping-center-thread)
      * [Waking Up A Sleeping Center Thread](#waking-up-a-sleeping-center-thread)
   * [Conclusion](#conclusion)
      * [Recording and Upcoming Streams](#recording-and-upcoming-streams)
   * [Generated by Video Highlight](#generated-by-video-highlight)

<!-- Created by https://github.com/ekalinin/github-markdown-toc -->
<!-- Added by: runner, at: Mon Apr 10 15:22:02 UTC 2023 -->

<!--te-->

## Introduction

Section Overview: In this section, the speaker introduces the episode and explains that it is a variant of the live
streams where he tackles beginner-intermediate content. He also mentions that in this episode, they will be looking at
the standard sync mpsc module.

### Introduction to Crust of Rust

- The speaker introduces another episode of Crust of Rust.
- He explains that it is a variant of his live streams where he tackles beginner-intermediate content.
- The speaker mentions that in this episode, they will be looking at the standard sync mpsc module.

## What is Crust of Rust?

Section Overview: In this section, the speaker describes what Crust of Rust is and who it's for.

### Description of Crust of Rust

- The speaker describes Crust of Rust as a series where he tackles beginner-intermediate content.
- He explains that it's for people who have read the rust book and have some familiarity with the language but are
  looking to understand more advanced topics.

## Introduction to Standard Sync Mpsc Module

Section Overview: In this section, the speaker introduces the standard sync mpsc module and explains what a channel is.

### Introduction to Standard Sync Mpsc Module

- The speaker introduces the standard sync mpsc module.
- He explains that mpsc is a channel implementation that comes in the standard library.
- A channel is just a way to send data from one place and receive it somewhere else.
- The mpsc part of it means multi-producer single consumer. This means you can have many senders but only one receiver
  so it's a many-to-one channel.

## Topics Covered in Past Videos

Section Overview: In this section, the speaker talks about some topics covered in past videos related to rust.

### Topics Covered in Past Videos

- The speaker mentions that he has covered topics such as lifetime annotations, declarative macros, iterators, smart
  pointers, and interior immutability.
- These are the topics that start popping up once you start getting deeper into rust.

## Overview of Standard Sync Mpsc Module

Section Overview: In this section, the speaker gives an overview of the standard sync mpsc module and how it works.

### Overview of Standard Sync Mpsc Module

- The speaker explains that when you create a channel using the standard sync mpsc module, you get a sender and receiver
  handle.
- You can move these handles independently to different threads to communicate over the channel.
- The channel is unidirectional. Only senders can send and only receivers can receive.
- You can clone the sender but not the receiver. This is why it's called multi-producer single consumer.

## Implementation of Channel

Section Overview: In this section, the speaker talks about implementing a channel and compares it with other
implementations.

### Implementation of Channel

- The speaker explains that they will be implementing their own channel in this episode.
- They will compare their implementation with both the standard library channel and other implementations out there.
- The speaker talks about some considerations that come up when deciding how to use channels.

## Notification for Upcoming Streams

Section Overview: In this section, the speaker talks about how to stay notified for upcoming streams.

### Notification for Upcoming Streams

- The speaker mentions that if you're interested in these crossover streams, you should follow him on Twitter or
  subscribe on YouTube or Twitch.
- This way, you'll be notified whenever he does any upcoming stream.

## Channel Implementation in Rust

Section Overview: In this section, the speaker discusses the implementation of channels in Rust.

### Sender and Receiver Types

- A channel has two halves: a sender half and a receiver half.
- The sender type is used to send data over the channel, while the receiver type is used to receive data from the
  channel.
- The sender and receiver types are different and can be distinguished from each other.

### Sending Data through Channels

- Data sent through a channel does not necessarily have to be sent between threads.
- If a channel is used within the same thread, then the data is sent on that thread without needing to be sent across
  threads.
- There are no constraints on what types of data can be sent through a channel. Any type can be sent as long as it is
  owned.

### Size Constraints for Types Sent through Channels

- Every type that is sent through a channel must implement the `Sized` trait unless you use `?Sized`.
- If there's no `?Sized`, then every type that is sent must be sized.

### Ownership of Data in Channels

- The channel object owns any data that is stored in it.
- When something is sent over a channel, it remains owned by the channel until it's received by another thread or
  dropped.

## Panama: A Simple Channel Implementation in Rust

Section Overview: In this section, the speaker demonstrates how to implement a simple version of channels in Rust using
concurrency primitives from its standard library.

### Implementation Details

- To implement channels in Rust, we need to create two structs: `Sender<T>` and `Receiver<T>`.
- We also need to create a function called `channel` which returns both halves of our newly created struct.
- Our implementation will use other parts of Rust's standard library such as sync module for concurrency primitives.

### Performance Considerations

- The implementation demonstrated in this section is not the most performant.
- Implementing a highly performant channel requires more subtlety and trickiness that may be hard to cover in a stream.

### Back Pressure

- The speaker mentions that they will discuss how back pressure works when they start implementing the channel.

## Mutex and Conditional Variables

Section Overview: In this section, the speaker explains how mutexes and conditional variables work in Rust's concurrency
model.

### Mutex

- A mutex is a way to ensure that only one thread can access a shared resource at any given time.
- If two threads try to lock the same mutex, one will get to go while the other will block until the first thread
  releases the guard.
- This ensures that there's only ever one thread modifying the shared resource at any given time.

### Arc and Rc

- Arc is an atomically reference counted type that allows for safe sharing of data across thread boundaries.
- Rc is a reference counter type that does not allow for safe sharing of data across thread boundaries.

### Conditional Variables

- A conditional variable (convar) is a way to announce to a different thread that you've changed something it cares
  about.
- It allows a sender to wake up a receiver who was waiting because there was no data yet.
- Together with mutexes, convars provide useful concurrency primitives for writing concurrent code safely.

## Channel Implementation in Rust

Section Overview: In this section, the speaker outlines how channels are implemented in Rust using mutexes and
conditional variables.

### Inner Type

- The inner type holds the data that is shared between multiple handles or halves of the channel.
- For now, it's defined as an empty vector but will be modified later on.

### Sender and Receiver Types

- Both sender and receiver types contain an inner type which is an arc-mutex-t holding shared state.
- They both return their respective types containing this inner type when created.

### Alternative Implementations

- Linked lists could be used instead of vectors for storing data in channels.
- An unsynced version could be created for non-sendable types by defining an unsynced mpsc.

## Mutex and Convar

Section Overview: In this section, the speaker discusses the implementation of mutex and convar in Rust's standard
library. They also explain why a mutex is needed for synchronization between sender and receiver threads.

### Parking Lot Implementations

- The parking lot implementations of mutex and convar could be used as well.
- There has been talk of making them the standard library ones.
- The reason to not put the mutex in inner is discussed.

### Default Implementation of Vec

- The default implementation of vec can be used.
- The question about why the receiver type needs to have an arc protected by a mutex if the channel may only have a
  single consumer thread is answered.

### Arc Mutex vs Boolean Semaphore

- A mutex is effectively a boolean semaphore, so there is no difference between them.
- Using a boolean semaphore over the implementation in mutex would require spinning, which makes it less efficient than
  using a mutex that integrates with parking mechanisms.

## Mutex vs Boolean Semaphore Continued

Section Overview: In this section, the speaker continues discussing the differences between using a boolean semaphore
versus using a mutex for synchronization.

### Problem with Boolean Semaphore

- If someone else has already taken the lock with a boolean semaphore, then you have to spin repeatedly checking it
  until it becomes available again.

### Advantages of Mutex

- With a mutex, if someone else has taken it, then the operating system can put your thread to sleep and wake it back up
  when the lock becomes available again.
- This generally more efficient than spinning with a boolean semaphore but adds some latency.

## Why Arc Is Needed

Section Overview: In this section, the speaker explains why an arc is needed for sender and receiver threads to
communicate.

### Sharing Inner

- If there was no arc, then the sender and receiver would have two different instances of inner.
- They need to share an inner because that's where the sender puts data and where the receiver takes data out of.

## Implementing Sender

Section Overview: In this section, the speaker starts implementing a send function for the sender thread.

### Send Function

- The send function takes immute self and t that we're going to send.
- It returns nothing for now.
- The lock is taken on self.inner.
- Lock result is used in case another thread panicked while holding the lock.
- Inner q is locked, and t is pushed onto it.

## Implementing Receiver

Section Overview: In this section, the speaker implements a receive method for the receiver thread.

### Receive Method

- The receive method does not take a t but returns it instead.
- The queue is locked with inner q.lock().
- T is popped from the queue.

## Using Vec as a Stack and Ring Buffer

Section Overview: In this section, the speaker explains how to use Vec as a stack and why it is better to use a ring
buffer instead.

### Using Vec as a Stack

- A Vec can be used like a stack.
- When removing an element from the front of the Vec, all other elements have to be shifted down to fill the hole left
  by the removed element.

### Ring Buffer

- A ring buffer is a type of collection that implements fixed memory.
- It is similar to a vector but keeps track of start and end positions separately.
- If you push an element to the end, it pushes it to the end.
- If you pop from the front, it removes the element and moves a pointer to where data starts.
- This allows for using it as a queue instead of just as a stack.

## Blocking Version of Receive

Section Overview: In this section, we learn about providing blocking version of receive in Rust.

### Try Receive Method

- We can provide try_receive method that returns an option t if there's nothing in there.
- It just returns none if there's nothing to receive.

### Blocking Version of Receive

- We want to provide blocking version of receive that waits for something in channel.
- To do this, we need convar which needs to be outside mutex because when holding mutex and waking up another thread
  causes deadlock.
- The convar requires giving in mutex guard so that it does this step atomically.
- We match on cue pop front; if sum t then return t else block with self inner available wait.

## Notification and Waiting Receivers

Section Overview: In this section, the speaker discusses how the sender needs to notify waiting receivers once it sends.
They explain that if a thread enters a loop and is sleeping when a send happens, the thread needs to wake up. The
speaker also introduces the use of `condvar` for notification.

### Using Condvar for Notification

- The speaker explains that they will use `condvar` for notification.
- They mention that `condvar` has a `notify_one()` and a `notify_all()` call.
- The speaker drops the lock so that whoever they notify can wake up.
- They notify one thread because they are the sender and know that this will be a receiver.

## Vector Double Ended Queue

Section Overview: In this section, the speaker briefly explains what a vector double ended queue is.

### Understanding Vector Double Ended Queue

- The speaker mentions that vector double ended queue is just like a vector with head and tail index.

## Async Wait vs Weight

Section Overview: In this section, the speaker clarifies the difference between async wait and weight.

### Difference Between Async Wait and Weight

- The speaker explains that weight requires you to give up the guard before going to sleep because otherwise whoever
  would have woken you up can't get the mutex.
- They clarify that async wait is generally used when you are I/O bound, not CPU bound.
- If you're woken up without there being anything for you to do, you loop around, check the queue, realize it's still
  empty, then go back to sleep again.

## Convars Spurious Wake Ups

Section Overview: In this section, the speaker explains how convars can have spurious wake ups.

### Convars Spurious Wake Ups

- The speaker mentions that one thing that can happen with convars is when you call weight, the operating system doesn't
  guarantee that you aren't woken up without there being anything for you to do.
- They explain that the loop checks the queue and goes back to sleep again if there's nothing to do.

## Dropping Lock Before Notify

Section Overview: In this section, the speaker clarifies why they drop the lock before notify.

### Dropping Lock Before Notify

- The speaker clarifies that they drop the lock before notify so that when the other thread wakes up, it can immediately
  take the lock.

## Notify One vs Notify All

Section Overview: In this section, the speaker explains notify one and notify all.

### Notify One vs Notify All

- The speaker explains that notify one means notifying one of the threads waiting on a specific convar.
- They clarify that notify all notifies all waiting threads.

## Introduction

Section Overview: In this section, the speaker introduces the topic of the video and discusses the current setup for
senders.

### Current Setup for Senders

- Senders cannot block in the current setup.
- When a sender gets the lock, it always succeeds in sending.
- There are never any waiting senders in the current design.

## Making Sender Cloneable

Section Overview: In this section, the speaker discusses how to make sender cloneable and why deriving clone is not
sufficient.

### Deriving Clone for Sender

- Deriving clone at least at the moment actually desugars into impul t clone clone for sender t.
- This implementation added the clone bound to t as well.
- Inner might like if the structure you're deriving clone on contains a t then t does need to be cloned in order for you
  to clone the whole type.

### Implementing Clone Manually

- Arc implements clone regardless of whether the inner type is cloned that's sort of what reference counting means.
- We don't actually need t to be cloned; we want this implementation.
- The reason why we need to implement clone ourselves manually luckily it's pretty simple though um it's just inner is
  self inner.clone().

## Using Our Clone Implementation

Section Overview: In this section, we discuss how to use our implemented clone method and why using dot operator can
cause issues.

### Using Our Clone Method

- Usually what you want to do here is use our_clone() to say that I specifically want to clone the arc and not the thing
  inside the arc.

### Issues with Dot Operator

- Imagine that inner also implemented clone rust won't know whether this call is supposed to clone the arc or thing
  inside because arc dereferences to inner type and dot operator sort of recurses into the inner drafts.
- Usually, it's not what you want to write.

## Blocking Send and Receive Methods

Section Overview: In this section, we discuss how send and receive methods should be blocking.

### Blocking Send and Receive Methods

- Send and receive should be blocking methods.
- If you try to receive, and there's nothing in the channel, we want the thread to block.

## Testing Our Implementation

Section Overview: In this section, we discuss how to test our implementation using a ping pong test.

### Ping Pong Test

- We create a tx and an rx.
- We use super::channel() to create a channel.
- We send 42 using tx.send(42).
- We assert equals rx.receive(42).

## Arc Clone and Channel Implementation

Section Overview: In this section, the speaker discusses the implementation of arc clone and channels in Rust.

### Coercing to Trait Objects

- The `arc clone` cannot coerce to trait objects automatically.
- It needs to be done manually by using `r clone as arctin trade`.

### Naming Channels

- The speaker prefers using `tx` and `rx` for channels instead of the standard documentation's use of `std`.
- However, it ultimately comes down to personal preference.

### Problems with Channel Implementation

- If there are no senders left, the receiver will block forever.
- To solve this problem, we need a way to indicate that there are no more senders left and that the channel has been
  closed.

### Implementing Shared Channels

- We change the naming convention from `q` to `inner`.
- A mutex is used to protect an inner t which holds both the queue and a count of senders.
- Every time a sender is cloned, we increase the number of senders in that value.
- When a sender goes away, we grab the lock and check if it was the last one. If so, we wake up any blocking receivers.
- The receiver now returns an option t rather than just a t because it could be that the channel truly is empty forever.

## Introduction

Section Overview: In this section, the speakers discuss potential optimizations for their implementation of a channel in
Rust.

### Potential Optimization

- The speakers discuss the possibility of optimizing their implementation by using the reference count in the arc
  instead of keeping track of senders.
- They explain that with Arc, there is a strong count which tells how many instances of that arc there are. If there's
  only one, then that must be the receiver and therefore there are no senders.
- They agree that it's a good optimization and decide to get rid of the sender's field.

## Complicated Case

Section Overview: In this section, the speakers discuss a complicated case related to dropping a sender.

### Dropping Sender

- The speakers mention that if you drop a sender, you don't know whether to notify because if the count is two, you
  might be the last sender or you might be the second to last sender and the receiver has been dropped.
- They decide to keep it as it was since it's easier to read and there are plenty of optimizations they can make over
  this implementation.

## Atomic Use Size

Section Overview: In this section, the speakers discuss using an atomic use size and shared rather than creating inner.

### Using Atomic Use Size

- The speakers suggest using an atomic use size and shared instead of creating inner.
- However, they explain that at the moment you take a mutex, there isn't really much value to it. It would mean you
  don't have to take lock in and drop and clone, but those should be relatively rare and the critical sections are short
  enough that the lock should be fast anyway.

## Notifying All for Drop

Section Overview: In this section, the speakers discuss whether to notify all for drop.

### Notifying All for Drop

- The speakers discuss whether to notify all for drop.
- They explain that when the last sender goes away, there will be at most one thread waiting which will be the receiver
  if any.
- They mention that there's no correctness issue to waking up more threads, but it is a performance issue.

## Vector Default

Section Overview: In this section, the speakers discuss vector default.

### Vector Default

- The speakers answer a question about the difference between vectec new and vector default. They explain that there is
  no difference.

## Initializing Senders

Section Overview: In this section, the speakers discuss initializing senders in their implementation of channel in Rust.

### Initializing Senders

- The speakers mention that they think the error was initializing senders to one in the constructor and then calling
  clone on the sender they return.
- They also answer a question about whether you can get false sharing in between vectec and sender count. They explain
  that they're under a mutex anyway so that shouldn't matter.

## Debugging Closed Test

Section Overview: In this section, the speakers debug why their closed test did not work as expected.

### Debugging Closed Test

- The speakers mention that their closed test hangs forever on receive and they need to figure out why.
- They decide to debug print the value of senders and see what comes out.
- They realize that the sender is not being dropped and try assigning to underscore, but it doesn't work as expected.
- They finally realize that this implementation is pretty straightforward.

## Debugging Rust Programs with GDB

Section Overview: In this section, the speaker discusses using GDB to debug Rust programs and how print debugging is
easier for small examples.

### Using Print Debugging Instead of GDB

- The speaker uses print debugging instead of GDB for small examples.

## Removing Unnecessary Code

Section Overview: In this section, the speaker removes unnecessary code from the implementation.

### Removing Unnecessary Code

- The speaker removes unnecessary code from the implementation.

## Identifying a Problem with the Implementation

Section Overview: In this section, the speaker identifies a problem with the current implementation and discusses
whether send should always succeed or fail in some way.

### Identifying a Problem with Closed RX Channels

- The current implementation has a problem where it can go the other way around if there's a closed RX channel.
- It's not clear whether send should always succeed or fail in some way when trying to send something on a closed
  channel.
- The decision is made to keep it as is but in real implementations, send might return an error if the channel is
  closed.
- If send fails, make sure to give back the value that was tried to be sent so that it can be used elsewhere or logged.

## Implementing Closed Flag for Failed Sends

Section Overview: In this section, the speaker discusses implementing a closed flag for failed sends and how it would
work.

### Implementing Closed Flag for Failed Sends

- If you want sending to be able to fail, you need to add a closed flag to the inner or just a boolean that the sender
  sets.
- When you send, if the flag is set, you return an error rather than pushing to the queue.

## Multi-Producer Multiple Consumer

Section Overview: In this section, the speaker discusses whether it's possible to resurrect a drop channel and how
multi-producer multiple consumer channels work.

### Resurrecting a Drop Channel and Multi-Producer Multiple Consumer Channels

- If the sender goes away, there's no way to send anymore in this particular design.
- In theory, we can add a method that lets you construct a sender from the receiver but most implementations are not
  quite as symmetric as this one and you can't easily create a center from a receiver.
- Every operation takes the lock which is fine for low-performance channels but for high-performance channels with many
  sends competing with each other, it might be better not to have them contend with one another.
- Realistically, an implementation could allow sends to compete with each other and only synchronize between senders and
  receivers instead of locking all of them.
- The standard library channel has two different sender types: sender and sync sender. The difference between these is
  that one is synchronous and the other is asynchronous.

## Synchronous and Asynchronous Channels

Section Overview: This section discusses the difference between synchronous and asynchronous channels, and how they
handle back pressure.

### Synchronous vs. Asynchronous Channels

- The sender and receiver go in lockstep in a synchronous channel.
- Sends can't block in this implementation of asynchronous channels.
- A synchronous channel has back pressure, meaning the sender will eventually start blocking if the receiver isn't
  keeping up.
- The standard library has a sync channel function that takes a bound (channel capacity), whereas our channel method is
  an infinite queue.

## Weak Counters and ConVars

Section Overview: This section discusses weak counters, convars, and their use cases.

### Using Weak Counters

- Senders could use weak counters to determine if the receiver is still there before sending data.
- Upgrading to a strong reference using weak counters adds overhead to each send operation.

### ConVars Without Mutexes

- ConVars require mutex guards, so it's not possible to have them without mutexes.

## Blocking Sends on Vector Resizing

Section Overview: This section discusses vector resizing during sends and its impact on blocking.

### Blocking Sends During Vector Resizing

- Pushing beyond the capacity of a vector requires allocating new memory for it, which takes time but doesn't block
  sends.
- However, during this time, sends take longer than usual because they're waiting for the vector resize to complete.

## Resizing Insights

Section Overview: In this section, the speaker talks about an optimization that can be made to the implementation of
channels.

### Optimization for Channels

- An optimization is made by encoding the assumption that there is only one receiver in the code to make it more
  efficient.
- The trick used is to steal all items that have been queued up rather than just stealing one since no one else will
  take them.
- When someone calls receive, we first check if we still have some leftover items from last time we took the lock. If
  so, we return from there without taking the lock again.
- If the queue is not empty and we get an item, then we check if there are more items in the queue and steal all of
  them. We swap that vec deck with the one buffered inside ourselves and leave m empty self.buffer.

### Benefits of Optimization

- This optimization reduces contention because now, instead of taking a lock on every receive, it's taken fewer times
  making it faster to acquire.
- It triggers twice as many resizes at predictable intervals but only uses twice as much memory which is also amortized.

### Implementation Source

- The speaker got this optimization from other implementations since it's a pretty common implementation.

## Branch Predictor and Channel Implementations

Section Overview: In this section, the speaker discusses the branch predictor and different flavors of channel
implementations.

### Branch Predictor

- The CPU has a built-in component that observes all conditional jumps.
- It tries to remember whether it took the branch or not the last time.
- Speculative execution comes into play where if it runs that code again, the branch predictor is going to say it's
  probably going to take the branch or it's probably not going to take the branch.
- Start running that code under the assumption that it will or won't and then if it doesn't end up doing that then go
  back and unwind what you did and then do that stuff instead.

### Channel Implementations

- There are multiple different kinds of flavors for channel implementations.
- Usually, they take one of two approaches:
    - Different types for different implementations of channels
    - A single sender type with an enum-like implementation under the hood
- Flavors have multiple implementations of your channel with different backing implementations chosen depending on how
  the channel is used.
- Common flavors include synchronous channels, asynchronous channels, rendezvous channels, and one-shot channels.

#### Synchronous Channels

- This is a channel where send blocks.
- Usually has limited capacity.

#### Asynchronous Channels

- This is a channel where send cannot block.
- Usually unbounded so any number of sends can build up as much as stuff impossible in memory.

#### Rendezvous Channels

- A synchronous channel with capacity equals zero.
- Used for thread synchronization rather than sending data between threads.
- Only allows sending if there's currently a blocking receiver because you can't store anything in the channel itself.

#### One-Shot Channels

Not discussed in detail.

## Flavors of Channels

Section Overview: This section covers the different flavors of channels in Rust.

### Two-way Synchronization

- Two-way synchronization occurs because the receiver cannot proceed until the sender arrives.
- The channel version is still a two-way synchronization.

### One-shot Channels

- One-shot channels are channels that you only send on once.
- These can be any capacity, but usually, they are things like an application where you have a channel that you use to
  tell all the threads that they should exit early.
- You might have a channel that you only send on once and don't send anything useful.

### Bounded and Unbounded Channels

- Circular channels are often called bounded channels.
- Asynchronous channels are often called unbounded channels.
- In Rust, these flavors aren't different types in the type system; they're different implementations chosen between at
  runtime.

### Rendezvous Channel

- A rendezvous is not a mutex because it doesn't guarantee mutual exclusion. It's more like a convar in that you can
  wake up another thread.
- You can totally send data in a rendezvous channel right; it still has the t type, but if both sender and receiver
  aren't present, nothing happens.

## Different Implementations of Channels

Section Overview: This section covers different implementations of Rust's synchronous channel and async-await futures.

### Synchronous Channel with Mutex Plus Convar

- For synchronous channels, what we implemented was a mutex plus convar as well.

### Upgrading Channel Type at Runtime

- Initially, assume that the channel is a one-shot channel.
- The moment an additional send happens, upgrade it to be a different type of channel.
- This means that the first send will be more efficient than later ones.

## Async Await in Futures

Section Overview: This section covers async-await in Rust's futures.

### Async-Await

- The idea of async-await is that you can write code that looks like it's synchronous, but it's actually asynchronous.
- You can use the `async` keyword to mark a function as being asynchronous.
- You can then use the `await` keyword to wait for an asynchronous operation to complete.
- In Rust, futures are used to represent asynchronous operations.

## Implementing Channels in Rust

Section Overview: In this section, the speaker discusses different ways to implement channels in Rust.

### Atomic Vectec or Queue

- Use an atomic vectec or queue to update head and tail pointers atomically.
- No need to take a mutex in order to send.
- Update head and tail pointers atomically to ensure no thread ever tries to touch data that another thread is currently
  touching.

### Signaling Mechanism

- Use park and notify primitives from the standard library or parking lot for wake-ups.
- Need some signaling mechanism where if the sender is asleep because it's blocking, the receiver needs to wake it up if
  it receives something because now there's capacity available.
- Similarly, if the receiver is blocking because the channel is empty and a sender comes along and sends something, it
  needs to make sure to wake up the receiver.

### Linked List Implementation

- Use a linked list implementation instead of a vectec when resizing becomes problematic.
- When a sender comes along, append or push to the front of the linked list.
- The receiver steals all items by setting head to null or none and walking backwards through the doubly linked list.

### Atomic Linked List

- An atomic linked list is often referred to as an atomic queue.
- Mix atomic head and tail with an atomic linked list so that only occasionally do you need to actually append new
  items.
- This avoids problematic operations when two senders want to send at the same time with a linked list.

## Optimizing Channel Implementations

Section Overview: In this section, the speaker discusses how to optimize channel implementations by using a wake-up
primitive instead of a linked list for rendezvous channels.

### Using Wake-Up Primitive Instead of Linked List

- A block atomic linked list is more efficient in practice.
- A single place in memory can store the item for handoff.
- You don't need a linked list at all; you only need a wake-up primitive.
- An atomic place in memory that is either none or some can be used to swap elements.

### Specialized Implementations

- More specialized implementations can be written that are faster for specific use cases.
- Async await channels have different implementations than blocking thread world channels.

## Memory Overhead vs. Memory Allocator Performance

Section Overview: In this section, the speaker discusses the trade-off between memory overhead and memory allocator
performance when using linked lists in channel implementations.

### Linked Lists and Allocation/Deallocation

- With linked lists, an allocation/deallocation occurs on each push/pop operation.
- The memory allocation system is not always the bottleneck, especially with thread-local allocation like jmallock.
- Measuring memory overhead versus memory allocator performance is important.

### Reusing Pool of Nodes

- Keeping a pool of nodes around can help reduce allocations/deallocations.
- Managing the pool atomically requires synchronization primitives and may not be better than using a memory allocator.

## Implementing Channels for Async Await

Section Overview: In this section, the speaker discusses the challenges of implementing channels for both async await
and blocking thread worlds.

### Differences in Primitives

- It is difficult to write a channel implementation that works for both async await and blocking thread worlds.
- Yielding to the parent future is necessary in async await when the channel is full.
- Notification primitives are similar but not quite the same.

### Internal Implementation

- Writing an implementation that knows whether it's being used in async context or blocking context without exposing it
  to the user can be challenging.

## Channel Implementations

Section Overview: In this section, the speaker discusses channel implementations and their underlying data structures.
They also talk about memory allocation and the use of channels in their thesis.

### Blocking vs Async Worlds

- At runtime, channel implementations diverge into different ways of managing the underlying data store depending on
  whether you're in the blocking world or not.
- In practice, the data structure used is fairly similar regardless of whether you're using a vector or an atomic linked
  list.

### Memory Allocation

- Allocators are good at taking advantage of repeated patterns, so it's hard to write good performant garbage collection
  and memory allocator has had a lot of practice with it.
- It's not clear if you could beat the allocator because we always need allocations of the same size.

### Use of Channels in Thesis

- The speaker uses Tokyo channels because they needed one with async. They don't have a particularly strong feeling
  about it.
- The decision wasn't that important since Noria is not really channel-bound.
- The standard library channel didn't support async await when they started adding them.

## Real Implementation Examples

Section Overview: In this section, the speaker recommends looking at real implementation examples for channels and
provides some pointers on where to look next.

### Standard Library Implementation

- The mpsc module in Rust's standard library has really good documentation on what's going on under the hood and some
  optimizations that they do like internal atomic counters.

### Crossbeam Channel Implementation

- Crossbeam has a cross beam channel subdirectory that holds all different flavor implementations such as array for
  synchronous channels, headtail business for multi-producer single consumer, and atomic block linked list for
  multi-producer multi-consumer.
- Flume is a different implementation of channels that popped up fairly recently. It has a very different implementation
  to what crossbeam does.

### Benchmarking Channel Implementations

- Benchmarking channel implementations is hard. Burnt Sushi did a bunch of benchmarking of channels looking at go
  channels, the standard library channels, flume, crossbeam channel, and chan crate.
- When benchmarking channels, you want to try to benchmark all the different flavors because they represent real use
  cases. You want your benchmark to test cases where you send large or small things with many or few senders.

## Benchmarking Possible Configurations

Section Overview: The speaker suggests creating a grid of all possible configurations and benchmarking each one
separately.

### Creating a Grid of Configurations

- The speaker suggests creating a grid of all possible configurations.
- Rendezvous channels are like the default go channels with zero capacity.
- A bump allocator would be really good since you would likely allocate memory atomically quite possibly and also
  because it you don't need to drop anything in that case because the memory has already been handed off.

## Async Await Ecosystem

Section Overview: The speaker explains how async is supported without tying it to a specific executor like Tokyo.

### Supporting Async Without Tying It To A Specific Executor

- The primary reason for the lack of harmony in the async await ecosystem is around the i/o traits.
- Implementing a channel does not require either asynchronous or spawn features. All that's needed is the primitive
  provided by the standard library, which is the waker trait, and the ability to yield or go to sleep and wake something
  up or notify something. These are from the standard task module in the standard library, so they can be used
  independently of what executor is being used.

## Sleeping Center Thread

Section Overview: The speaker explains how to wake up a sleeping center thread if its receiver is dropped.

### Waking Up A Sleeping Center Thread

- If you have a sleeping center thread and you'd like to wake it if the receiver is dropped, implement drop for your
  receiver where it will do a notify all to wake up all sleeping centers which could then do whatever freeing up of
  resources it needs to do.

## Conclusion

Section Overview: The speaker concludes the video and provides information on where to find the recording and upcoming
streams.

### Recording and Upcoming Streams

- The recording will be uploaded to YouTube, and the speaker will tweet about it.
- Follow the speaker on Twitter or join their Discord for updates on upcoming streams.
- The Discord channel includes other Rust streamers, including Steve Klabneck.

## Generated by Video Highlight

https://videohighlight.com/video/summary/b4mS5UPHh20：