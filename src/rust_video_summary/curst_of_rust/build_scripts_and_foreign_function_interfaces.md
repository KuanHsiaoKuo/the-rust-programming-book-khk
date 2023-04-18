# Build scripts and foreign function interfaces

<!--ts-->


<!-- Created by https://github.com/ekalinin/github-markdown-toc -->
<!-- Added by: runner, at: Tue Apr 18 03:41:52 UTC 2023 -->

<!--te-->

## Introduction

Section Overview: In this section, the speaker introduces the topic of the stream which is build scripts and foreign
function interfaces (FFI). The speaker explains that build scripts are a program that runs before a crate is compiled,
and FFI is how Rust code interacts with code written in other languages.

## Understanding Build Scripts

Section Overview: In this section, the speaker talks about build scripts and their usefulness.

### What are Build Scripts?

- Build scripts are programs that run before a crate is compiled.
- They are a general mechanism that can be hard to understand.
- They have limitations and patterns they usually apply when used.

### Why Use Build Scripts?

- They allow for customization of the build process.
- They can be used to generate code or perform other tasks before compilation.

## Foreign Function Interfaces (FFI)

Section Overview: In this section, the speaker talks about FFI and its importance in Rust programming.

### What is FFI?

- FFI stands for foreign function interface.
- It's how Rust code interacts with code written in other languages.
- There aren't many great resources available on it.

### Linking Against C

- The mechanisms discussed apply to linking against any C-compatible API, including C++.
- Bridging the gap from the C API to another language has nuances related to Rust as well.

## Bindings to Lib Sodium

Section Overview: In this section, the speaker discusses bindings to lib sodium cryptography library written in C.

### About Lib Sodium

- Lib sodium is a widely-used cryptography library written in C.
- It aims to provide implementations that are hard to misuse and relatively low in configuration.
- The API is easy to bind to because it's written entirely in C.

### Writing Bindings

- Build scripts can be used to write bindings to external libraries.
- The speaker will use lib sodium as an example of how the pieces fit together.

## Conclusion

Section Overview: In this section, the speaker concludes the stream and summarizes what was covered.

### Summary

- Build scripts are programs that run before a crate is compiled and allow for customization of the build process.
- FFI is how Rust code interacts with code written in other languages, and there aren't many great resources available
  on it.
- Linking against C-compatible APIs has nuances related to Rust, but the mechanisms discussed apply to any C-compatible
  API.
- Lib sodium is a widely-used cryptography library written in C that provides implementations that are hard to misuse
  and relatively low in configuration.

## Understanding Build Scripts in Rust

Section Overview: This transcript explains what build scripts are and how they work in Rust. It also covers the
communication between build scripts and crates being compiled.

### What is a Build Script?

- A build script is a program that defines a main function, which serves as the entry point to the script.
- The build script must be written in such a way that it can run on other people's computers.
- The output of the build script is not included in cargo publish, so it must be generated in such a way that it can be
  used by others.

### Communication Between Build Scripts and Crates

- The primary means of communication between build scripts and crates being compiled is through environment variables
  and the out directory.
- When a build script runs, it gets access to the out directory environment variable, which is writable by the build
  script.
- After running the build script, the out directory environment variable will be set for the crate being compiled to
  that same directory.
- This allows for generated source code to be included in rust code using things from under outdoor.

### Printing Output from Build Scripts

- By default, output from successful builds is not printed to the terminal unless there is an error.
- This is because build scripts tend to produce large amounts of output, including special instructions for Cargo like
  changing link search paths or setting environment variables.
- However, it is possible to get this output even if there are no errors by unsetting cargo Target dur and running cargo
  run again.

## Understanding the Build Script

Section Overview: This section explains how to compile a build script and view its output.

### Compiling the Build Script

- The build script is compiled, resulting in a crate that includes the output and standard error of its build script.
- Running `cat Target` shows debug build, an ffi AFC output.

### Viewing Build Script Output

- Standard output is empty, but standard error contains the output from the debug macro.
- When building something that depends on another crate with a build script (e.g. openssl sys), use this method to view
  the output of that build script.
- The out directory under target/debug/build contains generated sources for both the build script and the crate being
  built.

## Using Environment Variables in Build Scripts

