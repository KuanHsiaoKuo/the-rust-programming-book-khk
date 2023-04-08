# Solving distributed systems challenges in Rust

<!--ts-->
* [Solving distributed systems challenges in Rust](#solving-distributed-systems-challenges-in-rust)
   * [Introduction](#introduction)
      * [Distributed Systems Challenges](#distributed-systems-challenges)
   * [What is Maelstrom?](#what-is-maelstrom)
      * [About Maelstrom](#about-maelstrom)
   * [Implementing the Protocol](#implementing-the-protocol)
      * [Protocol Implementation](#protocol-implementation)
   * [Setting up the Project](#setting-up-the-project)
      * [Naming the Project](#naming-the-project)
   * [Defining Message Struct](#defining-message-struct)
      * [Struct Definition](#struct-definition)
   * [Enum and Payload](#enum-and-payload)
      * [Using Enums for Message Types](#using-enums-for-message-types)
      * [Flattening Payload](#flattening-payload)
   * [State Machine Driver](#state-machine-driver)
      * [Deserializing Inputs](#deserializing-inputs)
      * [State Machine Model](#state-machine-model)
   * [Echo Node Step Function](#echo-node-step-function)
      * [Echo Node Step Function](#echo-node-step-function-1)
   * [Introduction](#introduction-1)
      * [Running a Command](#running-a-command)
   * [Debugging](#debugging)
      * [Running Binary](#running-binary)
      * [Responding to Messages](#responding-to-messages)
   * [Responding to Init Message](#responding-to-init-message)
      * [Node ID and Node IDs Fields](#node-id-and-node-ids-fields)
      * [Init Message Type](#init-message-type)
      * [Responding with Required Message](#responding-with-required-message)
      * [Flushing Standard Out](#flushing-standard-out)
   * [Printing New Line](#printing-new-line)
      * [Pretty Formatter](#pretty-formatter)
      * [Inner Part of Output Stream](#inner-part-of-output-stream)
      * [Implementing JSON Serialization](#implementing-json-serialization)
      * [Returning Messages from Step Function](#returning-messages-from-step-function)
      * [Implementing a Globally Unique ID Generation System](#implementing-a-globally-unique-id-generation-system)
   * [Introduction](#introduction-2)
      * [Testing](#testing)
   * [Payload Definition](#payload-definition)
      * [Generic Payload](#generic-payload)
      * [Node Service Implementation](#node-service-implementation)
   * [Splitting Up Shareable Parts](#splitting-up-shareable-parts)
      * [Copying Echo](#copying-echo)
      * [Generating Unique IDs](#generating-unique-ids)
      * [Responding with Generate Okay Message](#responding-with-generate-okay-message)
      * [Node Receiving Generate Messages](#node-receiving-generate-messages)
   * [Unique IDs](#unique-ids)
      * [Generating Unique IDs](#generating-unique-ids-1)
      * [Implementing Unique IDs](#implementing-unique-ids)
      * [Extracting Init](#extracting-init)
   * [Extracting Initialization Message](#extracting-initialization-message)
      * [Extracting Initialization Message](#extracting-initialization-message-1)
      * [Creating Init Response](#creating-init-response)
      * [Generating Payload](#generating-payload)
      * [Incrementing ID](#incrementing-id)
   * [Generating Init Reply](#generating-init-reply)
      * [Generating Init Reply](#generating-init-reply-1)
      * [Implementing Payload Trait for Echo](#implementing-payload-trait-for-echo)
   * [Restructuring the Code](#restructuring-the-code)
      * [Handling Init Messages](#handling-init-messages)
      * [Removing Payload Trait](#removing-payload-trait)
      * [Updating Echo Node](#updating-echo-node)
      * [Unique IDs](#unique-ids-1)
   * [Changes to Unique ID Generator](#changes-to-unique-id-generator)
      * [Unique ID Generator Changes](#unique-id-generator-changes)
   * [Debugging Echo Functionality](#debugging-echo-functionality)
      * [Debugging Echo Functionality](#debugging-echo-functionality-1)
   * [Updating Stream Deserializer](#updating-stream-deserializer)
      * [Updating Stream Deserializer](#updating-stream-deserializer-1)
   * [Unique IDs and Broadcast Challenge](#unique-ids-and-broadcast-challenge)
      * [Generating Unique IDs](#generating-unique-ids-2)
      * [Single Node Broadcast System](#single-node-broadcast-system)
   * [Conclusion](#conclusion)
   * [Order of Returned Values and Topology](#order-of-returned-values-and-topology)
      * [The Order of Returned Values](#the-order-of-returned-values)
      * [Topology](#topology)
   * [Setting Up Messages](#setting-up-messages)
      * [Initial Setup](#initial-setup)
      * [Broadcast Message](#broadcast-message)
   * [Introduction](#introduction-3)
      * [Implementing Broadcast](#implementing-broadcast)
   * [Multi-node Broadcast](#multi-node-broadcast)
      * [Gossip Protocols](#gossip-protocols)
   * [Broadcast and Gossip](#broadcast-and-gossip)
      * [Broadcast vs. Gossip](#broadcast-vs-gossip)
      * [How Gossip Works](#how-gossip-works)
   * [Understanding Gossip Protocol](#understanding-gossip-protocol)
      * [How Gossip Protocol Works](#how-gossip-protocol-works)
      * [Syncing Data Between Nodes](#syncing-data-between-nodes)
   * [The Two Generals Problem](#the-two-generals-problem)
      * [Sending Messages and Acknowledgments](#sending-messages-and-acknowledgments)
      * [No Solution to Consensus](#no-solution-to-consensus)
      * [Optimizing Gossip Protocol](#optimizing-gossip-protocol)
      * [Implementing Known Values](#implementing-known-values)
      * [Implementing Broadcast](#implementing-broadcast-1)
      * [Determining Nodes to Communicate With](#determining-nodes-to-communicate-with)
      * [Network Partitioning Issues](#network-partitioning-issues)
      * [Implementing Gossip Protocol](#implementing-gossip-protocol)
   * [Introduction to Asynchronous Programming](#introduction-to-asynchronous-programming)
      * [Two Approaches to Writing Asynchronous Code](#two-approaches-to-writing-asynchronous-code)
      * [Avoiding Asynchronous Programming for Now](#avoiding-asynchronous-programming-for-now)
      * [Synchronous Programming Techniques](#synchronous-programming-techniques)
   * [Using Channels for Input Messages](#using-channels-for-input-messages)
      * [Creating a Channel](#creating-a-channel)
      * [Receiving from Channel and Performing Step Function](#receiving-from-channel-and-performing-step-function)
      * [Dropping Standard In](#dropping-standard-in)
   * [Introduction](#introduction-4)
      * [Thread Panics and Error Handling](#thread-panics-and-error-handling)
   * [Main Loop](#main-loop)
      * [Generating Additional Input Events](#generating-additional-input-events)
   * [Event Payload](#event-payload)
      * [Differentiating Between Injected Payloads and Actual Messages](#differentiating-between-injected-payloads-and-actual-messages)
   * [Echo Fix](#echo-fix)
      * [Fixing Echo](#fixing-echo)
   * [End of File and Injected Payloads](#end-of-file-and-injected-payloads)
      * [Handling End-of-File Events](#handling-end-of-file-events)
      * [Handling Injected Payloads](#handling-injected-payloads)
      * [Starting a New Thread](#starting-a-new-thread)
      * [Serialized Payload](#serialized-payload)
      * [Enum for Injected Payload](#enum-for-injected-payload)
   * [Understanding Gossip Protocol](#understanding-gossip-protocol-1)
      * [Implementing Injected Payload](#implementing-injected-payload)
      * [Creating Helper Functions](#creating-helper-functions)
      * [Updating Gossip Protocol](#updating-gossip-protocol)
      * [Filtering Messages](#filtering-messages)
   * [Gossip Protocol Implementation](#gossip-protocol-implementation)
      * [Sending Messages](#sending-messages)
      * [Gossiping Between Nodes](#gossiping-between-nodes)
      * [Naive Gossip Implementation](#naive-gossip-implementation)
   * [Performance Analysis](#performance-analysis)
      * [Broadcast Rate vs. Propagation Delay](#broadcast-rate-vs-propagation-delay)
      * [Visibility Delay](#visibility-delay)
      * [Naive Gossip Implementation](#naive-gossip-implementation-1)
   * [Generated by Video Highlight](#generated-by-video-highlight)

<!-- Created by https://github.com/ekalinin/github-markdown-toc -->
<!-- Added by: runner, at: Sat Apr  8 16:56:02 UTC 2023 -->

<!--te-->

## Introduction

Section Overview: In this section, the speaker introduces a set of distributed systems challenges that use a platform
called Maelstrom. The challenges are designed to build an increasingly sophisticated distributed system and run it
through Maelstrom to test its correctness.

### Distributed Systems Challenges

- The challenges use a platform called Maelstrom, which is a distributed systems testing framework.
- The challenges involve building an increasingly sophisticated distributed system and running it through Maelstrom to
  test its correctness.
- The exercises are written in Go, but demo implementations of the node code are available in Ruby, JavaScript, Java,
  and Python.
- The speaker will be implementing the challenges in Rust since there isn't a demo implementation available for Rust.

## What is Maelstrom?

Section Overview: In this section, the speaker provides more information about Maelstrom and how it works.

### About Maelstrom

- Maelstrom is a distributed systems testing framework or exercise framework.
- It can orchestrate message passing between nodes in a distributed system and emulate things like delayed messages or
  reordered messages or drop messages nodes coming and going that kind of stuff.
- It's written by or alongside the author of Jepson, which is doing correctness research for distributed systems.
- Jepson has found bugs in real systems like Redis, Raft, Postgres, etcd by studying real systems.

## Implementing the Protocol

Section Overview: In this section, the speaker discusses how they will implement the protocol required by Maelstrom.

### Protocol Implementation

- Each node is just a binary that receives JSON messages from standard input and sends JSON messages to standard output.
- Messages have three parts: source (identifying string of sender), destination (identifying string of receiver), body (
  message content).
- Message bodies have reserved keys, and additional keys can be set depending on the message type.
- The speaker will need to implement this protocol in Rust to get everything set up.

## Setting up the Project

Section Overview: In this section, the speaker sets up the project and decides on a name for it.

### Naming the Project

- The speaker decides to call the project "rusten gun" after a battle spell from Naruto.
- Timestamp:

## Defining Message Struct

Section Overview: In this section, the speaker defines a struct for messages that will be passed throughout the program.

### Struct Definition

- The message struct has fields for source, destination (renamed to "dust"), type (renamed to "ty"), message ID, in
  reply to, and body.
- Body is either a string or a hashmap of string keys and serde_json values.
- The speaker considers making body an enum tagged by type but ultimately decides against it.
- Timestamp:

## Enum and Payload

Section Overview: In this section, the speaker discusses how to use enums and payloads in Rust.

### Using Enums for Message Types

- The speaker explains that they will use enums to define message types.
- They rename all enums to snake case.
- They construct a deserializer using `serde_json` from standard input.

### Flattening Payload

- The speaker mentions that they want to flatten the payload.
- They are unsure if flattening works in this regard.

## State Machine Driver

Section Overview: In this section, the speaker discusses constructing a state machine driver in Rust.

### Deserializing Inputs

- The speaker constructs a stream deserializer using `serde_json` from standard input.
- They use `into_iter()` to turn it into an iterator of messages.

### State Machine Model

- The speaker models the state machine with a struct called Echo Node.

## Echo Node Step Function

Section Overview: In this section, the speaker discusses the step function for an echo node in a distributed system.
They talk about the input message and how it returns a mutable reference to the state of the node.

### Echo Node Step Function

- The step function gets a mutable reference to the state of the node in the distributed system and the input message.
- A stream serializer is used to send messages as well as trigger messages to other nodes.
- The step function needs a way to wait for a message before replying to the current message.
- The reply is going to be a message with source, destination, body, and ID of response generated by self.id.
- If we receive an echo, we do nothing. If we get an echo, that's when we want to send the second reply.
- We construct an output channel using Certi Jason serializer new standard out.
- State starts at zero and then we're going to do state step give it input.

## Introduction

Section Overview: In this section, the speaker introduces the topic and mentions that they will be running a command.

### Running a Command

- The speaker mentions that they will run "cargo r" command.

## Debugging

Section Overview: In this section, the speaker runs a binary and encounters an error.

### Running Binary

- The speaker runs a binary using "debug or sting gun".
- The program crashes with an error message "unknown variant variant in it right".

### Responding to Messages

- The speaker explains that in response to an init message, each node must respond with a message of type init.
- They mention that there are two types of messages: init and echo.

## Responding to Init Message

Section Overview: In this section, the speaker talks about how nodes should respond to an init message.

### Node ID and Node IDs Fields

- The node ID field indicates the ID of the node which is receiving this message.
- The node IDs fields list all nodes in the cluster including the recipient.

### Init Message Type

- In response to an init message, we have to respond with a message type of init.
- Node ID is going to be a string and also node IDs which is a vek of string.

### Responding with Required Message

- For init, we don't actually care about any of the fields but what we're going to do is respond with the required
  message which is here in it okay.
- If we receive an edit okay message that should never happen we might receive an echo okay.

### Flushing Standard Out

- The speaker explains that the problem is that standard out is buffered so when we write out here we're not flushing
  standard out as well which means that the message isn't actually getting out to the server.
- It might be enough to just print a new line because this is a line buffered writer.

## Printing New Line

Section Overview: In this section, the speaker talks about printing a new line.

### Pretty Formatter

- The speaker suggests using pretty formatter to print a new line.
- The protocol itself also requires that you print a new line because it's newline separate adjacent objects.

### Inner Part of Output Stream

- In order to print a new line, we need to get at the inner part of this output stream which serializer doesn't let us
  do.
- We can unwrap it and then put it back together but that feels unfortunate to have to do that but it might just be what
  we have to do here.

## [#](t=0:36:49s) Implementing JSON Serialization

Section Overview: In this section, the speaker discusses how to implement JSON serialization in Rust.

### Implementing JSON Serialization

- [](t=0:36:49s) The speaker explains that the code requires encoding a line as JSON and mentions that it needs to be
  one per line.
- [](t=0:37:20s) The speaker suggests not constructing the serializer here and instead using standard out. They mention
  using `reply.serialize` and `serde_json::to_writer`.
- [](t=0:37:49s) The speaker uses `output.write_all` to write all of the output.
- [](t=0:38:13s) The speaker checks if everything is happy with the changes made so far.
- [](t=0:38:31s) The speaker makes sure that they are using standard out.
- [](t=0:38:59s) The speaker tries to figure out why it's not being helpful and considers doing a reborrow.
- [](t=0:39:14s) The speaker explains what they want to do with mutable references and re-borrowing them.
- [](t=0:39:37s) The speaker does a reborrow where they dereference and create a mutable reference which they can use
  again later.

## [#](t=0:40.03s) Returning Messages from Step Function

Section Overview: In this section, the speaker discusses why step functions don't return messages.

### Returning Messages from Step Function

- [](t=0.40.03s) A question is asked about whether or not step functions can return messages, but the answer is no
  because there may be multiple messages sent or received at once.
- [](t=0.41.04s) The speaker confirms that returning messages is possible for convenience, but it's separate from the
  mechanism of sending messages.

## [#](t=0:42:42s) Implementing a Globally Unique ID Generation System

Section Overview: In this section, the speaker discusses implementing a globally unique ID generation system that runs
on Maelstrom's unique IDs workload.

### Implementing a Globally Unique ID Generation System

- [](t=0:42:42s) The speaker explains that they need to implement a globally unique ID generation system that runs on
  Maelstrom's unique IDs workload.
- [](t=0:43:00s) The speaker decides to tidy up the code before generalizing and moves all the stuff over to `live.rs`.
- [](t=0:43:00s) The payload is defined in `live.rs`.

## Introduction

Section Overview: The speaker introduces the topic and mentions that they want to test something.

### Testing

- The speaker wants to see if their idea works.

## Payload Definition

Section Overview: The speaker discusses payload definition and how it is based on the node service being implemented.

### Generic Payload

- The payload needs to be fully defined by the collar, but it's okay because it can be generic.
- If the payload is all of the message types used by a given service, then at deserialization time, only those messages
  should be exchanged in that particular messaging network.
- A struct is initialized with fields that are public.

### Node Service Implementation

- A pub trait called "node" could be created to define the step function payload.
- A main loop is created for Echo to reuse.

## Splitting Up Shareable Parts

Section Overview: The speaker splits up shareable parts so that they can copy Echo to unique IDs.

### Copying Echo

- Now that shareable parts have been split up, Echo can be copied to unique IDs.

## [#](t=0:51:46s) Generating Unique IDs

Section Overview: In this section, the speaker discusses how to generate unique IDs for messages in the network.

### Generating Unique IDs

- There are different ways to generate unique IDs, such as running a consensus algorithm or generating a unique string.
- The easy way is to generate a unique string that is long enough and has a low risk of collisions.
- The speaker suggests using GUID instead of ID for clarity in the code.
- The ULID crate provides a standard way to generate globally unique identifiers that are lexicographically sortable.
    - A ULID consists of a header, timestamp in milliseconds, and randomness at the end.
    - It's not guaranteed to be globally unique but close enough.

[](t=0:52:03s)

## [#](t=0:52:03s) Responding with Generate Okay Message

Section Overview: In this section, the speaker talks about responding with a Generate Okay message after receiving a
Generate message.

### Responding with Generate Okay Message

- When receiving a Generate message, we need to respond with a Generate Okay message that includes an ID (GUID).
- This ID is locally unique and doesn't have to be globally unique.
- We can use ULID crate to generate GUIDs.

[](t=0:53:39s)

## [#](t=0:53:39s) Node Receiving Generate Messages

Section Overview: In this section, the speaker discusses what happens when nodes receive Generate messages.

### Node Receiving Generate Messages

- Nodes may not receive any Generate messages if they're not sending any.
- If nodes do receive them, it's not considered problematic.

## Unique IDs

Section Overview: In this section, the speaker discusses generating unique IDs for messages and nodes.

### Generating Unique IDs

- The speaker introduces the concept of generating unique IDs.
- The speaker shows how to view all the unique IDs being generated and notes that they appear to be truly unique.
- The speaker proposes a method for guaranteeing that message IDs are unique per node and that the combination of a
  node's ID and message ID is globally unique.

### Implementing Unique IDs

- The speaker notes that the combination of a node's ID and message ID can be used as a generated ID when responding to
  messages.
- The speaker demonstrates how to format the generated ID using self.node, self.id, and self.node (which has not yet
  been stored).
- The speaker returns itself from init in order to construct state.
- The speaker encounters an issue with constructing parts of state in advance due to limitations with from_init.
- The speaker considers making payload a combination enum or creating a separate trait for payload.
- The speaker suggests extracting init on payload instead.
- The speaker notes that it would be difficult to use step function because it is called on a node before it is
  constructed.

### Extracting Init

- The speaker suggests extracting init on payload instead.
- However, since payload is generic, we don't know how to get at the init variant.
- A separate trait for payload is suggested, which would define extract_init.
- The speaker suggests using option and panicking in code when we know that it should be present.

## Extracting Initialization Message

Section Overview: In this section, the speaker discusses extracting the initialization message and using it to create a
node.

### Extracting Initialization Message

- The initialization message is extracted from the payload.
- The speaker mentions that "no next exists."
- There may be an issue with deserialization of the message.
- There is a pause in speaking.
- Node initialization may fail.

### Creating Init Response

- The init response can be generated without passing on the message.
- The init response is created.

### Generating Payload

- Information needed for generating a payload is discussed.
- A payload should be generated with self as its output.
- A reply message can now be constructed.

### Incrementing ID

- ID incrementation is discussed and deemed annoying.

## Generating Init Reply

Section Overview: In this section, the speaker discusses generating an init reply and how it relates to node
implementation.

### Generating Init Reply

- An init reply is generated on behalf of the underlying node.
- Zero is reserved for ended okay response.
- PS payload and serialize are required here.

## [#](4312)s Implementing Payload Trait for Echo

Section Overview: In this section, the speaker discusses implementing payload trait for Echo.

### Implementing Payload Trait for Echo

-[](4323)s This will likely cause an error.
-[](4330s) Rusting gun payload is mentioned.
-[](4343s) The speaker uses let else let payload in it.
-[](4363s) Resting on in it is discussed.

## Restructuring the Code

Section Overview: In this section, the speaker discusses restructuring the code to handle messages more efficiently.

### Handling Init Messages

- The speaker suggests handling init messages differently by first deserializing a single thing from standard in using
  their own concrete enum type instead of P.
- The speaker suggests initializing the message as an init payload and constructing a message where the payload is init
  payload.

### Removing Payload Trait

- The speaker suggests removing the payload trait and constructing a message where the payload is init payload.

### Updating Echo Node

- The speaker updates Echo node to remove implementation of trait and no longer receive an init message or an in it.
- Main Loop now becomes just Echo node with no initial state.

### Unique IDs

- The speaker updates Unique IDs to remove initial state that cares about context from surrounding environment.

## Changes to Unique ID Generator

Section Overview: In this section, the speaker discusses changes made to the unique ID generator.

### Unique ID Generator Changes

- The init payload and internet okay variant main loop are no longer present.
- The unique node is now followed by nothing and has no initial state.
- The node ID is always set as part of init, so there is no need for unwrapping.
- The source destination node IDs in the response are inverted.

## Debugging Echo Functionality

Section Overview: In this section, the speaker debugs issues with the Echo functionality.

### Debugging Echo Functionality

- An error occurs when attempting to build.
- The error message indicates that a node did not respond to an init message.
- The speaker questions why the node did not respond.
- Cargo ARB is used to send an init message for debugging purposes.

## Updating Stream Deserializer

Section Overview: In this section, the speaker updates the stream deserializer.

### Updating Stream Deserializer

- Standard in waits for end of file instead of new lines, causing issues with deserialization.
- A stream deserializer is used instead of standard in dot lines.
- Since the format is new line separated, it would be better to deserialize each line at a time using standard in dot
  lines.
- Splitting by lines and straight deserializing the entire string of a line is used instead of stream deserializer.

## Unique IDs and Broadcast Challenge

Section Overview: In this section, the team works on implementing a broadcast system that gossips messages between all
nodes in the cluster. They start by generating unique IDs and move on to implementing a single node broadcast system.

### Generating Unique IDs

- The team generates unique IDs for each node in the cluster.
- The generated string is globally unique, assuming that node IDs are not reused when nodes restart.

### Single Node Broadcast System

- The team moves on to implement a single node broadcast system.
- The workload has three RPS message types: broadcast, read, and topology.
- A broadcast message sends an integer value to all nodes in the cluster.
- Nodes store the set of integer values they receive from broadcast messages so they can be returned later via the read
  message RPC.
- A read message requests that a node returns all values it has seen.

## Conclusion

Section Overview: In this section, there is no new content.

## Order of Returned Values and Topology

Section Overview: In this section, the speaker discusses the order of returned values and topology.

### The Order of Returned Values

- The order of the returned values does not matter.
- It could be a set since we're guaranteed that the messages are unique.

### Topology

- This message informs the node of who its neighboring nodes are.
- Maelstrom has multiple topologies available, and you can ignore this message and make your own topology from the list
  of nodes in the node IDs Method.
- All nodes can communicate with each other regardless of the topology passed in.
- The topology is a hash map from node to a vector of nodes.
- In response, you should return a topology.

## Setting Up Messages

Section Overview: In this section, the speaker discusses setting up messages.

### Initial Setup

- We need the node ID, message ID, and messages which is a VEC view size. Initially, messages are empty.

### Broadcast Message

- When we get a broadcast with a message, we're going to send a broadcast.
- We want to make it easier to construct something like this by having an Associated method on message which is like
  prepare reply that sets ID to one that's passed in if you have one and it sets in reply as necessary.
- There is now a reply which consumes the message. It returns itself and what it does is exactly what prepare reply
  does.

## Introduction

Section Overview: In this section, the speaker introduces the topic of gossip protocols and explains that they will be
implementing a broadcast system that propagates values to all nodes in the cluster.

### Implementing Broadcast

- The speaker explains that they need to implement broadcast for read and topology.
- They mention that topology takes a topology and ignores anything that is broadcast.
- The speaker adds a message into reply helper.

## Multi-node Broadcast

Section Overview: In this section, the speaker discusses multi-node broadcast and how values should propagate to all
nodes within a few seconds.

### Gossip Protocols

- The speaker explains that they will use gossip protocols to propagate values around the network.
- They give an example of three nodes where an operation comes in saying "broadcast 34".
- The goal is for every node in the system to know about every broadcast message.

## Broadcast and Gossip

Section Overview: This section explains the difference between broadcast and gossip protocols, how gossip works, and how
it scales better than broadcast.

### Broadcast vs. Gossip

- Broadcast sends a message to every node in the system, which doesn't scale well.
- Gossip sends a message to nodes in its topology or neighborhood, defined by direct network links or other criteria.
- Topologies do not have to be symmetrical; nodes can have different neighborhoods.
- Values should propagate to all other nodes within a few seconds.

### How Gossip Works

- When a node receives a message through gossip, it sends it to everyone in its topology except for the sender.
- The receiving nodes then send the message to their topologies until every node has received it.
- As long as there is at least one link from one node to another through transitive closures, any set of topologies will
  eventually propagate messages to every node.
- Scheduled gossip can be used instead of immediate gossip for scalability.

## Understanding Gossip Protocol

Section Overview: In this section, the speaker explains how gossip protocol works and how it differs from blind
forwarding. The speaker also discusses the issue of syncing data between nodes in a minimal fashion.

### How Gossip Protocol Works

- Gossip protocol exchanges information with neighbors about data that one has but the other does not.
- There is no blind forwarding in gossip protocol, which means there are no loops where messages keep getting sent back
  and forth between nodes.
- When a node receives a new message, it contacts its neighbors to compare notes and sync up their data sets.

### Syncing Data Between Nodes

- Syncing data between nodes can be problematic because sending an entire data set on every message is not practical.
- To minimize the amount of data being sent during syncing, nodes can eliminate messages that they already have or know
  about.
- Nodes need to remember which messages they have synced with other nodes in the past to avoid sending unnecessary
  messages during future syncs.

## The Two Generals Problem

Section Overview: This section discusses the challenge of consensus in distributed systems, specifically the Two
Generals Problem.

### Sending Messages and Acknowledgments

- A sends messages 24, 36, and 48 to B.
- B responds with messages 12 and 13.
- A never receives B's response.
- There is no way for A to know if B received the message or not.

### No Solution to Consensus

- The Two Generals Problem is a problem of consensus in distributed systems.
- There is no solution to this problem if arbitrary messages can be dropped.
- There is a mathematical proof that you cannot solve this with a finite number of messages.

### Optimizing Gossip Protocol

- We want it to be the case that if messages aren't dropped then we're able to eliminate messages from the sync.
- It's okay for us to send some extra values if some messages happen to be dropped as long as the recipient has a way to
  detect that it already knows something.
- All the messages in the system have unique IDs which makes it possible for us to keep a set and add new values when we
  hear things from a neighbor.

### Implementing Known Values

- We need to keep track of known values using a hash map from node identifier (ID) to known values.
- We also need a hash set for all incoming messages so we can add new values when they arrive.
- We need an additional hashmap called "message communicated" which will be explained later.

## [2:09:28](t=7768s) Step Function Broadcast

Section Overview: In this section, the speaker discusses how to implement the broadcast step function in the gossip
protocol.

### Implementing Broadcast

- To implement broadcast, simply insert the message.
- Use a hash set for encoding messages as JSON sequences.
- If there are issues with encoding, fix them later.

## [2:10:07](t=7807s) Topology and Neighborhood

Section Overview: In this section, the speaker discusses how to determine which nodes to communicate with using topology
and neighborhood.

### Determining Nodes to Communicate With

- The topology tells us which nodes we should communicate with.
- We want a neighborhood of nodes that we should gossip with.
- The neighborhood is a vector view of sizes no Evac of strings.
- When we get a topology, remove ourselves from it using `self.neighborhood = topology.remove(ourselves)`.

## [2:11:22](t=7882s) Network Partitioning

Section Overview: In this section, the speaker discusses network partitioning and its implications for gossip protocols.

### Network Partitioning Issues

- Randomly choosing nodes for communication can lead to network partitioning.
- This can result in clusters that cannot communicate with each other.
- One solution is to require every neighborhood size to be at least four including oneself so three additional nodes
  must overlap between neighborhoods.
- Another solution is to use a broader topology instead of random selection.

## [2:14:39](t=8079s) Gossip Protocol Implementation

Section Overview: In this section, the speaker discusses implementing the gossip protocol and some challenges associated
with it.

### Implementing Gossip Protocol

- There's no message that tells us when to do a gossip in the gossip protocol.
- One way is to start up a separate thread that generates input events every 500 milliseconds.
- Another way is to make the main loop be a gossip protocol.

## Introduction to Asynchronous Programming

Section Overview: In this section, the speaker discusses two approaches to writing asynchronous code and decides to
avoid making the code asynchronous for now.

### Two Approaches to Writing Asynchronous Code

- The first approach is to write all the code in an asynchronous style.
- The second approach is to use an actor system where every node is an actor that handles one event at a time
  synchronously.

### Avoiding Asynchronous Programming for Now

- The speaker decides to avoid making the code asynchronous for now.
- To inject additional input messages, the outer loop needs a way to do so synchronously.

### Synchronous Programming Techniques

- In synchronous programming, there are limited ways of selecting between events. One way is by using a read from
  standard in with a timeout.
- Another way is by introducing a channel and cloning the sender side of the channel. Each thread will block and perform
  operations on their respective clones.

## Using Channels for Input Messages

Section Overview: In this section, the speaker demonstrates how channels can be used for injecting input messages into
an outer loop.

### Creating a Channel

- A channel is created using `txrx := std::sync::mpsc::channel()`.
- A thread is spawned for each clone of the sender side of the channel.

### Receiving from Channel and Performing Step Function

- For each input message received from `RX`, perform step function.
- Inside each thread, parse out messages from standard in and send them as input messages
  using `std::io::stdin().lock().lines()` and `tx.send(input)` respectively.
- If sending the message fails, return from the thread.

### Dropping Standard In

- To avoid locking standard in inside a thread, it is dropped before creating the channel.
- All instances of standard in are replaced with `std::io::stdin().lock().lines()`.
- Since `self` is consumed, there is no need to worry about dropping P.

## Introduction

Section Overview: In this section, the speaker introduces the topic of thread panics and error handling.

### Thread Panics and Error Handling

- When joining a thread, the first layer of result is whether or not the thread panicked.
- The second layer of result is whether or not an error was returned.

## Main Loop

Section Overview: In this section, the speaker discusses how to generate additional input events in the main loop.

### Generating Additional Input Events

- To generate additional input events, we need to give away the TX handle to the node.
- We can construct this handle first and then initialize it with the node.
- We can spawn a thread that generates messages or events on a time schedule.
- This will surface in the main read loop when we call `node.step`.

## Event Payload

Section Overview: In this section, the speaker discusses how to differentiate between injected payloads and actual
messages from the network.

### Differentiating Between Injected Payloads and Actual Messages

- We want to differentiate between injected payloads and actual messages from the network.
- We can use an enum called "event" with three options: message payload, body, or direct payload.
- We need to serialize/deserialize these events.

## Echo Fix

Section Overview: In this section, the speaker fixes Echo so that it differentiates between injected events and actual
messages.

### Fixing Echo

- Echo needs to differentiate between injected events and actual messages.
- We can use an event payload input to differentiate between the two.
- If there is an injected event when there should not be, we will panic.
- The transmit handle in broadcast will never exit because it is held by the broadcast node.

## End of File and Injected Payloads

Section Overview: In this section, the speaker discusses how to handle end-of-file events and injected payloads in a
node.

### Handling End-of-File Events

- The end of file is reached.
- The result of reaching the end of file is not important.
- A way for the node to learn that it should exit is now available.

### Handling Injected Payloads

- "Inject" refers to a message being sent to a node from outside its network.
- If the input is a message, do what was done previously.
- If the input is an injected payload or end-of-file event, do something different.
- It may be necessary to use different enums for injected events and messages in some cases.

### Starting a New Thread

- A new thread will be started when constructing from init.
- This thread will generate gossip events using a loop that sleeps for 300 milliseconds before sending an injected
  gossip event.
- An atomic bool can be used to terminate this loop when the node receives an end-of-file event.

### Serialized Payload

- Using variants for payloads means they must be serialized and deserialized even if it's unnecessary.
- Injected payloads should be separate from other payloads so that they don't need to be matched on unnecessarily.

### Enum for Injected Payload

-[]( t = 9289 s ) An enum called "injected payload" will have a default value of the unit type.
-[]( t = 9371 s ) Injected payloads need to be sent and they need to be static.
-[]( t = 9418 s ) Broadcast will now infer that we have an injected payload gossip.

## Understanding Gossip Protocol

Section Overview: In this section, the speaker explains how to implement a gossip protocol in Python.

### Implementing Injected Payload

- The speaker explains that they will be injecting payload into the gossip protocol. [](t=2m37s)
- They explain that they should be able to hear a match on payload and that the only injected payload is
  Gossip. [](t=2m37s)

### Creating Helper Functions

- The speaker creates a helper function called "on_message" which sends preference to self and an impul writer or imple
  right. [](t=2m38s)
- They also create another helper function called "serialized_response_message" which is used for serialization of
  response messages. [](t=2m40s)

### Updating Gossip Protocol

- The speaker updates the gossip protocol by sending messages to mute output using context gossip to n in
  neighborhood. [](t=2m40s)
- They explain how to fill out the message with source, destination, ID, and payload information.[](t=2m41s)
- The speaker discusses how gossips do not need responses and can be fire-and-forget messages.[](t=2m42s)

### Filtering Messages

- The speaker filters out messages that are already known by the recipient using a copied filter method.[](t=2m43s)
- They discuss updating known-to-n but leave it for later implementation.[](t=2m44s)

## Gossip Protocol Implementation

Section Overview: In this section, the speaker explains how to implement a gossip protocol. They discuss sending
messages and gossiping between nodes.

### Sending Messages

- All messages will be sent to everyone in the neighborhood during gossip time.
- When someone tells us about a message, we add it to the set that we have.

### Gossiping Between Nodes

- During gossip scene, we extend the scene and do not reply.
- The nodes start gossiping with each other, descending to random other nodes in the network.
- As messages become longer, it takes longer for any given message to propagate across the network.

### Naive Gossip Implementation

- When receiving a gossip message, we know that the sender knows about all of the messages they sent us.
- If node A told me about three messages, I know that A knows about those three and I never need to send them again.

## Performance Analysis

Section Overview: In this section, the speaker analyzes performance issues related to implementing a gossip protocol.

### Broadcast Rate vs. Propagation Delay

- The broadcast rate is not what we want because it only measures how fast broadcast operations succeed.
- Longer gossip messages mean slower propagation between new nodes and longer delays for any given message to propagate
  across the network.

### Visibility Delay

- We want visibility delay which shows delay between when a message is broadcast and when it's visible to all peers.
- The delay doesn't seem to surface in the current view.

### Naive Gossip Implementation

- Broadcast won't get slower with naive gossip implementation because it doesn't trigger any work.
- The speaker concludes that naive gossip implementation is not good enough and proposes a better solution.
  I'm sorry, but I cannot summarize the transcript as there are only a few bullet points with timestamps available. The
  provided transcript is not sufficient to create a comprehensive and informative markdown file.
  [_CUTOFF_LIMIT_]

## Generated by Video Highlight

https://videohighlight.com/video/summary/gboGyccRVXI