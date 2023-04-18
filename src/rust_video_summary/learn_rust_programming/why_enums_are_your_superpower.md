# Why Enums Are Your SuperPower

<!--ts-->


<!-- Created by https://github.com/ekalinin/github-markdown-toc -->
<!-- Added by: runner, at: Tue Apr 18 03:42:02 UTC 2023 -->

<!--te-->

## [#](t=0:00:00s) Introduction

Section Overview: In this section, the speaker introduces the three main topics that will be discussed in the video.

### Topics to be Discussed

- The definition and usage of enums
- The benefits of using enums for safe input handling and removing blind booleans from code
- Finite state machines within Rust

## [#](t=0:01:35s) What is an Enum?

Section Overview: In this section, the speaker explains what an enum is and demonstrates how it represents multiple
positions.

### Enum Definition and Demonstration

- An enum represents multiple positions or states.
- A traffic light example is used to demonstrate how an enum can represent different states such as green, yellow, red,
  and flashing yellow.
    - [](t=0:04:10s) Code is shown to print out a traffic light's state using an enum.

## [#](t=0:03:29s) Enums vs Sum Types

Section Overview: In this section, the speaker briefly discusses sum types and their equivalence to enums.

### Comparison of Enums and Sum Types

- A sum type is a term from type theory that is equivalent to an enum.
- The speaker notes that beginners may not understand what a sum type is and will hold off on discussing it further.

## [#](t=0:05:09s) Printing Enums in Rust

Section Overview: In this section, the speaker explains how to print enums in Rust.

### Printing Enums with Debug Derive Implementation

- To print out an enum's value in Rust, use the debug derive implementation by adding #[derive(Debug)] above the enum
  definition.
    - [](t=0:05:35s) Code is shown with #[derive(Debug)] added above the enum definition.
- Use println!() with curly braces {} inside to indicate where the enum value should be printed.
    - [](t=0:04:41s) Code is shown with println!() used to print out a traffic light's state using an enum.

## [#](t=0:06:20s) Safe Input Handling with Enums

Section Overview: In this section, the speaker explains how enums can be used for safe input handling.

### Benefits of Using Enums for Safe Input Handling

- Enums can be used to define a set of valid inputs, ensuring that only those inputs are accepted.
- This helps prevent errors and bugs caused by invalid inputs.
- The speaker notes that enums are not the only way to achieve safe input handling but are one effective method.

## [#](t=0:07:10s) Removing Blind Booleans from Code

Section Overview: In this section, the speaker explains how enums can be used to remove blind booleans from code.

### Definition of Blind Booleans

- Blind booleans refer to functions that return a boolean without providing context or meaning for what true or false
  represents.
- This can lead to confusion and errors when working with the returned value later on.

### Benefits of Using Enums to Remove Blind Booleans

- By using an enum instead of a blind boolean, the returned value has clear meaning and context.
- This helps prevent errors and makes code easier to understand and maintain.

## [#](t=0:08:30s) Finite State Machines in Rust

Section Overview: In this section, the speaker discusses finite state machines within Rust.

### Definition of Finite State Machines

- A finite state machine is a mathematical model used to represent systems that have a finite number of states and
  transitions between those states.
- They are commonly used in programming for things like user interfaces, games, and network protocols.

### Discussion on Finite State Machines in Rust

- The speaker notes that finite state machines are an advanced topic in Rust.
- They mention that they have a big example but haven't made it simple enough for the video yet.
- The speaker will do their best to discuss finite state machines within Rust.

## [0:06:34](t=394s) Rust Enum Types

Section Overview: In this section, the speaker explains how Rust's enum types are distinct from C or Java and can act
like a struct.

### Enum Types in Rust

- Rust's enum types can act like a struct.
- Enums in Rust can encode both a unit as well as a value.
- The speaker gives an example of defining temperature using enums in Rust.
- If you have only ever used enums to encode constants, it may look strange but interesting.

## [0:09:27](t=567s) Working with Enums in Rust

Section Overview: In this section, the speaker explains how to work with enums in Rust using the match keyword.

### Using Match Keyword

- The match keyword is similar to the switch keyword but more powerful and type-safe.
- It is impossible to not take into account all possible variants every time you encounter a match or use match.
- The speaker gives an example of matching room temperature and converting it to Celsius using high school mathematics
  formula.