Section Overview: This section explains how to use environment variables in Rust build scripts.

### Setting Environment Variables

- The outdoor environment variable is set when running a Rust build script.
- Use NV instead of standard end VAR because NV reads environment variables at compile time rather than runtime.

### Using Environment Variables

- To use an environment variable in practice, join it with standard FS and include it using the include macro.
- The include macro substitutes a call with file contents at compile time.

## Understanding Build Scripts and Cargo

Section Overview: In this section, the speaker explains the connection between build scripts and cargo. They also
discuss how build scripts can communicate with a crate being compiled using various methods.

### Connection between Build Scripts and Cargo

- The mod Foo now just contains pubfenfu, which is the result of running cargo expense.
- Include is similar to pound includency. They are basically the same thing.

### Communication Methods for Build Scripts

- Outdoor is one way that build scripts can communicate with a crate being compiled. There are others.
- By printing to standard out something that begins with cargo colon, build scripts can communicate with cargo.
- Rusty link ARG is used to pass additional flags to the linker. It can be used to tell cargo where to find a particular
  shared library file.
- Rusty link search sets a search path for libraries.
- Rusty link lib tells cargo which shared library to link against.
- Rusty config passes additional flags to rust C itself. It's often used in conditional compilation based on what these
  flags are passed in.

### Config Aspect of Cargo Options

- Config macro is used for compile-time properties and conditional compilation based on features enabled by passing
  additional config flags like let's say you know openssl110 which indicates it's at least version 1.1.0
- Rusty config allows passing additional config Flags so that crates can inform about conditional compilation options
  they might need to enable or disable.

## Distinction between Cargo and Rusty

Section Overview: In this section, the speaker explains the distinction between cargo things and rusty things. They also
discuss how config is a rust sea language feature that allows conditional compilation based on properties passed to it.

### Distinction between Cargo and Rusty

- Config is a rust sea language feature of just saying you can conditionally compile based on properties that are passed
  to it.
- Config is no longer related to cargo but allows conditional compilation based on what these flags are passed in.

## Rusty Config Flags

Section Overview: In this section, we learn about Rusty config flags and how to use them for conditional compilation.

### Using Rusty Config Flags

- Rusty config flags can be used for conditional compilation.
- Config flags are definitive and can be accessed through a macro.
- To access the config flag macro, use `config` followed by the desired value.

## Communicating Between Build Scripts and Cargo

Section Overview: This section covers two ways of communicating between build scripts and cargo: Cargo warning and Cargo
key-value pairs.

### Cargo Warning

- The output from build scripts does not get printed when run unless the build fails.
- If a build script writes to standard out a line that starts with `cargo:warning=`, then the remainder of that message
  gets printed when cargo runs.
- Warnings are intended for warnings, not logging.

### Cargo Key-Value Pairs

- Metadata declared using `cargo:key=value` syntax turns into environment variables.
- Downstream builds can consume these environment variables.
- The name of the environment variable is `dep_links_attribute_key_value`.
- Use links attribute in your crate's cargo.toml file to declare shared library dependencies.

## Links Attribute in Crate's Cargo.toml File

Section Overview: This section explains how to use links attribute in your crate's cargo.toml file to declare shared
library dependencies.

### Declaring Shared Library Dependencies

- Declare links equal in the name of the shared library in that crates cargo.tamil file.
- Doing this allows cargo to check that only one crate in the entire dependency graph links against the shared library.

## Rust Build Scripts

Section Overview: This transcript discusses the implications of cargo checks and how to avoid having multiple crates
that provide bindings to the same library. It also explains how to generate a ciscrate and the importance of avoiding
major version bumps.

### Single Crate for Shared Libraries

- A single crate called Dash sis is used for most shared libraries in Rust.
- The crate only provides bindings as the shared library and exposes raw ffi methods.
- Multiple libraries can use this sys library and generate nice bindings on top of it.
- Having a single ciscrate ensures that you only link against it in one place, which has implications for semantic
  versioning.

### Semantic Versioning

- If you cut a breaking release to CIS, every consumer of that ciscrate should also bump to that new version at the same
  time.
- Major version bumps should be avoided because two different crates both have the links keyword for the same name,
  causing Cargo to complain and not build.

### Rerun if Changed

