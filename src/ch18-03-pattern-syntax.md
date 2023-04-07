# Pattern Syntax

```admonish abstract title="Summarize made by chatGPT" collapsible=true
1. Patterns are used in Rust for destructuring complex types, matching against specific values, and binding variables.
2. Rust's pattern syntax includes various types of patterns such as literals, variables, wildcards, ranges, references,
   structs, enums, and more.
3. Pattern matching can be used in various contexts such as function arguments, match expressions, let statements, and
   more.
4. Pattern matching can also be used with guards to specify additional conditions for a match arm.
5. Rust also supports destructuring nested structures using patterns like tuples, structs, and enums with multiple
   fields.
6. Patterns can be combined using the | operator to match multiple patterns, and the .. operator to ignore some parts of
   a pattern.
7. Rust provides the ability to define custom patterns using the @ operator.
8. The _ wildcard pattern can be used as a catch-all to match any value.
9. The ref and ref mut patterns can be used to create references to the matched value.
10. The box pattern can be used to match against a boxed value.
11. The @ operator can also be used to bind a variable to a pattern without destructuring it.
12. Rust provides various shorthand notations for patterns such as .. for matching the rest of a slice or array, and ..=
    for inclusive ranges.
13. Rust also supports pattern matching with if let and while let expressions, which are useful for handling optional
    values.
14. Finally, the webpage provides examples and code snippets to illustrate the use of pattern matching in Rust.
```

```admonish question title="Questions made by chatGPT" collapsible=true
1. What are some of the different types of patterns that Rust's pattern syntax includes?
2. What are some of the contexts in which pattern matching can be used in Rust?
3. How can you combine patterns using Rust's pattern syntax?
4. What are some of the shorthand notations for patterns that Rust provides?
5. What are some examples of when you might use pattern matching with if let or while let expressions in Rust?
6. How does Rust's pattern matching syntax differ from that of other programming languages you are familiar with?
7. What are some of the benefits of using pattern matching in Rust?
8. Can you think of any potential drawbacks or challenges associated with using pattern matching in Rust?
9. How might you go about practicing and experimenting with pattern matching in Rust?
10. Can you provide an example of a real-world problem that you might solve using pattern matching in Rust?
```

```admonish tip title="Answers made by chatGPT" collapsible=true
1. Rust's pattern syntax includes various types of patterns, such as literals, variables, wildcards, ranges, references, structs, enums, and more. These patterns can be used to match against specific values, bind variables, and destructure complex types.
2. Pattern matching can be used in various contexts in Rust, such as function arguments, match expressions, let statements, and more. It is a powerful tool for handling complex data types and making decisions based on their values.
3. Rust's pattern syntax allows you to combine patterns using the | operator to match multiple patterns, and the .. operator to ignore some parts of a pattern. This can make it easier to write concise and expressive code that handles a wide range of values.
4. Rust provides various shorthand notations for patterns, such as .. for matching the rest of a slice or array, and ..= for inclusive ranges. These notations can help simplify pattern matching code and make it more readable.
5. if let and while let expressions in Rust are useful for handling optional values. For example, you might use them to check whether a variable contains a value, and then do something with that value if it does. This can help prevent runtime errors caused by trying to access None values.
6. Rust's pattern matching syntax is similar to that of other programming languages like Scala and Haskell, but it has some unique features such as the ability to define custom patterns using the @ operator.
7. Some benefits of using pattern matching in Rust include its ability to make code more concise and readable, its ability to handle complex data types easily, and its ability to prevent runtime errors by ensuring that all possible cases are handled.
8. Some potential challenges associated with using pattern matching in Rust include the need to carefully consider all possible cases and the potential for complex code that is difficult to maintain.
9. To practice and experiment with pattern matching in Rust, you might start by writing simple programs that use basic patterns like literals and variables, and then gradually work your way up to more complex patterns like structs and enums. The Rust documentation and other online resources can also be helpful for learning more about pattern matching in Rust.
10. A real-world problem that you might solve using pattern matching in Rust could be parsing a file format that contains structured data. By using patterns to match against different parts of the file format, you could easily extract the data you need and transform it into a more useful format
```

