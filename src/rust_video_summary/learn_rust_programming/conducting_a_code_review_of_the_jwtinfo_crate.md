# Conducting a code review of the jwtinfo crate

<!--ts-->
* [Conducting a code review of the jwtinfo crate](#conducting-a-code-review-of-the-jwtinfo-crate)
   * [Introduction](#introduction)
      * [Starting the Stream](#starting-the-stream)
   * [Setting Up](#setting-up)
      * [Cloning Repository](#cloning-repository)
      * [Choosing an IDE](#choosing-an-ide)
   * [Reviewing Code](#reviewing-code)
      * [Cargo.toml File](#cargotoml-file)
      * [Dependencies](#dependencies)
   * [Continuing Reviewing Code](#continuing-reviewing-code)
      * [Argo.tml](#argotml)
      * [Dependencies](#dependencies-1)
   * [Reviewing Code and Project Structure](#reviewing-code-and-project-structure)
      * [Reviewing Project Structure](#reviewing-project-structure)
      * [Fuzzing Tools](#fuzzing-tools)
   * [Asking Questions Live](#asking-questions-live)
      * [Live Q&amp;A](#live-qa)
      * [Testing for Illegal Input](#testing-for-illegal-input)
      * [Using Quick Check](#using-quick-check)
      * [Handling Null Bytes](#handling-null-bytes)
   * [Introduction](#introduction-1)
      * [Speaker Introduction](#speaker-introduction)
   * [Project Structure](#project-structure)
      * [Project Structure](#project-structure-1)
      * [Error Management](#error-management)
   * [Pause Token](#pause-token)
      * [Pause Token](#pause-token-1)
   * [Improving the Ergonomics of Functions](#improving-the-ergonomics-of-functions)
      * [Accepting Arbitrary Input](#accepting-arbitrary-input)
      * [Refactoring Function Declaration](#refactoring-function-declaration)
      * [Rust's Relationship with Text Types](#rusts-relationship-with-text-types)
   * [Limitations of Accepting Arbitrary Input](#limitations-of-accepting-arbitrary-input)
      * [Problem with Accepting Arbitrary Bytes](#problem-with-accepting-arbitrary-bytes)
   * [Error Management](#error-management-1)
      * [Dealing with Multiple Error Types](#dealing-with-multiple-error-types)
      * [Custom Error Type](#custom-error-type)
      * [Displaying Errors](#displaying-errors)
   * [Understanding the Code](#understanding-the-code)
      * [Simplifying the Code](#simplifying-the-code)
      * [Improving Readability](#improving-readability)
      * [Modularization](#modularization)
      * [Implementing Pattern of Simplification](#implementing-pattern-of-simplification)
      * [Alternative Method for Returning Messages](#alternative-method-for-returning-messages)
      * [Personal Opinion on Using Rust 2018 Idioms](#personal-opinion-on-using-rust-2018-idioms)
      * [Benefits of Splitting Out Different Parts of Code](#benefits-of-splitting-out-different-parts-of-code)
      * [Retaining Index of String Input](#retaining-index-of-string-input)
      * [Rust's Result Object](#rusts-result-object)
      * [Adding Derives](#adding-derives)
   * [Rust Programming and Cultural Differences](#rust-programming-and-cultural-differences)
      * [Bureaucratic Nature of Rust](#bureaucratic-nature-of-rust)
      * [JWT Keto Types](#jwt-keto-types)
      * [Claims Set](#claims-set)
   * [Derive Debug and Type Conversion](#derive-debug-and-type-conversion)
      * [Deriving Debug](#deriving-debug)
      * [Type Conversion](#type-conversion)
      * [Mistake Correction](#mistake-correction)
   * [IP Addresses and Color Change](#ip-addresses-and-color-change)
      * [IP Addresses](#ip-addresses)
      * [Color Change](#color-change)
   * [Exposing a Library and CLI Together](#exposing-a-library-and-cli-together)
      * [Understanding the Library Component](#understanding-the-library-component)
      * [Building Documentation](#building-documentation)
      * [Improving Library Functionality](#improving-library-functionality)
   * [Testing and Code Coverage](#testing-and-code-coverage)
      * [Resilience Against Malicious Input](#resilience-against-malicious-input)
      * [Code Coverage](#code-coverage)
   * [Build Process and Shell Check](#build-process-and-shell-check)
      * [Installing Latest Release Based on Dip](#installing-latest-release-based-on-dip)
      * [Compliance with RSA and Documenting Platforms](#compliance-with-rsa-and-documenting-platforms)
      * [Introduction to Shell Check](#introduction-to-shell-check)
   * [Importance of Being Controversial](#importance-of-being-controversial)
      * [Being Controversial](#being-controversial)
   * [Q&amp;A Session](#qa-session)
      * [Questions and Discount Code](#questions-and-discount-code)
   * [Discount Strategies for Technical Books](#discount-strategies-for-technical-books)
      * [Discount Strategies](#discount-strategies)
   * [Wrapping Up](#wrapping-up)
      * [Final Thoughts](#final-thoughts)
   * [Book Publishing and Royalties](#book-publishing-and-royalties)
      * [Book Publishing Contract](#book-publishing-contract)
      * [Book Sales](#book-sales)
   * [Conclusion and Next Steps](#conclusion-and-next-steps)
      * [Wrapping Up](#wrapping-up-1)
      * [Raiding Another Streamer](#raiding-another-streamer)
      * [Future Streaming Schedule](#future-streaming-schedule)
   * [Generated by Video Highlight](#generated-by-video-highlight)

<!-- Created by https://github.com/ekalinin/github-markdown-toc -->
<!-- Added by: runner, at: Mon Apr 24 15:56:32 UTC 2023 -->

<!--te-->

## Introduction

Section Overview: In this section, the speaker introduces the topic of the stream and explains that there may be some
technical issues with the audio.

### Starting the Stream

- The speaker mentions that they will review a crate suggested by Luciano.
- The crate is called GWT Info and it's related to JavaScript web tokens.
- The speaker shares a link to the code in the chat.
- They explain that they will clone the project and add comments to create a commit for Luciano to review.

## Setting Up

Section Overview: In this section, the speaker sets up their environment for reviewing Luciano's crate.

### Cloning Repository

- The speaker opens up a terminal and navigates to their scratch directory.
- They clone their own branch of the repository so as not to interfere with Luciano's work.
- They check out a new branch called "code review."

### Choosing an IDE

- The speaker decides to use C-line instead of VS Code because Rust Analyzer has crashed on them before.

## Reviewing Code

Section Overview: In this section, the speaker begins reviewing Luciano's code.

### Cargo.toml File

- The cargo.tml file is used for getting information about GWT tokens.
- The categories listed in this file may not be necessary.
- Adding "GWT" as a tag could be helpful for others using this library.

### Dependencies

- clap is used for producing command line utilities and clap stands for command line argument parser.
- base64 is pretty basic and version 1.0.0 is being used.
- cedar-json enables serde to understand JSON.
- lazy-static enables global state variables to be added.

## Continuing Reviewing Code

Section Overview: In this section, the speaker continues reviewing Luciano's code.

### Argo.tml

- The argo.tml file is used for getting information about GWT tokens.
- The categories listed in this file may not be necessary.
- Adding "GWT" as a tag could be helpful for others using this library.

### Dependencies

- clap is used for producing command line utilities and clap stands for command line argument parser.
- base64 is pretty basic and version 1.0.0 is being used.
- cedar-json enables serde to understand JSON.
- lazy-static enables global state variables to be added.

## Reviewing Code and Project Structure

Section Overview: The speaker reviews code and project structure, discussing the creation of a mod directory, testing
implementation, and fuzzing tools.

### Reviewing Project Structure

- The team wants a review on project structure.
- The speaker suggests that creating a mod directory for a single module adds unnecessary bureaucracy.
- Testing should be close to the implementation.

### Fuzzing Tools

- Fuzz testing is suggested to test arbitrary input.
- Property testing is recommended for avoiding repetition in tests.
- Rust compiler takes time to compile.

## Asking Questions Live

Section Overview: The speaker encourages viewers to ask questions live during the stream.

### Live Q&A

- Viewers are welcome to ask questions at any stage of the stream.

## [#](t=0:14:11s) Testing JWT Info

Section Overview: In this section, the speaker discusses testing JWT info and how to handle illegal input.

### Testing for Illegal Input

- [](t=0:14:11s) The speaker discusses using a validator for JW2s and testing it with colleague Stefano.
- [](t=0:14:48s) They discuss testing with invalid input, such as non-UTF8 characters.
- [](t=0:15:31s) The speaker asserts that they can handle bad input by giving an arbitrary name to the test case.
- [](t=0:17:20s) They mention another test case for non-UTF8 input and discuss not panicking when encountering illegal
  input.

### Using Quick Check

- [](t=0:18:05s) The speaker mentions libraries like Quick Check that can search through infinite spaces of strings to
  find instances where tests will fail.
- [](t=0:18:46s) They introduce Vic Q8, which stands for an arbitrary vector of u8 bytes.

### Handling Null Bytes

- [](t=0:19:31s) The speaker discusses how strings in Rust must be UTF-8 encoded and introduces the problem of always
  parsing a string.
- [](t=0:20:03s) They experiment with null bytes and control characters to see what happens when parsing them.

## Introduction

Section Overview: In this section, the speaker introduces themselves and welcomes viewers to ask questions in the chat.

### Speaker Introduction

- The speaker introduces themselves and welcomes viewers to ask questions in the chat.

## Project Structure

Section Overview: In this section, the speaker discusses project structure and error management.

### Project Structure

- The speaker suggests putting JWT inside a module directory because it's simple.
- The speaker mentions that they have touched on project structure at the start of the video.

### Error Management

- The speaker talks about great error management and handling error types within Rust.

## Pause Token

Section Overview: In this section, the speaker focuses specifically on pause token.

### Pause Token

- The speaker explains that pause token expects an ampersand stir which is a string slice.
- The speaker mentions their confusion as to why their editor has decided not to provide any assistance. They update
  Rust and restart their ID.
- After resolving technical issues, the speaker continues explaining pause token and provides type hints with color
  syntax highlighting.

## Improving the Ergonomics of Functions

Section Overview: In this section, the speaker discusses how to improve the ergonomics of functions that can only accept
string or ampersands.

### Accepting Arbitrary Input

- The speaker talks about allowing people to put in anything and accepting arbitrary input.
- One idea is to change `parse_token` to `parse_token<T: AsRef<str>>`.
- This means anything that can act as a string will be accepted.
- The type of text and type are examples of things that can act as a string.

### Refactoring Function Declaration

- The speaker makes a small change to the function declaration by adding `as ref`.
- This allows for token as ref but doesn't feel like a large change.
- All tests should still be able to run except for the two added ones that fail.

### Rust's Relationship with Text Types

- Rust has a difficult relationship with text types, which is discussed in detail in one of the speaker's recent talks.
- There are eight different types of strings in Rust, each serving a specific purpose.
- These distinctions are important because they allow Rust to guarantee safety when dealing with operating system parts
  like file paths versus arbitrary string input.

## Limitations of Accepting Arbitrary Input

Section Overview: In this section, the speaker discusses limitations when accepting arbitrary input.

### Problem with Accepting Arbitrary Bytes

- The problem with accepting arbitrary bytes is that it won't be able to become a string.
- The speaker is trying to respond to Luciano's request for a code review and reviewing a crate that provides commandant
  utility.

## Error Management

Section Overview: The speaker discusses error management in Rust and how to deal with errors from multiple locations.

### Dealing with Multiple Error Types

- Every error type is its own thing in Rust.
- Luciano and Stefano created a wrapper type that wraps all of the errors from various crates that might blow up on
  them.
- This is one of the strategies for dealing with errors from multiple locations.

### Custom Error Type

- The speaker suggests adding pointers to the beginning of the slice to enable better error reporting like more helpful
  error messages.
- Luciano and Stefano have created an enum which wraps three different errors plus it also provides its own bespoke
  error.

### Displaying Errors

- The custom enum needs to be able to be printed to the screen, which can be done using format display out of the
  standard library.
- Part era is used to describe which section of the input is invalid.

## Understanding the Code

Section Overview: The speaker discusses the code and suggests improvements to make it more readable.

### Simplifying the Code

- The message format returns a string, which can be used with the right macro.
- There is no functional difference between the commented out variation and the suggested improvement.
- The match syntax provides a value, and each branch returns a string that can be assigned to a variable. This removes
  one element of duplication.

### Improving Readability

- Suggests using local import for multi-line strings to simplify code.
- Considers allowing raids on stream and getting better at streaming.
- Discusses Twitch raiding and suggests doing one at the end of this stream if people want to keep learning about Rust.

### Modularization

- Discusses whether or not things should be in a separate directory. Speaker believes there's no need for it in this
  particular project but acknowledges that it's subjective based on gut feel.

## [#](t=0:47:42s) Simplifying Code with Match Statements

Section Overview: In this section, the speaker discusses simplifying code using match statements and reducing
repetition. They also explore an alternative method of returning a message and avoiding problems with multiple
implementations.

### Implementing Pattern of Simplification

- The speaker plans to re-implement the pattern of simplifying code using match statements.
- They find repetition in code to be problematic and want to reduce it.
- A trick they use is sending the format string and its arguments back.

### Alternative Method for Returning Messages

- The speaker explores an alternative method for returning messages that avoids problems with multiple implementations.
- They acknowledge that this approach may be over-engineered but provide it as an option for exploration.

## [#](t=0:52:08s) Using Rust 2018 Idioms

Section Overview: In this section, the speaker discusses their opinion on using Rust 2018 idioms and whether or not they
should be used if they are working well for you.

### Personal Opinion on Using Rust 2018 Idioms

- The speaker shares their personal opinion on using Rust 2018 idioms.
- They believe that if something is working well for you and makes sense, there is no need to feel obligated to use all
  of the Rust 2018 idioms just because they exist.

## [#](t=0:53:26s) Splitting Out Different Parts of Code

Section Overview: In this section, the speaker discusses how splitting out different parts of code can lead to more
informative error messages.

### Benefits of Splitting Out Different Parts of Code

- The speaker explains how splitting out different parts of code can lead to more informative error messages.
- By breaking down each part into smaller components, errors can be isolated more easily.

## [#](t=0:55:23s) Retaining the Index of Where the Error was Detected

Section Overview: In this section, the speaker discusses retaining the index of where an error was detected in a JWT
library. This information can be useful for debugging purposes.

### Retaining Index of String Input

- The speaker mentions that they would like to retain the index of where an error was detected in a JWT library.
- This information can provide users with more information about where they can fix things.
- The speaker notes that JWTS are almost always automatically generated, so this feature may not be used very often.
- However, if someone were to create their own JWT library, it could be helpful to know where errors are being created.

## [#](t=0:58:07s) Understanding Rust's Result Object

Section Overview: In this section, the speaker explains Rust's Result object and how it is used in error handling.

### Rust's Result Object

- The speaker explains that Rust's Result object is an enum with two states: pass (okay) and fail (error).
- They note that there are generics involved and explain what those are.
- The speaker briefly mentions panicking but notes that they have already dealt with error handling.

## [#](t=1:00:50s) Adding Derives to Types

Section Overview: In this section, the speaker considers adding derives to types and making them comparable.

### Adding Derives

- The speaker suggests adding derives to make types comparable.
- They note that they don't know if this will be useful or not.

## Rust Programming and Cultural Differences

Section Overview: In this section, the speaker discusses the cultural differences between Rust programming and other
languages like Python or JavaScript.

### Bureaucratic Nature of Rust

- Rust can feel very bureaucratic compared to dynamic languages like Python or JavaScript.
- The RFC defines a method for validating JWT, which can be used to add further type safety to input validation code.

### JWT Keto Types

- The speaker looks up JWT keto types in the standard to see if they are defined.
- Type is almost always records JWT.
- Implementing from stir provides fast feedback to users about what's wrong with their input.

### Claims Set

- There is a claims set that needs to be validated.
- Defining an error will allow for more specific validation of string slices becoming a concrete type.

## Derive Debug and Type Conversion

Section Overview: In this section, the speaker discusses how to derive debug and type conversion in Rust.

### Deriving Debug

- To derive debug, add `#[derive(Debug)]` above the struct definition. [](t=1:09:47s)
- This allows for easy debugging of structs by printing them out.

### Type Conversion

- Generic types can be used but it is not recommended. [](t=1:10:20s)
- Use `match` to validate input types. [](t=1:10:34s)
- Implement `FromStr` trait to convert strings into concrete types outside ourselves. [](t=1:14:08s)
- This enables parsing methods without needing to bring in a dependency which is quite nice.
- It becomes very ergonomic as someone who's using a library.
- Exposing a library and CLI together can be done but may not always be necessary.

### Mistake Correction

- The speaker made a mistake earlier when explaining type conversion that was already implemented using Surday string
  serialization system. [](t=1:13:01s)
- The explanation on type conversion was unnecessary but left in for reference purposes.

## IP Addresses and Color Change

Section Overview: In this section, the speaker talks about IP addresses and changing the color of documentation.

### IP Addresses

- Implementing the right trait allows converting raw strings into concrete types outside ourselves which becomes very
  ergonomic as someone who's using a library. [](t=1:14:23s)

### Color Change

- Changing the color of documentation is possible but using dark and light together can be confusing or hurtful to
  people's eyes so it should be done with caution. [](t=1:15:43s)

## Exposing a Library and CLI Together

Section Overview: In this section, the speaker talks about how to stick to Rust and expose a library and CLI together.

### Understanding the Library Component

- The focus of creating a library is understanding what it is that you have.
- The functionality "pub" is public and becomes part of the library.

### Building Documentation

- To build documentation for a crate, use Cargo, which is like Javadoc for Rust.
- Adding comments and usage examples for public functions can improve documentation.

### Improving Library Functionality

- Adding documentation should be the first thing to do when exposing a library.
- Testing should be done to ensure utility resilience against malicious input.

## Testing and Code Coverage

Section Overview: In this section, the speaker talks about testing and code coverage.

### Resilience Against Malicious Input

- It's important to make sure that your utility is resilient against malicious input.

### Code Coverage

- Code coverage is useful in testing.
- Tooling around code coverage can help with testing.

## Build Process and Shell Check

Section Overview: In this section, the speaker discusses the build process and introduces shell check, a linter for
shell code.

### Installing Latest Release Based on Dip

- The speaker suggests downloading and installing the latest available release based on dip.

### Compliance with RSA and Documenting Platforms

- The speaker considers compliance with RSA and documenting platforms, OS, arc CPU architecture.

### Introduction to Shell Check

- Shell check is introduced as a linter for shell code.
- It provides warnings that suggest changes to improve code quality.
- It can be downloaded and installed for free from shellcheck.net.
- The speaker mentions that it has found their code to be pretty good but suggests avoiding camelcase.

## Importance of Being Controversial

Section Overview: In this section, the speaker talks about how being controversial is important and how it can be
nerve-wracking to talk about certain topics.

### Being Controversial

- The speaker believes that if someone cannot talk about controversial topics, they should not be speaking publicly at
  all.
- People tend to avoid talking about controversial topics because it can be scary, but the speaker thinks it's important
  to call things out.
- The speaker encourages people to ask questions and engage in discussions.

## Q&A Session

Section Overview: In this section, the speaker invites questions from the audience and provides a discount code for his
book.

### Questions and Discount Code

- The speaker invites questions from the audience.
- The speaker provides a discount code for his book on wrestlingaction.com.
- The speaker mentions that chapters of his book can also be read for free on lifebook.

## Discount Strategies for Technical Books

Section Overview: In this section, the speaker discusses discount strategies for technical books and their impact on
royalties.

### Discount Strategies

- The speaker discusses how discounts can be a double-edged sword when it comes to selling technical books.
- The speaker suggests that increasing ticket prices may make people more likely to buy discounted versions of technical
  books.
- Alternatively, reducing the full price version may encourage more sales without relying heavily on discounts.

## Wrapping Up

Section Overview: In this section, the speaker wraps up the stream and invites people to hang out on his Discord server.

### Final Thoughts

- The speaker mentions that being a new streamer is fun because there are no expectations.
- The speaker reflects on the success of his book in terms of reputation but notes that it has not provided financial
  stability.
- The speaker provides information about his book sales and royalties.
- The speaker invites people to hang out on his Discord server.

## Book Publishing and Royalties

Section Overview: In this section, the speaker talks about their experience with book publishing and royalties.

### Book Publishing Contract

- The contracts for first-time authors are structured differently.
- The speaker accepted a contract that gave them 10% of net royalties.
- Net royalties means the amount left after costs such as affiliate fees and credit card payment fees are deducted.

### Book Sales

- The book is selling well, but most people buy it heavily discounted.
- After fees, the speaker earns about $2 per title sold.

## Conclusion and Next Steps

Section Overview: In this section, the speaker concludes their stream and provides information on how to connect with
them offline.

### Wrapping Up

- The speaker thanks viewers for hanging out on the stream.
- They provide links to their Discord server and Twitter account for offline connections.

### Raiding Another Streamer

- The speaker decides to raid another streamer named Luis who has only one viewer at the moment.
- They provide a link to Luis's channel for viewers to hang out there instead.

### Future Streaming Schedule

- The speaker plans to stream once or twice a week at different times to accommodate viewers in different time zones.

## Generated by Video Highlight

https://videohighlight.com/video/summary/1JZ1oWp3PuA