- Build scripts emit either rerun if changed or rerun if n changed or both.
- Rerun if n changed is used for things like openssl libder which is used to locate openssl.
- Rerun if changed is usually used when your build script compiles a little C program into a shared Library file or
  static library.

### Sharp Tools

- Build scripts are very sharp tools and can easily hurt yourself with them because they are not sandboxed in any
  meaningful way at least not at the moment.

## Understanding Build Scripts

Section Overview: In this section, the speaker talks about build scripts and their implicit trust. They also discuss the
need to sandbox build scripts in meaningful ways.

### Build Scripts for Common Crates

- The speaker discusses a crate called gitu that binds against libgit2, which is a re-implementation of Git in C.
- The speaker looks at the cargo.toml file for libgit2sis, which declares links equals get two and has a bunch of
  features.
- The speaker examines the build.rs file for libgit2sis and explains how it tries to locate the library in question on
  standard system paths before building it from source if necessary.
- The speaker explains that after obtaining the shared library, bindgen is used to generate Rust bindings to that shared
  library.

### Environment Variables and Features

- The speaker reads out a bunch of environment variables commonly used by crates when building dependencies.
- The speaker mentions that some crates have a feature that determines whether or not to vendor the crate in question.

## Understanding Package Config

Section Overview: This section explains how package config works and how it is used to locate shared libraries.

### Using Package Config to Locate Shared Libraries

- If the vendored flag is not set, cargo will use package config to locate shared libraries.
- The package config crate is a thin wrapper around a command that ships on most Unix systems called package config. It
  provides information about a library's link properties.
- The Linker Flags are emitted by package config and include information such as version requirements and whether to
  link against the library statically or dynamically.
- Syscrates use package config to locate shared libraries rather than implementing their own mechanism for searching
  through user lib and the like.

### How Package Config Works

- When using the crate, it tells you whether a given library was available and outputs all of the necessary cargo
  standard out instructions for a build script to set the link search path and linker args.
- Package config uses files called "package config files" which contain information about where the library is located,
  its lib directory, include directory, name of the library, version, and additional link properties.
- If cargo does not find a library on the system path, it emits Rusty Config saying that it has been vendored. This
  allows conditional compilation on whether or not to vendor.

## Building Libgit2 from Source

Section Overview: This section discusses the steps needed to build libgit2 from source and how it is included in the
Libya 2 CIS crate.

### Building from Source

- In order to vendor, it needs to have the source for the library that it's going to build.
- If the sub module hasn't been checked out then run sub module update in it so that you get access to the submodule.
- When you run cargo publish by default will include anything that's not ignored by git ignore so if they haven't get
  ignored libgit2 all of Libya 2 is going to include it as well.
- The Libya 2 CIS crate Source tarball on crates.io includes the source code for libgit2.

### Steps Needed to Build Libgit2

- It uses outdoor to figure out where to build it like where you know what the scratch directory is essentially for
  building artifacts.
- Figure out what Target you're compiling for they use the CC crate which is a really nice crate that's a wrapper around
  a standard C compiler that knows about all of the standard environment variables like CC and AR and um uh LD and LD
  flags and C flags and cxx flags and cxx like all of those things that you know Sea World have accepted as things we
  use for um for compiling C code.
- Ultimately somewhere down here so you see there are a lot of steps um
- At least as long as that succeeds, it's gonna then emit all of the necessary Rusty link properties are needed which
  the CC crate also takes care of emitting.

## Naming Conventions in Rust

Section Overview: This section discusses naming conventions in Rust and why the API for package config uses the name
static with a K instead of C.

- Static with the c is a keyboard in Rust so you're not allowed to use it for function names.
- Most people just use like the standard is you know instead of static you use static tick instead of class instead of
  create.

## Pre-built Libraries

Section Overview: This section discusses whether users have to build the C library on their machines or if they can
publish a pre-built.so.

- Cargo doesn't prevent you from including a DOT so in your build artifact um and in very rare cases, it's a good idea
  usually not so's but you'll see this with like more like dot Os or dot A's where.

## Building and Linking Against libgit2

Section Overview: In this section, the speaker discusses how to build and link against libgit2 in Rust.

### Building Against Different Targets

