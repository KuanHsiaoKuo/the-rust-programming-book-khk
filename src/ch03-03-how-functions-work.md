# Functions

<!--ts-->
* [Functions](#functions)
   * [Parameters](#parameters)
   * [Statements and Expressions](#statements-and-expressions)
      * [Function bodies are made up of statements](#function-bodies-are-made-up-of-statements)
      * [Expression-based language](#expression-based-language)
      * [Differences](#differences)
   * [Functions with Return Values](#functions-with-return-values)
      * [Last return or early return](#last-return-or-early-return)
      * [place a semicolon or not](#place-a-semicolon-or-not)

<!-- Created by https://github.com/ekalinin/github-markdown-toc -->
<!-- Added by: runner, at: Wed Nov 16 15:48:25 UTC 2022 -->

<!--te-->

Functions are prevalent in Rust code. You’ve already seen one of the most
important functions in the language:

- the `main` function, which is the entry point of many programs.
- You’ve also seen the `fn` keyword, which allows you to declare new functions.

Rust code uses *snake case* as the conventional style for function and variable
names, in which all letters are lowercase and underscores separate words.

~~~admonish info title="Here’s a program that contains an example function definition:" collapsible=true
```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-16-functions/src/main.rs}}
```
~~~

- We define a function in Rust by entering `fn` followed by a function name and a
  set of parentheses.
- The curly brackets tell the compiler where the function
  body begins and ends.
- We can call any function we’ve defined by entering its name followed by a set
  of parentheses.
- Because `another_function` is defined in the program, it can be
  called from inside the `main` function.
- Note that we defined `another_function`
  *after* the `main` function in the source code; we could have defined it before
  as well.

> Rust doesn’t care where you define your functions, only that they’re
> defined somewhere in a scope that can be seen by the caller.

Let’s start a new binary project named *functions* to explore functions
further. Place the `another_function` example in *src/main.rs* and run it.

~~~admonish info title="You should see the following output:" collapsible=true
```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-16-functions/output.txt}}
```
~~~

The lines execute in the order in which they appear in the `main` function.
First the “Hello, world!” message prints, and then `another_function` is called
and its message is printed.

## Parameters

We can define functions to have *parameters*, which are special variables that
are part of a function’s signature. When a function has parameters, you can
provide it with concrete values for those parameters.

~~~admonish tip title="Arguments or Parameters" collapsible=true
> Technically, the concrete
> values are called *arguments*, but in casual conversation, people tend to use
> the words *parameter* and *argument* interchangeably for either the variables
> in a function’s definition or the concrete values passed in when you call a
> function.
~~~

~~~admonish info title="In this version of *another_function* we add a parameter:" collapsible=true
```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-17-functions-with-parameters/src/main.rs}}
```
~~~

~~~admonish info title="Try running this program; you should get the following output:" collapsible=true
```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-17-functions-with-parameters/output.txt}}
```
~~~

- The declaration of `another_function` has one parameter named `x`.
- The type of `x` is specified as `i32`. When we pass `5` in to `another_function`
- the `println!` macro puts `5` where the pair of curly brackets containing `x` was
  in the format string.

~~~admonish tip title="In function signatures, you *must* declare the type of each parameter." collapsible=true
This is a deliberate decision in Rust’s design: 

requiring type annotations in function definitions means the compiler almost never needs you to use them elsewhere in
the code to figure out what type you mean. 

The compiler is also able to give
more helpful error messages if it knows what types the function expects.
~~~

~~~admonish info title="When defining multiple parameters, separate the parameter declarations with commas, like this:" collapsible=true
```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-18-functions-with-multiple-parameters/src/main.rs}}
```
~~~

This example creates a function named `print_labeled_measurement` with two parameters:

1. The first parameter is named `value` and is an `i32`.
2. The second is
   named `unit_label` and is type `char`.
3. The function then prints text containing
   both the `value` and the `unit_label`.

Let’s try running this code.

~~~admonish info title="Replace the program currently in your *functions* project’s *src/main.rs* file with the preceding example and run it using *cargo run*: " collapsible=true

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-18-functions-with-multiple-parameters/output.txt}}
```
~~~

Because we called the function with `5` as the value for `value` and `'h'` as
the value for `unit_label`, the program output contains those values.

## Statements and Expressions

### Function bodies are made up of statements

Function bodies are made up of **a series of statements**, optionally ending in an
expression.

> So far, the functions we’ve covered haven’t included an ending
> expression, but you have seen an expression as part of a statement.

### Expression-based language

~~~admonish tip title="Rust is an expression-based language" collapsible=true
> Because
> Rust is an expression-based language, this is an important distinction to
> understand.
~~~

Other languages don’t have the same distinctions, so let’s look at
what statements and expressions are and how their differences affect the bodies
of functions.

### Differences

~~~admonish tip title="Diffreence between statements and expressions" collapsible=true
- **Statements** are instructions that perform some action and do not return a value, end with a semicolon
- **Expressions** evaluate to a resultant value. Let’s look at some examples.
~~~

We’ve actually already used statements and expressions:

1. Creating a variable and assigning a value to it with the `let` keyword is a statement.

In Listing 3-1, `let y = 6;` is a statement.

~~~admonish info title="Listing 3-1: A *main* function declaration containing one statement" collapsible=true
```rust
{{#rustdoc_include../listings/ch03-common-programming-concepts/listing-03-01/src/main.rs}}
```
~~~

2. Function definitions are also statements;
3. the entire preceding example is a statement in itself.

4. Statements do not return values.

Therefore, you can’t assign a `let` statement to another variable, as the following code tries to do; you’ll get an error:

~~~admonish info title="Therefore, you can’t assign a *let* statement to another variable, as the following code tries to do; you’ll get an error:" collapsible=true
```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-19-statements-vs-expressions/src/main.rs}}
```
~~~

~~~admonish info title="When you run this program, the error you’ll get looks like this:" collapsible=true
```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-19-statements-vs-expressions/output.txt}}
```
~~~

The `let y = 6` statement does not return a value, so there isn’t anything for `x` to bind to.

> This is different from what happens in other languages, such as
> C and Ruby, where the assignment returns the value of the assignment. In those
> languages, you can write `x = y = 6` and have both `x` and `y` have the value
`6`; that is not the case in Rust.

5. Expressions evaluate to a value and make up most of the rest of the code that
   you’ll write in Rust.

Consider a math operation, such as `5 + 6`, which is an
expression that evaluates to the value `11`.

6. Expressions can be part of statements:

in Listing 3-1, the `6` in the statement `let y = 6;` is an expression that evaluates to the value `6`.

7. Calling a function, a macro, a scope, all are expressions.

~~~admonish info title=" A new scope block created with curly brackets is an expression, for example:" collapsible=true
```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-20-blocks-are-expressions/src/main.rs}}
```
~~~

This expression:

```rust,ignore
{
    let x = 3;
    x + 1
}
```

is a block that, in this case, evaluates to `4`. That value gets bound to `y`
as part of the `let` statement.

> Note that the `x + 1` line doesn’t have a
> semicolon at the end, which is unlike most of the lines you’ve seen so far.

8. Expressions do not include ending semicolons.

> If you add a semicolon to the end
> of an expression, you turn it into a statement, and it will then not return a
> value.

Keep this in mind as you explore function return values and expressions next.

## Functions with Return Values

### Last return or early return

1. Functions can return values to the code that calls them.

2. We don’t name return values, but we must declare their type after an arrow (`->`).

3. Last Return:

In Rust, the return value of the function is synonymous with the value of *the final
expression* in the block of the body of a function.

4. Early Return:

You can return early from a
function by using the `return` keyword and specifying a value, but most
functions return the last expression implicitly.

~~~admonish info title="Here’s an example of a function that returns a value:" collapsible=true
```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-21-function-return-values/src/main.rs}}
```
~~~

There are no function calls, macros, or even `let` statements in the `five`
function—just the number `5` by itself.

That’s a perfectly valid function in Rust.

Note that the function’s return type is specified too, as `-> i32`.

~~~admonish info title="Try running this code; the output should look like this:" collapsible=true
```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-21-function-return-values/output.txt}}
```
~~~

The `5` in `five` is the function’s return value, which is why the return type is `i32`.

### place a semicolon or not

Let’s examine this in more detail.

> There are two important bits:

1. first, the line `let x = five();` shows that we’re using the return value of a
   function to initialize a variable.

Because the function `five` returns a `5`, that line is the same as the following:

```rust
let x = 5;
```

2. Second, the `five` function has no parameters and defines the type of the
   return value, but the body of the function is a lonely `5` with no semicolon
   because it’s an expression whose value we want to return.

~~~admonish info title="Let’s look at another example:" collapsible=true
```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-22-function-parameter-and-return/src/main.rs}}
```
~~~

Running this code will print `The value of x is: 6`.

~~~admonish info title="But if we place a semicolon at the end of the line containing *x + 1*, changing it from an expression to a statement, we’ll get an error:" collapsible=true

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-23-statements-dont-return-values/src/main.rs}}
```

Compiling this code produces an error, as follows:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-23-statements-dont-return-values/output.txt}}
```

~~~

> The main error message, `mismatched types`, reveals the core issue with this code：

1. The definition of the function `plus_one` says that it will return an
   `i32`, but statements don’t evaluate to a value, which is expressed by `()`,
   the unit type.

2. Therefore, nothing is returned, which contradicts the function
   definition and results in an error.

3. In this output, Rust provides a message to
   possibly help rectify this issue: it suggests removing the semicolon, which
   would fix the error.