## Implementation for 32 to rise not available

Section Overview: The speaker mentions that the implementation for 32 to rise is not available.

### Fahrenheit to Celsius conversion

- Fahrenheit needs to be a floating point number.
- Match function ensures all variants are taken into account.
- Error message indicates if a variant is not covered.
- Wildcard match can be used but is not particularly useful.

### Safe input handling

- Speaker introduces safe input handling as a superpower.
- Example of reading from network and defining response object with text string.
- Demonstrates pulling out text from response object and converting string literal to string type.
- Difference between string slice and string type in Rust explained.

### Protecting code

- String slice is just a pointer and length, while string has pointer to memory on heap that it owns completely.
- Speaker discusses protecting code in languages like Python, JavaScript, Ruby.

## Introduction to Enums

Section Overview: In this section, the speaker introduces the concept of enums and explains how they can be used to
remove strings from code.

### Separating Business Logic from String Input Handling

- Enums allow for separation of concerns between business logic and handling string input.
- Strings are risky because invalid inputs can be dangerous or malicious. Converting them to a type is safer.

### Implementing Enums in Rust

- The `from_str` method allows for parsing strings into enums.
- Implementing `FromStr` trait enables creation of paths automatically.

### Parsing Method and Business Logic

- The parsing method should contain the business logic, such as converting to lowercase and checking if it matches "
  okay".
- Only two inputs are legal: "okay" or "error". Any other input is considered illegal. An error message is returned for
  illegal inputs.

## [#](t=0:27:18s) Parsing Strings to Enums

Section Overview: In this section, the speaker discusses how to parse strings into enums in Rust.

### Implementing FromStr Trait

- To parse a string into an enum, we need to implement the `FromStr` trait.
- We can use the `match` statement to match the input string with the corresponding enum variant. [](t=0:27:18s)

### Returning Status and Error

- When parsing a string into an enum, we can return either a status or an error.
- We wrap our result in `Ok` or `Err`, depending on whether it's a status or error. [](t=0:27:46s)

### Handling Illegal Inputs

- If we receive unexpected input, we return an error message.
- We distinguish between expected and unexpected outcomes using Rust's version of result and our own version of
  result. [](t=0:28:24s)

### Testing Implementation

- The speaker tests their implementation of `FromStr`.
- They check for errors using the compiler and try running their code. [](t=0:29:31s)

### Debugging Errors

- The speaker encounters errors while debugging their code.
- They make changes such as importing traits and removing unnecessary variables. [](t=0:30:11s)

### Using Enums to Remove Stringly Typed Data

- By using enums, we can remove stringly typed data from our program and remove business logic from parsing strings.
- This allows us to deal with input in our application code more effectively. [](t=0:33:16s)

## [#](t=0:34:51s) Pattern Matching and Rust Enums

Section Overview: In this section, the speaker discusses pattern matching in Rust and how it relates to Rust enums. They
explain how to use pattern matching to handle different variants of an enum, such as handling errors in a Result type.

### Using Pattern Matching with Enums

- [](t=0:34:51s) The speaker explains that they can use pattern matching in Rust.
- [](t=0:34:58s) They note that while the syntax for pattern matching may be unfamiliar at first, it is the idiomatic
  way to handle certain situations in Rust.
- [](t=0:35:26s) The speaker demonstrates how they used pattern matching to handle illegal input from a response object.
- [](t=0:36:30s) They explain that using enums allows for more effective error handling because every variant must be
  handled at compile time.

### Unwrapping Results

- [](t=0:35:54s) The speaker introduces the unwrap method for Results, which returns the value inside an Ok variant or
  panics if it is an Err variant.
- [](t=0:39:21s) They demonstrate how to use unwrap with a type hint before a function call.
- [](t=0:41:22s) The speaker addresses whether using unwrap is considered "evil" and notes that it depends on the
  situation.

### Understanding Option and Result Types

- [](t=0:40:01s) The speaker explains that understanding enums is important for understanding Option and Result types in
  Rust.
- [](t=0:40:29s) They emphasize that knowing how these types work is crucial for developing Rust code effectively.

## [#](t=0:42:26s) Error Handling and Blind Booleans

Section Overview: In this section, the speaker discusses error handling in Rust and introduces the concept of blind
booleans.