- When building on one target and running on another, differences in targets or library versions can cause issues.
- It is generally safer to use whatever libraries are available on the target machine rather than trying to build for
  all possible consumers.

### Linking Against libgit2

- The code in `build.rs` tells Cargo how to link against libgit2.
- However, it does not explain how to call functions from Rust.
- One way to do this is by using `extern C` declarations in a separate Rust file that has equivalent types and extern
  offenses generated by bindgen.

### Using Bindgen

- Bindgen is a tool that automatically generates Rust FFI bindings to C libraries based on their header files.
- By calling bindgen on a C header file containing function definitions and type definitions for an interface, it will
  generate a Rust file with equivalent types and extern offenses.
- This is useful when working with stable libraries like libgit2.

## Understanding Extern Keyword and FFI

Section Overview: In this section, the speaker explains how the `extern` keyword works in Rust's foreign function
interface (FFI).

### Extern Keyword

- The `extern` keyword changes the calling convention used for a function call. It also indicates that the function is
  defined elsewhere so just look in the symbol table of the binary if you see a call to this actually called that.

- When using `extern C`, all arguments must be valid C equivalent arguments, so any struct used must be wrapped in C.

### Opaque Types

- Sometimes, an opaque type is used to indicate that the internals of a type are none of our business. This is done by
  declaring a `Pub enum` with an empty value.

## Rust Bindings for C Libraries

Section Overview: This section discusses the two main approaches to creating Rust bindings for C libraries: manually
writing the bindings and using a tool like bindgen to generate them automatically.

### Manual Binding Writing

- The manual approach involves defining Rust structs that mirror the layout of the corresponding C structs.
- This approach provides more control over the naming and layout of each field in the struct.
- However, it requires updating the bindings if changes are made to the underlying library.

### Automatic Binding Generation with Bindgen

- Bindgen is a tool that generates Rust bindings from C header files automatically.
- It can produce convoluted bindings that are difficult to read but are generally correct.
- Bindgen-generated bindings may not be stable over time, which can cause backwards compatibility issues for crates that
  rely on them.

## Using Bindgen in Build Scripts

Section Overview: This section demonstrates how to use bindgen in build scripts to generate Rust bindings for C
libraries.

### Steps for Using Bindgen in Build Scripts

- Add bindgen as a build dependency in Cargo.toml.
- Create a wrapper.h file that includes the necessary headers for your library.
- In build.rs, use bindgen to generate Rust bindings based on wrapper.h and write them out to a file (e.g. bindings.rs).
- Run cargo build or cargo run to trigger the build script and generate the bindings.

### Benefits and Drawbacks of Using Bindgen

- Using bindgen simplifies binding generation by automating most of the process.
- However, it can produce unstable bindings that may change over time due to updates in bindgen itself.

## Generating Rust Bindings with Bindgen

Section Overview: This section discusses how to generate Rust bindings using Bindgen.

### Using Bindgen to Generate Rust Bindings

- Bindgen is a tool used to generate Rust bindings for C and C++ libraries.
- It takes the header file of a library and generates equivalent Rust bindings for all types and functions in the header
  file.
- The generated bindings are marked as public, which means that if they are published as a crate, any changes made to
  them will require a major version change.
- This can be problematic because it requires everyone who uses the crate to also update their code.
- To avoid this issue, it is recommended to find ways to ensure that your public interface remains stable over time.
- One way to do this is by creating private modules that include only specific bindings you want to expose publicly.
- Another way is by using blacklists or whitelists in your build.rs file to limit what bindgen generates bindings for.

## Walking Through Build Scripts of Existing Crates

Section Overview: This section walks through the build scripts of existing crates.

### SSH 2 Crate

- The SSH 2 crate generates bindings for the libssh2 C library and has two crates: ssh2 and libssh2-sys.
- The crate has manually written bindings because it wants control over its stability over time.
- The patterns used in these manually written bindings are similar to those generated by bindgen.

### Libgit2 Crate

- The libgit2 crate also has manually written bindings.
- The bindings are checked into the repository to avoid generating them automatically on every build.

### Libsodium Crate

- The libsodium crate uses bindgen to generate its Rust bindings.
- The generated bindings are checked into the repository and tuned manually for stability over time.

## Bindgen and Builder