<!--ts-->
* [Pattern Syntax](#pattern-syntax)
* [A. Matching Situations](#a-matching-situations)
   * [Matching Literals](#matching-literals)
   * [Matching Named Variables](#matching-named-variables)
   * [Extra Conditionals with Match Guards](#extra-conditionals-with-match-guards)
   * [Multiple Patterns](#multiple-patterns)
   * [Matching Ranges of Values with ..=](#matching-ranges-of-values-with-)
* [B. Destructuring to Break Apart Values](#b-destructuring-to-break-apart-values)
   * [Destructuring Structs](#destructuring-structs)
   * [Destructuring Enums](#destructuring-enums)
   * [Destructuring <strong>Nested</strong> Structs and Enums](#destructuring-nested-structs-and-enums)
   * [Destructuring Structs and Tuples](#destructuring-structs-and-tuples)
* [C. Ignoring Values in a Pattern](#c-ignoring-values-in-a-pattern)
   * [1. Ignoring an Entire Value with _](#1-ignoring-an-entire-value-with-_)
   * [2. Ignoring Parts of a Value with a Nested _](#2-ignoring-parts-of-a-value-with-a-nested-_)
   * [3. Ignoring an Unused Variable by Starting Its Name with _](#3-ignoring-an-unused-variable-by-starting-its-name-with-_)
   * [4. Ignoring Remaining Parts of a Value with ..](#4-ignoring-remaining-parts-of-a-value-with-)
* [Summary](#summary)

<!-- Created by https://github.com/ekalinin/github-markdown-toc -->
<!-- Added by: runner, at: Fri Apr  7 14:49:51 UTC 2023 -->

<!--te-->

In this section, we gather all the syntax valid in patterns and discuss why and
when you might want to use each one.

# A. Matching Situations

## Matching Literals

As you saw in Chapter 6, you can match patterns against literals directly.

~~~admonish info title=" The following code gives some examples:" collapsible=true
```rust
{{#rustdoc_include ../listings/ch18-patterns-and-matching/no-listing-01-literals/src/main.rs}}
```
~~~

This code prints `one` because the value in `x` is 1.

This syntax is useful
when you want your code to take an action if it gets a particular concrete
value.

## Matching Named Variables

Named variables are irrefutable patterns that match any value, and we’ve used
them many times in the book.

However, there is a complication when you use
named variables in `match` expressions:

- Because `match` starts a new scope,
  variables declared as part of a pattern inside the `match` expression will
  shadow those with the same name outside the `match` construct, as is the case
  with all variables.
- In Listing 18-11, we declare a variable named `x` with the
  value `Some(5)` and a variable `y` with the value `10`.
- We then create a `match` expression on the value `x`.
- Look at the patterns in the match arms and
  `println!` at the end, and try to figure out what the code will print before
  running this code or reading further.

~~~admonish info title="Listing 18-11: A *match* expression with an arm that introduces a shadowed variable *y*" collapsible=true
```rust
{{#rustdoc_include ../listings/ch18-patterns-and-matching/listing-18-11/src/main.rs:here}}
```
~~~

Let’s walk through what happens when the `match` expression runs:

1. The pattern in the first match arm doesn’t match the defined value of `x`, so the code continues.
2. The pattern in the second match arm introduces a new variable named `y` that will match any value inside a `Some`
   value.

- Because we’re in a new scope inside the `match` expression, this is a new `y` variable, not the `y` we declared at
  the beginning with the value 10.
- This new `y` binding will match any value inside a `Some`, which is what we have in `x`.
- Therefore, this new `y` binds to the inner value of the `Some` in `x`.
- That value is `5`, so the expression for that arm executes and prints `Matched, y = 5`.

3. If `x` had been a `None` value instead of `Some(5)`, the patterns in the first
   two arms wouldn’t have matched, so the value would have matched to the
   underscore.

- We didn’t introduce the `x` variable in the pattern of the
  underscore arm, so the `x` in the expression is still the outer `x` that hasn’t
  been shadowed.

- In this hypothetical case, the `match` would print `Default
  case, x = None`.

4. When the `match` expression is done, its scope ends, and so does the scope of
   the inner `y`. The last `println!` produces `at the end: x = Some(5), y = 10`.

> To create a `match` expression that compares the values of the outer `x` and `y`, rather than introducing a shadowed
> variable, we would need to use a **match guard conditional** instead.

We’ll talk about match guards later in the [“Extra
Conditionals with Match Guards”](#extra-conditionals-with-match-guards)<!--
ignore --> section.

## Extra Conditionals with Match Guards

A *match guard* is an additional `if` condition, specified after the pattern in
a `match` arm, that must also match for that arm to be chosen.

> Match guards are useful for expressing more complex ideas than a pattern alone allows.

The condition can use variables created in the pattern.

Listing 18-26 shows a `match` where the first arm has the pattern `Some(x)` and also has a match
guard of `if x % 2 == 0` (which will be true if the number is even).

~~~admonish info title="Listing 18-26: Adding a match guard to a pattern" collapsible=true
```rust
{{#rustdoc_include ../listings/ch18-patterns-and-matching/listing-18-26/src/main.rs:here}}
```
~~~

This example will print `The number 4 is even`.

- When `num` is compared to the pattern in the first arm, it matches, because `Some(4)` matches `Some(x)`.
- Then the match guard checks whether the remainder of dividing `x` by 2 is equal to
  0, and because it is, the first arm is selected.

- If `num` had been `Some(5)` instead, the match guard in the first arm would
  have been false because the remainder of 5 divided by 2 is 1, which is not
  equal to 0.
- Rust would then go to the second arm, which would match because the
  second arm doesn’t have a match guard and therefore matches any `Some` variant.

There is no way to express the `if x % 2 == 0` condition within a pattern, so
the match guard gives us the ability to express this logic.

> The downside of this additional expressiveness is that the compiler doesn't try to check for
> exhaustiveness when match guard expressions are involved.

In Listing 18-11, we mentioned that we could use match guards to solve our
**pattern-shadowing problem**.

Recall that we created a new variable inside the
pattern in the `match` expression instead of using the variable outside the
`match`.

That new variable meant we couldn’t test against the value of the
outer variable.

Listing 18-27 shows how we can use a match guard to fix this problem.

~~~admonish info title="Listing 18-27: Using a match guard to test for equality with an outer variable" collapsible=true
```rust
{{#rustdoc_include ../listings/ch18-patterns-and-matching/listing-18-27/src/main.rs}}
```
~~~

This code will now print `Default case, x = Some(5)`.

The pattern in the second
match arm doesn’t introduce a new variable `y` that would shadow the outer `y`,
meaning we can use the outer `y` in the match guard.

Instead of specifying the pattern as `Some(y)`, which would have shadowed the outer `y`, we specify
`Some(n)`.

This creates a new variable `n` that doesn’t shadow anything because
there is no `n` variable outside the `match`.

The match guard `if n == y` is not a pattern and therefore doesn’t introduce
new variables.

This `y` *is* the outer `y` rather than a new shadowed `y`, and
we can look for a value that has the same value as the outer `y` by comparing
`n` to `y`.

You can also use the *or* operator `|` in a match guard to specify multiple
patterns;

> the match guard condition will apply to all the patterns.

Listing
18-28 shows the precedence when combining a pattern that uses `|` with a match
guard.

The important part of this example is that the `if y` match guard
applies to `4`, `5`, *and* `6`, even though it might look like `if y` only
applies to `6`.

~~~admonish info title="Listing 18-28: Combining multiple patterns with a match guard" collapsible=true
```rust
{{#rustdoc_include ../listings/ch18-patterns-and-matching/listing-18-28/src/main.rs:here}}
```
~~~

1. The match condition states that the arm only matches if the value of `x` is
   equal to `4`, `5`, or `6` *and* if `y` is `true`. When this code runs, the
   pattern of the first arm matches because `x` is `4`, but the match guard `if y`
   is false, so the first arm is not chosen.

2. The code moves on to the second arm,
   which does match, and this program prints `no`. The reason is that the `if`
   condition applies to the whole pattern `4 | 5 | 6`, not only to the last value
   `6`.

3. In other words, the precedence of a match guard in relation to a pattern
   behaves like this:

```text
(4 | 5 | 6) if y => ...
```

rather than this:

```text
4 | 5 | (6 if y) => ...
```

After running the code, the precedence behavior is evident:

- if the match guard
  were applied only to the final value in the list of values specified using the
  `|` operator, the arm would have matched and the program would have printed
  `yes`.

## Multiple Patterns

In `match` expressions, you can match multiple patterns using the `|` syntax,
which is the pattern *or* operator.

For example, in the following code we match
the value of `x` against the match arms, the first of which has an *or* option,
meaning if the value of `x` matches either of the values in that arm, that
arm’s code will run:

~~~admonish info title="For example, in the following code we match the value of *x* against the match arms, the first of which has an *or* option, meaning if the value of *x* matches either of the values in that arm, that arm’s code will run:" collapsible=true
```rust
{{#rustdoc_include ../listings/ch18-patterns-and-matching/no-listing-02-multiple-patterns/src/main.rs:here}}
```
~~~

This code prints `one or two`.

## Matching Ranges of Values with `..=`

The `..=` syntax allows us to match to an inclusive range of values.

~~~admonish info title="In the following code, when a pattern matches any of the values within the given range, that arm will execute:" collapsible=true
```rust
{{#rustdoc_include ../listings/ch18-patterns-and-matching/no-listing-03-ranges/src/main.rs:here}}
```
~~~

If `x` is 1, 2, 3, 4, or 5, the first arm will match.

This syntax is more
convenient for multiple match values than using the `|` operator to express the
same idea;

if we were to use `|` we would have to specify `1 | 2 | 3 | 4 | 5`.

Specifying a range is much shorter, especially if we want to match, say, any
number between 1 and 1,000!

The compiler checks that the range isn’t empty at compile time, and because the
only types for which Rust can tell if a range is empty or not are `char` and
numeric values, ranges are only allowed with numeric or `char` values.

~~~admonish info title="Here is an example using ranges of *char* values:" collapsible=true
```rust
{{#rustdoc_include ../listings/ch18-patterns-and-matching/no-listing-04-ranges-of-char/src/main.rs:here}}
```
~~~

Rust can tell that `'c'` is within the first pattern’s range and prints `early
ASCII letter`.

## `@` Bindings: test a value and save it in a variable within one pattern

The *at* operator `@` lets us create a variable that holds a value at the same
time as we’re testing that value for a pattern match.

In Listing 18-29, we want
to test that a `Message::Hello` `id` field is within the range `3..=7`.

We also
want to bind the value to the variable `id_variable` so we can use it in the
code associated with the arm.

We could name this variable `id`, the same as the
field, but for this example we’ll use a different name.

~~~admonish info title="Listing 18-29: Using *@* to bind to a value in a pattern while also testing it" collapsible=true
```rust
{{#rustdoc_include ../listings/ch18-patterns-and-matching/listing-18-29/src/main.rs:here}}
```
~~~

This example will print `Found an id in range: 5`.

1. By specifying `id_variable @` before the range `3..=7`, we’re capturing whatever value matched the range
   while also testing that the value matched the range pattern.

2. In the second arm, where we only have a range specified in the pattern, the code
   associated with the arm doesn’t have a variable that contains the actual value
   of the `id` field.

- The `id` field’s value could have been 10, 11, or 12, but the code that goes with that pattern doesn’t know which it
  is.
- The pattern code isn’t able to use the value from the `id` field, because we haven’t saved the `id` value in a
  variable.

3. In the last arm, where we’ve specified a variable without a range, we do have
   the value available to use in the arm’s code in a variable named `id`.

- The reason is that we’ve used the struct field shorthand syntax.
- But we haven’t applied any test to the value in the `id` field in this arm, as we did with the
  first two arms: any value would match this pattern.

> Using `@` lets us test a value and save it in a variable within one pattern.

# B. Destructuring to Break Apart Values

We can also use patterns to destructure structs, enums, and tuples to use
different parts of these values. Let’s walk through each value.

## Destructuring Structs

Listing 18-12 shows a `Point` struct with two fields, `x` and `y`, that we can
break apart using a pattern with a `let` statement.

~~~admonish info title=" The following code gives some examples:" collapsible=true
```rust
{{#rustdoc_include ../listings/ch18-patterns-and-matching/listing-18-12/src/main.rs}}
```
~~~

This code creates the variables `a` and `b` that match the values of the `x`
and `y` fields of the `p` struct.

This example shows that the names of the
variables in the pattern don’t have to match the field names of the struct.

> However, it’s common to match the variable names to the field names to make it
> easier to remember which variables came from which fields.

Because of this common usage, and because writing `let Point { x: x, y: y } = p;` contains a
lot of duplication, Rust has a shorthand for patterns that match struct fields:

> you only need to list the name of the struct field, and the variables created
> from the pattern will have the same names.

Listing 18-13 behaves in the same
way as the code in Listing 18-12, but the variables created in the `let`
pattern are `x` and `y` instead of `a` and `b`.

~~~admonish info title="Listing 18-13: Destructuring struct fields using struct field shorthand" collapsible=true
```rust
{{#rustdoc_include ../listings/ch18-patterns-and-matching/listing-18-13/src/main.rs}}
```
~~~

This code creates the variables `x` and `y` that match the `x` and `y` fields
of the `p` variable.

The outcome is that the variables `x` and `y` contain the values from the `p` struct.

We can also destructure with literal values as part of the struct pattern
rather than creating variables for all the fields.

Doing so allows us to test
some of the fields for particular values while creating variables to
destructure the other fields.

In Listing 18-14, we have a `match` expression that separates `Point` values
into three cases: points that lie directly on the `x` axis (which is true when
`y = 0`), on the `y` axis (`x = 0`), or neither.

~~~admonish info title="Listing 18-14: Destructuring and matching literal values in one pattern" collapsible=true
```rust
{{#rustdoc_include ../listings/ch18-patterns-and-matching/listing-18-14/src/main.rs:here}}
```
~~~

1. The first arm will match any point that lies on the `x` axis by specifying that
   the `y` field matches if its value matches the literal `0`.

The pattern still
creates an `x` variable that we can use in the code for this arm.

2. Similarly, the second arm matches any point on the `y` axis by specifying that
   the `x` field matches if its value is `0` and creates a variable `y` for the
   value of the `y` field.
3. The third arm doesn’t specify any literals, so it
   matches any other `Point` and creates variables for both the `x` and `y` fields.

In this example, the value `p` matches the second arm by virtue of `x`
containing a 0, so this code will print `On the y axis at 7`.

> Remember that a `match` expression stops checking arms once it has found the
> first matching pattern

so even though `Point { x: 0, y: 0}` is on the `x` axis
and the `y` axis, this code would only print `On the x axis at 0`.

## Destructuring Enums

We've destructured enums in this book (for example, Listing 6-5 in Chapter 6),
but haven’t yet explicitly discussed that the pattern to destructure an enum
corresponds to the way the data stored within the enum is defined.

As an example, in Listing 18-15 we use the `Message` enum from Listing 6-2 and write
a `match` with patterns that will destructure each inner value.

~~~admonish info title="Listing 18-15: Destructuring enum variants that hold different kinds of values" collapsible=true
```rust
{{#rustdoc_include ../listings/ch18-patterns-and-matching/listing-18-15/src/main.rs}}
```
~~~

This code will print `Change the color to red 0, green 160, and blue 255`.

Try changing the value of `msg` to see the code from the other arms run.

For enum variants without any data, like `Message::Quit`, we can’t destructure
the value any further.

We can only match on the literal `Message::Quit` value, and no variables are in that pattern.

For struct-like enum variants, such as `Message::Move`, we can use a pattern
similar to the pattern we specify to match structs.

After the variant name, we place curly brackets and then list the fields with variables so we break apart
the pieces to use in the code for this arm.

Here we use the shorthand form as we did in Listing 18-13.

For tuple-like enum variants, like `Message::Write` that holds a tuple with one
element and `Message::ChangeColor` that holds a tuple with three elements, the
pattern is similar to the pattern we specify to match tuples.

The number of variables in the pattern must match the number of elements in the variant we’re matching.

## Destructuring **Nested** Structs and Enums

> So far, our examples have all been matching structs or enums one level deep,
> but matching can work on nested items too!

For example, we can refactor the code in Listing 18-15 to support RGB and HSV colors in the `ChangeColor`
message, as shown in Listing 18-16.

~~~admonish info title="Listing 18-16: Matching on nested enums" collapsible=true
```rust
{{#rustdoc_include ../listings/ch18-patterns-and-matching/listing-18-16/src/main.rs}}
```
~~~

- The pattern of the first arm in the `match` expression matches a
  `Message::ChangeColor` enum variant that contains a `Color::Rgb` variant;
- then the pattern binds to the three inner `i32` values.
- The pattern of the second arm also matches a `Message::ChangeColor` enum variant, but the inner enum
  matches `Color::Hsv` instead.
- We can specify these complex conditions in one `match` expression, even though two enums are involved.

## Destructuring Structs and Tuples

> We can mix, match, and nest destructuring patterns in even more complex ways.

~~~admonish info title="The following example shows a complicated destructure where we nest structs and tuples inside a tuple and destructure all the primitive values out:" collapsible=true
```rust
{{#rustdoc_include ../listings/ch18-patterns-and-matching/no-listing-05-destructuring-structs-and-tuples/src/main.rs:here}}
```
~~~

This code lets us break complex types into their component parts so we can use
the values we’re interested in separately.

Destructuring with patterns is a convenient way to use pieces of values, such
as the value from each field in a struct, separately from each other.

# C. Ignoring Values in a Pattern

You’ve seen that it’s sometimes useful to ignore values in a pattern, such as
in the last arm of a `match`, to get a catchall that doesn’t actually do
anything but does account for all remaining possible values.

> There are a few ways to ignore entire values or parts of values in a pattern:

- using the `_` pattern (which you’ve seen)
- using the `_` pattern within another pattern,
- using a name that starts with an underscore,
- or using `..` to ignore remaining parts of a value.

Let’s explore how and why to use each of these patterns.

## 1. Ignoring an Entire Value with `_`

We’ve used the underscore as a wildcard pattern that will match any value but
not bind to the value.

This is especially useful as the last arm in a `match` expression, but we can also use it in any pattern, including
function
parameters, as shown in Listing 18-17.

~~~admonish info title="Listing 18-17: Using *_* in a function signature" collapsible=true
```rust
{{#rustdoc_include ../listings/ch18-patterns-and-matching/listing-18-17/src/main.rs}}
```
~~~

This code will completely ignore the value `3` passed as the first argument,
and will print `This code only uses the y parameter: 4`.

In most cases when you no longer need a particular function parameter, you
would change the signature so it doesn’t include the unused parameter.

> Ignoring a function parameter can be especially useful in cases when, for example,
> you're implementing a trait when you need a certain type signature but the
> function body in your implementation doesn’t need one of the parameters.

You then avoid getting a compiler warning about unused function parameters, as you
would if you used a name instead.

## 2. Ignoring Parts of a Value with a Nested `_`

We can also use `_` **inside another pattern** to ignore just part of a value, for
example, when we want to test for only part of a value but have no use for the
other parts in the corresponding code we want to run.

Listing 18-18 shows code responsible for managing a setting’s value.

The business requirements are that
the user should not be allowed to overwrite an existing customization of a
setting but can unset the setting and give it a value if it is currently unset.

~~~admonish info title="Listing 18-18: Using an underscore within patterns that match *Some* variants when we don’t need to use the value inside the *Some*" collapsible=true
```rust
{{#rustdoc_include ../listings/ch18-patterns-and-matching/listing-18-18/src/main.rs:here}}
```
~~~

This code will print `Can't overwrite an existing customized value` and then
`setting is Some(5)`.

- In the first match arm, we don’t need to match on or use
  the values inside either `Some` variant, but we do need to test for the case
  when `setting_value` and `new_setting_value` are the `Some` variant.

In that
case, we print the reason for not changing `setting_value`, and it doesn’t get
changed.

- In all other cases (if either `setting_value` or `new_setting_value` are
  `None`) expressed by the `_` pattern in the second arm, we want to allow
  `new_setting_value` to become `setting_value`.

> We can also use underscores in multiple places within one pattern to ignore
> particular values.

Listing 18-19 shows an example of ignoring the second and fourth values in a tuple of five items.

~~~admonish info title="Listing 18-19: Ignoring multiple parts of a tuple" collapsible=true
```rust
{{#rustdoc_include ../listings/ch18-patterns-and-matching/listing-18-19/src/main.rs:here}}
```
~~~

This code will print `Some numbers: 2, 8, 32`, and the values 4 and 16 will be ignored.

## 3. Ignoring an Unused Variable by Starting Its Name with `_`

If you create a variable but don’t use it anywhere, Rust will usually issue a
warning because an unused variable could be a bug.

> However, sometimes it’s
> useful to be able to create a variable you won’t use yet, such as when you’re
> prototyping or just starting a project.

In this situation, you can tell Rust not to warn you about the unused variable by starting the name of the variable
with an underscore.

In Listing 18-20, we create two unused variables, but when
we compile this code, we should only get a warning about one of them.

~~~admonish info title="Listing 18-20: Starting a variable name with an underscore to avoid getting unused variable warnings" collapsible=true
```rust
{{#rustdoc_include ../listings/ch18-patterns-and-matching/listing-18-20/src/main.rs}}
```
~~~

Here we get a warning about not using the variable `y`, but we don’t get a
warning about not using `_x`.

> Note that there is a subtle difference between using only `_` and using a name
> that starts with an underscore:

- The syntax `_x` still binds the value to the
  variable, whereas `_` doesn’t bind at all.

- To show a case where this
  distinction matters, Listing 18-21 will provide us with an error.

~~~admonish info title="Listing 18-21: An unused variable starting with an underscore still binds the value, which might take ownership of the value" collapsible=true
```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch18-patterns-and-matching/listing-18-21/src/main.rs:here}}
```
~~~

We’ll receive an error because the `s` value will still be moved into `_s`,
which prevents us from using `s` again. However, using the underscore by itself
doesn’t ever bind to the value.

Listing 18-22 will compile without any errors because `s` doesn’t get moved into `_`.

~~~admonish info title="Listing 18-22: Using an underscore does not bind the value" collapsible=true
```rust
{{#rustdoc_include ../listings/ch18-patterns-and-matching/listing-18-22/src/main.rs:here}}
```
~~~

This code works just fine because we never bind `s` to anything; it isn’t moved.

## 4. Ignoring Remaining Parts of a Value with `..`

With values that have many parts, we can use the `..` syntax to use specific
parts and ignore the rest, avoiding the need to list underscores for each
ignored value.

The `..` pattern ignores any parts of a value that we haven’t
explicitly matched in the rest of the pattern.

In Listing 18-23, we have a
`Point` struct that holds a coordinate in three-dimensional space.

In the
`match` expression, we want to operate only on the `x` coordinate and ignore
the values in the `y` and `z` fields.

~~~admonish info title="Listing 18-23: Ignoring all fields of a *Point* except for *x* by using *.." collapsible=true
```rust
{{#rustdoc_include ../listings/ch18-patterns-and-matching/listing-18-23/src/main.rs:here}}
```
~~~

We list the `x` value and then just include the `..` pattern.

> This is quicker
> than having to list `y: _` and `z: _`, particularly when we’re working with
> structs that have lots of fields in situations where only one or two fields are
> relevant.

The syntax `..` will expand to as many values as it needs to be.

Listing 18-24 shows how to use `..` with a tuple.

~~~admonish info title="Listing 18-24: Matching only the first and last values in a tuple and ignoring all other values" collapsible=true
```rust
{{#rustdoc_include ../listings/ch18-patterns-and-matching/listing-18-24/src/main.rs}}
```
~~~

In this code, the first and last value are matched with `first` and `last`.

The `..` will match and ignore everything in the middle.

> However, using `..` must be unambiguous.

If it is unclear which values are
intended for matching and which should be ignored, Rust will give us an error.

Listing 18-25 shows an example of using `..` ambiguously, so it will not
compile.

~~~admonish info title="Listing 18-25: An attempt to use *..* in an ambiguous way" collapsible=true
```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch18-patterns-and-matching/listing-18-25/src/main.rs}}
```
~~~

~~~admonish info title="When we compile this example, we get this error:" collapsible=true
```console
{{#include ../listings/ch18-patterns-and-matching/listing-18-25/output.txt}}
```
~~~

It’s impossible for Rust to determine how many values in the tuple to ignore
before matching a value with `second` and then how many further values to
ignore thereafter.

This code could mean that we want to ignore `2`, bind `second` to `4`, and then ignore `8`, `16`, and `32`;

or that we want to ignore `2` and `4`, bind `second` to `8`, and then ignore `16` and `32`;

and so forth.

The variable name `second` doesn’t mean anything special to Rust, so we get a
compiler error because using `..` in two places like this is ambiguous.

# Summary

Rust’s patterns are very useful in distinguishing between different kinds of
data:

- When used in `match` expressions, Rust ensures your patterns cover every
  possible value, or your program won’t compile.
- Patterns in `let` statements and
  function parameters make those constructs more useful, enabling the
  destructuring of values into smaller parts at the same time as assigning to
  variables.
- We can create simple or complex patterns to suit our needs.

Next, for the penultimate chapter of the book, we’ll look at some advanced
aspects of a variety of Rust’s features.