### Error Handling

- If an input causes an error, the program will crash.
- Tolerating a crash in production code is not recommended.
- There are cases where tolerating a crash is justified.

### Blind Booleans

- A blind boolean is a function that takes a string as input and returns a boolean without any indication of what the
  boolean represents.
- The use of blind booleans can lead to confusion and errors in code.
- It's important to provide clear names for functions that return booleans.

## [#](t=0:43:38s) Future Streams and Discord Channel

Section Overview: In this section, the speaker talks about future streams and invites viewers to join their Discord
channel for updates.

### Future Streams

- The speaker plans to do weekly streams like this one.
- They have a list of future stream topics, including traits.
- Viewers are encouraged to put the streams on their calendars.

### Discord Channel

- The speaker has a Discord channel where they post updates about their streams.
- Viewers are welcome to join the channel.

## [#](t=0:46:06s) Checking Validity of Strings with Rust

Section Overview: In this section, the speaker demonstrates how to check if a string is valid using Rust.

### Checking Validity

- Use `is_valid` function on string slice to check validity
- If it's not valid print standard error message
- Otherwise print out rest of story

## Ownership in Rust

Section Overview: In this section, the speaker explains ownership in Rust and how to indicate to Rust that it should not
delete text when the scope of a variable ends.

### Indicating Ownership

- Taking ownership requires indicating to Rust that it should not delete text when the scope of a variable ends.
- Adding ampersands asks Rust to keep the value inside the outer scope.

## Improving Boolean Statements in Type Systems

Section Overview: In this section, the speaker discusses how booleans can be improved in type systems and suggests using
an enum instead.

### Creating an Enum

- The speaker wanted to create a situation where there is both French and English.
- Validation functions should return a language instead of booleans.
- Creating an enum with special handling for French and English or undetected would be better than using booleans.
- Matching detects language instead of chaining boolean statements.

### Returning Specific Language

- It is possible to return a specific language by defining result.

## Rust Enums

Section Overview: In this section, the speaker talks about how Rust enums are similar to French return and how they can
be used to define a little type for undetected language. The speaker also mentions that enums are highly appropriate for
encoding state machines.

### Using Enums to Define Undetected Language

- Rust enums are similar to French return.
- A little type can be defined for undetected language using enums.
- It may not necessarily be an error if the language is undetected as it could still be human language but with
  uncertain identification.
- Tests for identifying languages are probabilistic in nature.

### Returning a Result Using Enums

- To return a result using enums, wrap it in `Ok` and use `Err` for errors.
- A separate type can be created for undetected language.
- If we want to return a result, we can do so by wrapping it in `Ok` and providing the undetected language type.

### Encoding State Machines with Enums

- Enums are highly appropriate for encoding state machines.
- State transitions can be guaranteed at compile time using enums.

### Live Book Preview of the Speaker's Book

- The live book preview of the speaker's book is available on manning.com under "Free Preview".
- The live book allows readers to read a significant portion of the book for free and copy code examples.

### Using Enum Cases in Type Signatures

- Enum cases can be used in type signatures.
- For example, if there is an enum called TrafficLight with cases Red, Yellow, and Green, a function could take
  TrafficLight as an argument.

## Returning Traffic Light Green

Section Overview: In this section, the speaker discusses returning traffic light green and how Rust handles it.

### Handling Traffic Light Green

- Rust cannot return only traffic light green as the type is the actual enum itself.
- Individual structs for each state can be used, but it becomes less comfortable to use.
- A state machine composed of multiple different structures can be used, but there is a proposal that enables returning
  smaller types. The speaker recommends reading Anna's blog post on encoding transitions in the type system.

## Future Streams and Conclusion

Section Overview: In this section, the speaker talks about future streams and concludes the current stream.

### Future Streams

- The speaker agrees to do a stream on traits and trait objects.
- Requests for further streams are welcome, and the speaker will attempt to stream weekly depending on availability.

### Conclusion

- The speaker thanks everyone for their support and encourages them to subscribe or follow on Twitter.
- The clickable link to connect with the speaker is provided in the transcript.
- The speaker concludes by saying goodbye and wishing everyone well.

## Generated by Video Highlight

https://videohighlight.com/video/summary/MAPdmN4hKow