Section Overview: This section discusses how to use bindgen and the Builder documentation. It also covers the
configuration options available in Builder.

### Using Bindgen and Builder

- Bindgen doesn't know how to treat certain types, so we need to specify it using the Builder documentation.
- The Builder documentation has many configuration options such as generating Rust code for C enums, allowing/blocking
  specific items, injecting arbitrary Rust or C code, marking a type as opaque, etc.
- These configurations are useful when working with libraries like lib sodium.
- Other configuration options include type aliases, deriving different traits for FFI types, supporting namespacing,
  handling callbacks.

## SSH2 Build Script

Section Overview: This section covers the build script of SSH2 library and its similarities with other libraries like
Libgit2.

### SSH2 Build Script

- The build script of SSH2 library is similar to that of Libgit2.
- By default, libssh2 does not use package config but requires you to opt-in. However, if allowed to use package config
  it will automatically emit necessary build instructions for cargo.
- The build script sets include paths and uses environment variables like cargo colon key equals value.
- It creates a config SCC build into outdoor and checks out libssh2 as a submodule before building it from source.
- On Windows using MSVC you can't use package config so VC package is used instead.

## OpenSSL Crate

Section Overview: This section covers the build script of OpenSSL crate and its complexities.

### OpenSSL Crate

- The build script of OpenSSL crate is complex due to the many versions and variants of OpenSSL.
- It finds and links against OpenSSL, determines whether to build against it statically or as a shared library.
- It reads an environment variable called open SSL static to determine mode.

## Dynamic vs Static Linking

Section Overview: This section discusses the differences between dynamic and static linking, and how libraries are
linked in Rust.

### Linking Libraries

- If only the `.a` exists, then we statically link.
- If only the `.so` exists, then we dynamically link.
- If it contains both, then we do dynamic linking.

### Finding OpenSSL

- In `main`, `find_openssl()` is called to find OpenSSL.
- If the vendored feature is enabled and not explicitly set to not vendor through an environment variable,
  use `find_vendor_module()` to get OpenSSL.
- Otherwise, find it from system paths using packageconfig.

### To Vendor or Not to Vendor?

- Vendoring is convenient for consumers but building from source can be complicated and error-prone.
- It's better to issue an error telling users to install a library instead of trying to build it from source if it's too
  complicated.
- Best practice is to have a feature that lets people opt into vendoring if they specifically need it. An environment
  variable overrides this feature saying never vendor.

## Lib Sodium

Section Overview: This section talks about lib sodium and its Rust bindings.

### Lib Sodium Bindings

- Lib sodium has Rust bindings in a crate called `libsodium-sys`.
- The crate uses package config and vendors by building from source like other libraries.

## Introduction

Section Overview: In this section, the speaker discusses how to structure a crate and mentions vendoring.

### Crate Structure

- The ciscrate should only contain the FFI bindings.
- Another crate should provide safe and ergonomic wrappers around it.
- Vendoring usually means building from source.

## Installing Lib Sodium

Section Overview: In this section, the speaker installs libsodium and creates a new cargo project for it.

### Installation

- The speaker has already installed libsodium on their system.
- They create a new cargo project called "sodium".
- They add links equals sodium in their cargo.toml file.
- They copy over the build dependencies from another project.

## Continuing with Lib Sodium

Section Overview: In this section, the speaker continues working with lib sodium by copying over files and discussing
API stability.

### Copying Files

- The speaker copies over wrapper.h which doesn't include sodium.h
- They also copy over build.rs but are not too concerned with API stability for this one.

### Package Config

- The speaker uses package config to generate link search and link lib for them.
- They add package config as another build dependency in their cargo.toml file.
- It is recommended to have version requirements when generating bindings based on C header files.

## Generating Bindings Based on Lib Sodium

Section Overview: In this section, the speaker generates bindings based on lib sodium using bindgen.

### Allow List Function

- The speaker allows list function so that they can generate bindings based on that function only.

### Bindgen Docs Builder

- The tutorial is wrong because there is a newer version of binding.

### Build

- If we try to do something like build with verbose uh touchwrapper.h which is going to redo a bunch of the things what
  we'll see here is at the build at the end you'll see it includes uh.

## Opting Out of Shared Libraries

