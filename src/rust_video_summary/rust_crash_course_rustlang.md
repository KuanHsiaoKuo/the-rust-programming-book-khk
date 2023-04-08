# Rust Crash Course | Rustlang

<!--ts-->
* [Rust Crash Course | Rustlang](#rust-crash-course--rustlang)
   * [Introduction to Rust Programming Language](#introduction-to-rust-programming-language)
      * [What is Rust?](#what-is-rust)
      * [Garbage Collection](#garbage-collection)
      * [Cargo Package Manager](#cargo-package-manager)
   * [Installing Rust and Setting Up the Environment](#installing-rust-and-setting-up-the-environment)
      * [Installing Rust](#installing-rust)
      * [Basic Utilities](#basic-utilities)
      * [Creating a Rust File and Compiling It](#creating-a-rust-file-and-compiling-it)
      * [Initializing a Project with Cargo](#initializing-a-project-with-cargo)
   * [Introduction to Print Line Command and Formatting](#introduction-to-print-line-command-and-formatting)
      * [Creating a Function in Print File](#creating-a-function-in-print-file)
      * [Running the Function in Main RS Files](#running-the-function-in-main-rs-files)
      * [Basic Formatting](#basic-formatting)
   * [Basic Formatting with Multiple Placeholders](#basic-formatting-with-multiple-placeholders)
      * [Using Multiple Placeholders](#using-multiple-placeholders)
      * [Positional Arguments](#positional-arguments)
      * [Named Arguments](#named-arguments)
   * [Placeholder Traits](#placeholder-traits)
      * [Binary Trait](#binary-trait)
      * [Hexadecimal Trait](#hexadecimal-trait)
      * [Octal Trait](#octal-trait)
   * [Debug Traits](#debug-traits)
      * [Using Print Line Function](#using-print-line-function)
      * [Basic Math](#basic-math)
   * [Variables](#variables)
      * [Creating Variables](#creating-variables)
      * [Mutable Variables](#mutable-variables)
      * [Constants](#constants)
      * [Assigning Multiple Variables at Once](#assigning-multiple-variables-at-once)
   * [Introduction to Rust Programming Language](#introduction-to-rust-programming-language-1)
      * [Assigning Variables](#assigning-variables)
      * [Data Types](#data-types)
      * [Boolean Expressions](#boolean-expressions)
   * [Rust Primitive Types](#rust-primitive-types)
      * [Char and Unicode](#char-and-unicode)
   * [Strings in Rust](#strings-in-rust)
      * [Creating Strings](#creating-strings)
      * [Modifying Strings](#modifying-strings)
   * [Working with Strings](#working-with-strings)
      * [Checking for Substrings and Replacing Them](#checking-for-substrings-and-replacing-them)
      * [Looping Through Strings](#looping-through-strings)
      * [Creating Strings with Capacity](#creating-strings-with-capacity)
      * [Assertion Testing](#assertion-testing)
   * [Working with Tuples](#working-with-tuples)
      * [Introduction to Tuples](#introduction-to-tuples)
      * [Creating and Accessing Tuples](#creating-and-accessing-tuples)
      * [Destructuring Tuples](#destructuring-tuples)
      * [Tuple Methods](#tuple-methods)
   * [Tuples and Arrays in Rust](#tuples-and-arrays-in-rust)
      * [Creating Tuples](#creating-tuples)
      * [Creating Arrays](#creating-arrays)
      * [Modifying Array Values](#modifying-array-values)
      * [Debugging Arrays](#debugging-arrays)
   * [Slicing Arrays and Using Vectors](#slicing-arrays-and-using-vectors)
      * [Slicing Arrays](#slicing-arrays)
      * [Using Vectors](#using-vectors)
   * [Mutating Values in Vectors](#mutating-values-in-vectors)
   * [Conditionals](#conditionals)
      * [If-else statements](#if-else-statements)
      * [Operators](#operators)
      * [Shorthand If](#shorthand-if)
   * [Loops](#loops)
      * [Infinite Loop](#infinite-loop)
      * [While Loop](#while-loop)
      * [For Loop](#for-loop)
   * [Introduction to Loops and Functions](#introduction-to-loops-and-functions)
      * [While Loop](#while-loop-1)
      * [FizzBuzz Challenge](#fizzbuzz-challenge)
      * [For Range Loop](#for-range-loop)
      * [Functions](#functions)
   * [Rust Functions](#rust-functions)
      * [Creating a Function](#creating-a-function)
      * [Returning from a Function](#returning-from-a-function)
      * [Closures](#closures)
   * [Pointers and References](#pointers-and-references)
      * [Primitive Arrays](#primitive-arrays)
      * [Non-primitive Values](#non-primitive-values)
   * [Rust Basics: References and Structs](#rust-basics-references-and-structs)
      * [Creating References](#creating-references)
      * [Creating Structs](#creating-structs)
      * [Tuple Structs](#tuple-structs)
   * [Creating a Struct in Rust](#creating-a-struct-in-rust)
      * [Creating a New Person](#creating-a-new-person)
      * [Getting Full Name of Person](#getting-full-name-of-person)
      * [Changing Last Name of Person](#changing-last-name-of-person)
      * [Converting Name to Tuple](#converting-name-to-tuple)
   * [Structs](#structs)
   * [Introduction to Structs](#introduction-to-structs)
   * [Enums](#enums)
      * [Introduction to Enums](#introduction-to-enums)
      * [Using Enums](#using-enums)
   * [Functions with Enums](#functions-with-enums)
      * [Creating Functions with Enums](#creating-functions-with-enums)
   * [Command Line Arguments](#command-line-arguments)
      * [Introduction to Command Line Arguments](#introduction-to-command-line-arguments)
      * [Getting Command Line Arguments](#getting-command-line-arguments)
   * [Introduction to Command Line Applications](#introduction-to-command-line-applications)
      * [Getting Input from Command Line Arguments](#getting-input-from-command-line-arguments)
      * [Creating a Command Line Application](#creating-a-command-line-application)
   * [Conclusion](#conclusion)

<!-- Created by https://github.com/ekalinin/github-markdown-toc -->
<!-- Added by: runner, at: Sat Apr  8 16:38:12 UTC 2023 -->

<!--te-->

## Introduction to Rust Programming Language

Section Overview: This video is an introductory course on the fundamentals and syntax of the Rust programming language.
The instructor will cover what Rust is, its relevance in web development, and how it compares to other systems
languages.

### What is Rust?

- Rust is a fast and powerful programming language known for being a systems language.
- It's best suited for building drivers, compilers, and other tools that programmers use in development.
- Rust is becoming relevant in web development because of WebAssembly, which allows us to build secure, portable, and
  fast web applications using languages like C++ and Rust.

### Garbage Collection

- One of the biggest advantages of Rust is that it doesn't have garbage collection.
- In JavaScript, for example, garbage collection can take multiple seconds depending on the program.
- With languages like C++ you have to manage all memory allocation yourself which makes programming much harder.
- In contrast with both these approaches, Rust checks memory usage only when needed. If the heap gets close to being
  full or above some threshold it will then look for variables to free up memory.

### Cargo Package Manager

- Rust has its own package manager called Cargo which is similar to NPM for Node.js or Composer for PHP.

## Installing Rust and Setting Up the Environment

Section Overview: This section covers how to install Rust, set up the environment, and use some of the basic utilities.

### Installing Rust

- To install Rust, run the installer and hit one to proceed with the default installation.
- If rustup --version returns "not found," restart your terminal.

### Basic Utilities

- rustup is a version manager that can be used to check for updates with rustup update.
- Russ C is the compiler.
- Cargo is the package manager.

### Creating a Rust File and Compiling It

- Install the Rust RLS extension in VS Code for code completion and linting.
- Create a new file called hello.rs with an entry point function called main().
- Use println!() to print out "Hello World."
- Compile it using rustc hello.rs and run it using ./hello.

### Initializing a Project with Cargo

- Initialize a project in an existing folder using cargo init.
- The project structure includes a Cargo.toml file for application info and dependencies, a .gitignore file, and a
  source folder for all Rust code.
- Use cargo run to compile and run your project. Use cargo build to just build it. Use cargo build --release for
  production optimization.

## Introduction to Print Line Command and Formatting

Section Overview: In this section, the instructor introduces the print line command and formatting in Rust programming
language.

### Creating a Function in Print File

- To create a function in the print file, use `pub` which means public.
- Create a run function for each file to run it.
- Use `printLn!` to print to console.

### Running the Function in Main RS Files

- Use `mod` and then the name of the file above the main function.
- Use `print::function_name` to call the function.

### Basic Formatting

- Use curly braces as placeholders for variables or numbers that need replacement.
- Use double quotes for strings.
- Save and run code using cargo run.

## Basic Formatting with Multiple Placeholders

Section Overview: In this section, we learn how to format multiple placeholders using Rust programming language.

### Using Multiple Placeholders

- Add multiple placeholders by adding more curly braces.
- Replace each placeholder with its corresponding parameter index number.

### Positional Arguments

- Use positional parameters when using variables twice or more times in a string.
- Add index numbers inside curly braces according to their position in parameters list.

### Named Arguments

- Use named parameters instead of positional ones when you have many arguments.
- Assign names before values separated by an equal sign.

## Placeholder Traits

Section Overview: In this section, we learn about placeholder traits available in Rust programming language.

### Binary Trait

- The binary trait is represented by :B
- Used for converting integers into binary format

### Hexadecimal Trait

- The hexadecimal trait is represented by :X
- Used for converting integers into hexadecimal format

### Octal Trait

- The octal trait is represented by :O
- Used for converting integers into octal format

## Debug Traits

Section Overview: In this section, we learn how to use the print line function and tuples to print multiple values. We
also learn how to do basic math.

### Using Print Line Function

- Use `print line` function with a colon and a question mark.
- Put in multiple values using curly braces.
- Example: `print line!("{:?} {} {}", (10, true, "hello"));`

### Basic Math

- Use `print line` function with an expression.
- Example: `print line!("10 + 10 = {}", 10 + 10);`

## Variables

Section Overview: In this section, we learn about variables in Rust. Variables are immutable by default and Rust is a
block-scoped language.

### Creating Variables

- Use the `let` keyword to create variables.
- Example: `let name = "Brad";`
- Immutable by default.

### Mutable Variables

- Add the keyword `mut` to make variables mutable.
- Example: `let mut age = 37;`
- Can reassign value later.

### Constants

- Use the keyword `const` for constants.
- Must explicitly define type.
- Usually all uppercase.
- Example: `const ID:i32 = 001;`

### Assigning Multiple Variables at Once

- Use commas to assign multiple variables at once.
- Example:

```rust
let (x, y, z) = (1, 2, 3);
println!("x = {}, y = {}, z = {}", x, y, z);
```

## Introduction to Rust Programming Language

Section Overview: In this section, the instructor introduces Rust programming language and covers topics such as
assigning variables, data types, and how Rust is a statically typed language.

### Assigning Variables

- Multiple variables can be assigned at once in Rust.
- The `pub function run` is used to run the program.
- Integers come in signed and unsigned forms with different bit sizes. Floats have 32 and 64 bits. Booleans are
  represented by `bool`. Characters are represented by `char`.
- Strings are not primitive types in Rust. Tuples and arrays are also primitive types.

### Data Types

- Vectors are growable arrays while arrays have fixed lengths.
- Rust is a statically typed language which means that it must know the types of all variables at compile time. However,
  the compiler can usually infer what type we want to use based on the value and how we use it.
- Explicit typing can be done using a colon followed by the desired type.
- The maximum size of integers can be found using `STD::i32::MAX` or `STD::i64::MAX`.

### Boolean Expressions

- Booleans can be set explicitly or inferred from expressions.
- A boolean expression evaluates to either true or false depending on whether the condition is met.

## Rust Primitive Types

Section Overview: In this section, the speaker discusses primitive types in Rust, including char and Unicode.

### Char and Unicode

- A char is a single character that can be represented with single quotes.
- Unicode characters can also be used by specifying them with a slash u and curly braces.
- Emojis are also Unicode characters that can be used in Rust.

## Strings in Rust

Section Overview: In this section, the speaker discusses strings in Rust, including primitive strings and string types.

### Creating Strings

- There are two types of strings in Rust: primitive strings (immutable fixed-length) and string types (growable
  heap-allocated data structure).
- To create a primitive string, use single or double quotes.
- To create a string type, use `String::from()` method.

### Modifying Strings

- The `push()` method adds a single character to the end of a string.
- The `push_str()` method adds multiple characters to the end of a string.
- The `len()` method returns the length of a string.
- The `capacity()` method returns the number of bytes that can be stored in a string.
- The `is_empty()` method checks if a string is empty.

## Working with Strings

Section Overview: In this section, the instructor demonstrates how to work with strings in Rust.

### Checking for Substrings and Replacing Them

- Use `contains` method to check if a string contains a substring.
- Use `replace` method to replace a substring with another string.

### Looping Through Strings

- Use a for loop and `split_whitespace` method to loop through a string by whitespace.

### Creating Strings with Capacity

- Use `with_capacity` method to create a string with a certain capacity.
- Use `push` method to add characters to the created string.

### Assertion Testing

- Use assertion testing using the `assert_eq!()` macro to test if something is equal to something else.

## Working with Tuples

Section Overview: In this section, the instructor demonstrates how to work with tuples in Rust.

### Introduction to Tuples

- A tuple is a group of values in Rust.

### Creating and Accessing Tuples

- Create tuples using parentheses and commas between values.
- Access tuple elements using dot notation and index numbers starting from 0.

### Destructuring Tuples

- Destructure tuples into individual variables using pattern matching syntax.

### Tuple Methods

- The `.len()` method returns the number of elements in a tuple.

## Tuples and Arrays in Rust

Section Overview: In this section, the speaker introduces tuples and arrays in Rust programming language. They explain
how to create tuples and access their values using dot syntax. The speaker also demonstrates how to create arrays with
fixed lengths and change their values.

### Creating Tuples

- To create a tuple, use parentheses and separate the values with commas.
- Use dot syntax to access tuple values by index.
- Tuples are immutable by default but can be made mutable using `mut` keyword.

### Creating Arrays

- Arrays have a fixed length that must be specified during creation.
- Use square brackets to define an array's elements.
- Access array elements using zero-based indexing.
- Arrays are stack allocated, meaning they occupy contiguous memory locations.

### Modifying Array Values

- Use `mut` keyword to make an array mutable.
- Change an array value by assigning a new value at its index.

### Debugging Arrays

- Use `println!()` macro with debug trait (`{:?}`) to print entire arrays.
- Get the length of an array using `.len()` method.
- Get the amount of memory occupied by an array using `std::mem::size_of_val(&array)` method.

## Slicing Arrays and Using Vectors

Section Overview: In this section, the instructor demonstrates how to slice arrays and use vectors in Rust.

### Slicing Arrays

- To get a slice from an array, create a mutable variable with the type defined as `&[i32]`.
- Use brackets to specify the range of elements you want to include in the slice.
- The resulting slice can be printed using the debug trait.

### Using Vectors

- Vectors are resizable arrays that can have elements added or removed.
- To define a vector, use `Vec<i32>` instead of `[i32]` for the type definition.
- Use `.push()` to add elements to a vector and `.pop()` to remove them.
- A for loop can be used to iterate through all values in a vector. Use `iter_mut()` to mutate each value individually.

## Mutating Values in Vectors

Section Overview: In this section, the instructor shows how to mutate values in vectors by multiplying each element by

2.

- Use a for loop with `iter_mut()` on a vector.
- Multiply each element by 2 using `*=` operator.
- Print out the entire vector using debug trait.

## Conditionals

Section Overview: This section covers the use of conditionals in Rust.

### If-else statements

- Conditionals are used to connect the condition of something and then act on the result.
- An if-else statement is used to check a condition and execute code based on whether it's true or false.
- Use curly braces for blocks of code that should be executed when a condition is met.
- Use parentheses around conditions, but they are not required.

### Operators

- Rust has several operators, including + and -.
- A boolean variable can be created using `let check_ID: bool = false;`.
- Types can be added to variables if desired.

### Shorthand If

- Rust does not have a ternary operator like many other languages, but shorthand if statements can be used instead.
- Shorthand if statements work similarly to ternary operators in other languages.

## Loops

Section Overview: This section covers loops in Rust.

### Infinite Loop

- An infinite loop is a loop that runs indefinitely until it's interrupted by an external event or action.
- In Rust, an infinite loop can be created using the `loop` keyword followed by curly braces containing the code to
  execute repeatedly.

### While Loop

- A while loop executes as long as its condition remains true.
- The syntax for a while loop includes the `while` keyword followed by parentheses containing the condition and curly
  braces containing the code block to execute.

### For Loop

- A for loop iterates over a range or collection of values.
- The syntax for a for loop includes the `for` keyword followed by parentheses containing an iterator expression and
  curly braces containing the code block to execute.

## Introduction to Loops and Functions

Section Overview: In this section, the instructor introduces loops and functions in Rust programming language.

### While Loop

- A while loop is used to execute a block of code repeatedly as long as a condition is true.
- Use an if statement inside the loop to break out of it when a certain condition is met.
- Example: Print numbers 1 through 20 using a while loop with a break statement.

### FizzBuzz Challenge

- The FizzBuzz challenge requires looping through numbers from 0 to 100 and printing "Fizz" for multiples of 3, "Buzz"
  for multiples of 5, and "FizzBuzz" for multiples of both.
- Use the modulus operator (%) to check if a number is divisible by another number.
- Example: Implementing FizzBuzz challenge using while loop with conditional statements.

### For Range Loop

- A for range loop can be used to iterate over a range of values specified by the user.
- It's similar to the for-each loop in other programming languages.
- Example: Implementing FizzBuzz challenge using for range loop.

### Functions

- Functions are used to store blocks of code that can be reused multiple times throughout your program.
- They take input parameters and return output values (if necessary).
- Example: Creating functions in Rust, including defining function parameters and return types.

## Rust Functions

Section Overview: In this section, we learn about Rust functions and how to use them.

### Creating a Function

- To create a function in Rust, use the `fn` keyword followed by the function name.
- Parameters can be passed into the function using parentheses.
- Use an arrow syntax to specify the return type of the function.
- To call a function, simply write its name followed by any necessary arguments in parentheses.

### Returning from a Function

- To return a value from a Rust function, omit the semicolon at the end of the expression.
- The last expression in a Rust function is automatically returned.

### Closures

- Closures are similar to functions but are more compact and can use outside variables.
- Use `let` to define closures and bind them to variables.
- Closures can take parameters and return values just like functions.

## Pointers and References

Section Overview: In this section, we learn about pointers and references in Rust.

### Primitive Arrays

- Pointers can be used with primitive arrays in Rust.
- Use `let` to create variables that point to other variables or arrays.

### Non-primitive Values

- Non-primitive values require references when assigning another variable to their data.
- Use an ampersand (`&`) before non-primitive values when creating references.

## Rust Basics: References and Structs

Section Overview: In this section, the instructor covers how to create references in Rust using the ampersand symbol and
explains how to create structs in Rust.

### Creating References

- To create a reference in Rust, use the ampersand symbol.
- If you want to point to a non-primitive value, you need to create a reference.
- Use the ampersand symbol before the variable name to create a reference.

### Creating Structs

- Structs are custom data types used in Rust that are similar to classes.
- To create a struct, use the `struct` keyword followed by the name of your struct.
- Define properties or members of your struct using variables with their respective data types.
- Access properties of your struct using dot syntax.

### Tuple Structs

- Tuple structs are another way of creating structs in Rust.
- They do not have named properties like traditional structs but instead rely on their order within the tuple.
- You can access tuple struct values using index numbers instead of property names.

## Creating a Struct in Rust

Section Overview: In this section, the speaker creates a struct called "person" and defines functions to construct and
manipulate it.

### Creating a New Person

- The speaker creates a new struct called "person".
- A function called "new" is defined to create a new person. It takes in two string parameters: first name and last
  name.
- The "new" function returns a person with uppercase first and last names.

### Getting Full Name of Person

- A method called "full name" is created to get the full name of the person.
- The "full name" method returns a string using format! macro.
- The full name method is used to print the full name of the person.

### Changing Last Name of Person

- A function called "set last name" is created to change the last name of the person.
- The set last name function mutates self and takes in one parameter: last (string).
- The set last name function is used to change the last name of Mary Doe to Williams.

### Converting Name to Tuple

- A method called "to tuple" is created to convert the person's first and last names into a tuple.
- The method returns a tuple with two strings: first_name and last_name.

## Structs

Section Overview: In this section, the instructor explains how structs are used in Rust and compares them to classes in
other programming languages.

## Introduction to Structs

- A struct is a data type that groups together variables of different types.
- The syntax for creating a struct is `struct Name { field1: Type1, field2: Type2 }`.
- Structs are similar to classes in other programming languages like Python, PHP, JavaScript, and Java.

## Enums

Section Overview: In this section, the instructor introduces enums and demonstrates how they can be used in Rust.

### Introduction to Enums

- An enum is a type that has a few definite values.
- The syntax for creating an enum is `enum Name { Variant1, Variant2 }`.
- Variants are the possible values of an enum.

### Using Enums

- We can create functions that take enums as parameters.
- We can use match statements to perform actions based on the value of an enum.

## Functions with Enums

Section Overview: In this section, the instructor demonstrates how to create functions that take enums as parameters and
use match statements to perform actions based on their values.

### Creating Functions with Enums

- We can create functions that take enums as parameters by specifying the type of the parameter as the name of the enum.
- We can use match statements to perform actions based on the value of an enum.
- Match statements are similar to switch statements in other programming languages.

## Command Line Arguments

Section Overview: In this section, the instructor explains how command line arguments work in Rust and demonstrates how
to get them using env::args().

### Introduction to Command Line Arguments

- Command line arguments are values passed into a program when it is run from the command line.
- We can use command line arguments to customize the behavior of our program.

### Getting Command Line Arguments

- We can get command line arguments using the env::args() function from the standard library.
- The env::args() function returns a vector of strings containing the command line arguments.

## Introduction to Command Line Applications

Section Overview: In this section, the instructor introduces how to create command line applications in Rust.

### Getting Input from Command Line Arguments

- The `args` variable is used to get input from the command line.
- The first element of the `args` vector is always the target of the executable.
- Additional arguments passed in are added to the `args` vector.
- A specific argument can be accessed by its index in the `args` vector.

### Creating a Command Line Application

- Create a variable called `command` and set it equal to `args[1]`.
- Use an if statement to check what command was entered and execute different code based on that command.
- Use placeholders to insert variables into print statements.
- If an invalid command is entered, print a message indicating so.

## Conclusion

- Rust is a powerful language for creating command line applications.
- There is much more that can be done with Rust beyond what was covered in this video.
- The instructor plans on doing something with Web Assembly soon.

[Generated by Video Highlight](https://videohighlight.com/video/summary/zF34dRivLOw)