Section Overview: In this section, the speaker discusses opting out of shared libraries and how it can cause
hard-to-debug problems. They then demonstrate how to opt-out by setting `print system Libs` to false.

### Opting Out of Shared Libraries

- Opting out of shared libraries is important because it can cause hard-to-debug problems.
- To opt-out, set `print system Libs` to false.
- Demonstrates that `Dash L sodium` works because lip sodium is in user lip for them.

## Generating Bindings for Sodium Library

Section Overview: In this section, the speaker talks about generating bindings for the Sodium library using build.rs and
mod ffi.

### Generating Bindings for Sodium Library

- The build.rs file generates bindings for the Sodium library.
- Demonstrates how to include the generated bindings using `mod ffi`.
- Allows linking against package config on just that module.

## Understanding Generated Bindings

Section Overview: In this section, the speaker explains what was generated by bindgen and shows how to use it.

### Using Generated Bindings

- Demonstrates how to use `Pub use ffi sodium in it`.
- Shows what's in the generated binding file.
- Explains that every extern function is inherently unsafe because we don't know whether the signature even matches what
  the real function does much less what that function does in the first place.

## Initializing the Sodium Library

Section Overview: In this section, the speaker talks about initializing the Sodium library and using package config to
link against it.

### Initializing the Sodium Library

- Explains that `sodium in it` initializes the library and should therefore be called before any function provided by
  lib sodium.
- Talks about using a crate called ctor to annotate a function with ctor and run it before main as long as your crate is
  actually included.
- Explains that if there's an error when using something like ctor, there's no good way to report it to the user.

## Introduction to FFI

Section Overview: In this section, the speaker introduces the concept of Foreign Function Interface (FFI) and explains
how it can be used to interact with C libraries from Rust.

### Using FFI to Interact with C Libraries

- FFI allows Rust code to interact with C libraries.
- To declare a function that uses a C library, we need to use the `libc` crate.
- The Crypto Generic Cache API is a good starting point for writing bindings from other languages.
- We can enforce that `sodium_init()` has been called by using an `init` function.

## Implementing FFI in Rust

Section Overview: In this section, the speaker demonstrates how to implement FFI in Rust by creating bindings for the
Sodium library.

### Creating Bindings for Sodium Library

- We create bindings for the Sodium library using the `bindgen` crate.
- We test our implementation by calling `sodium_init()` and checking if it returns an error or not.
- Our implementation links against lib sodium and works as expected.

## Tips for Writing Wrappers for C Libraries

Section Overview: In this section, the speaker provides tips on writing wrappers for C libraries.

### Tips for Writing Wrappers

- You don't need to understand the C code itself as much as you need to understand its API.
- With tools like bindgen, you might not even need to understand the API since it generates equivalent Rust types
  automatically.
- The ergonomic interface should not be in the same crate as the FFI bindings.

## Understanding the Crypto Generic Hash Function

Section Overview: In this section, the speaker discusses the implementation of the Crypto Generic Hash function and how
to enforce invariance in Rust code.

### Implementation of Crypto Generic Hash Function

- The crypto generic hash function puts a fingerprint of the message whose length is Inland bytes into out. The output
  size can be chosen by the application.
- There's a constant for minimum recommended output size that should be allowed list VAR.
- These are things that we should turn into invariance in our Rust code that we assert and panic on if they're wrong.

### Using C int instead of u32

- C int is used instead of u32 because they are not necessarily the same as u32. Sometimes, C ones are
  platform-dependent, and it's essential to capture that in generated bindings.

### Asserting Output Size

- The output will be a mute u8.
- We want to assert that out dot Len is greater than or equal to expected user found U32.
- To ensure that if the type of this ever changes in the future, we could get an error saying that it might not fit in U
  size.

### Enforcing Minimum Recommended Key Size

- Key can be null, but if some key is provided, then we're going to assert that its length is key bytes.

## Converting Key to Pointer

Section Overview: In this section, the speaker explains how to convert a key to a pointer.

### Converting Key to Pointer

- To convert a key to a pointer, use `key.as_ptr()` and `key.len()`.
- If `let some_key = key`, then the code would be: `some_key.as_ptr()` and `some_key.len()`.
- The result will either be a pointer or zero.

## [Working with FFI in Rust](t=7009s)

Section Overview: In this section, the speaker demonstrates how to work with Foreign Function Interface (FFI) in Rust by
using an example of hashing arbitrary data.

### Setting up Hashing API

- Use `crypto_generic_hash` function from libsodium library to hash arbitrary data.
- Set input as arbitrary data to hash.
- Use byte string format by adding B prefix.
- Declare output as mutable variable and set it equal to the hash of the input using `crypto_generic_hash`.
- Use `hex` crate to print out the hash code in hexadecimal format.

### Testing Hashing API

- Run tests using `cargo tests`.
- Verify that the hash generated by Rust's FFI is identical to that generated by a real blake2 implementation.
- Compare results with B2 sum.

### Working with C Code

- To call Rust code from C, declare an external function in Rust and pass it as a function pointer to C.
- The process is similar for calling C code from Rust.

## Rust and C FFI

Section Overview: This section covers the basics of Rust and C Foreign Function Interface (FFI).

### Return Types and Memory Allocation

- Rust return types should be a valid representation in C to match what the C code expects to call.
- Be careful with memory allocation. If you allocate memory in Rust, make sure it gets freed and dropped in Rust. If
  it's allocated in C, make sure it gets deallocated in C as well.
- Keep track of where allocations happen. It's best if all allocations happen either in the C code or all happen in the
  Rust code rather than mixing them up.

### No Mangle

- Use no mangle on a function so that its name ends up exactly like this in the final binary symbol table.
- Declare functions as no mangle to ensure they get included in the final binary.

### FFI Going Both Ways

- There is nothing more special about FFI going the other way. Just make sure you match calling convention, types, and
  representations.

## Borrowing and Lifetimes

Section Overview: This section covers borrowing and lifetimes when using Rust with FFI.

### Duplicating References

- The return type is tied to the same lifetime as `this`. If someone gives a mutable borrow of `this`, they are not
  allowed to continue using it mutably as long as this value still lives.
- Restricting the thing returned to read-only can help prevent errors.

### Testing Borrowing Restrictions

- Trying to modify `out` after borrowing it mutably will result in an error.

## Build Scripts and Auto Config

Section Overview: This section covers build scripts and auto config in Rust.

### Build Scripts

- Use build scripts for bundled C libraries, finding C libraries, generating Rust modules, and platform-specific
  configuration.

### Auto Config

- Auto config captures compiler feature detection. It does what a bunch of different crates were previously using
  build.rs4 to do.

## Auto-Configure and C Bind Gen

Section Overview: This section covers the use of Auto-Configure and C Bind Gen in Rust programming.

### Auto-Configure

- Auto Configure is a tool used to do detection of conditional compilation properties in build.rs.
- It can be used for detecting whether a given type is available, getting the sysroot from rust C, checking whether a
  trait exists, or checking whether a constant exists.
- Auto Configure is intended entirely to be used in build.rs and it's used by things like anyhow to figure out if it can
  use nightly features.

### Config Accessible and Config Version

- Config Accessible and Config Version are two features that are being worked on under the Rust language.
- They will allow conditional compilation based on whether a given path (type, trait, constant) is available in the
  current version of Rust being used.
- This will eliminate the need for Auto Configure for many cases.

### C Bind Gen

- C Bind Gen generates C header files that let you call Rust code from other languages like Python or Node.js that
  expect to use the C ABI.
- It takes a Rust API as input and generates corresponding C header files.
- If you have a Rust library built as a shared library, you can generate a c header file that other languages can use to
  call your Rust code through the cabi.

### CXX

- cxx allows building an ergonomic interface between specifically rust and c++ by defining the bridge between these two
  languages.
- It requires changes on both sides of the interface but generates bindings that are much nicer for each language.

## Conclusion

Section Overview: This section concludes the video and provides information on where to find more resources.

- The speaker thanks everyone for attending and mentions that there are examples of many of the things discussed in the
  Cargo book under build script examples.
- The speaker invites viewers to follow them on Twitter or join the Discord server for Call of Rustation Station podcast
  to keep an eye out for future streams.
- The video ends with a farewell message.

## Generated by Video Highlight

https://videohighlight.com/video/summary/pePqWoTnSmQ