# Happy Little Fractals Coding the Mandelbrot Set in Rust

<!--ts-->


<!-- Created by https://github.com/ekalinin/github-markdown-toc -->
<!-- Added by: runner, at: Thu Apr 20 14:04:31 UTC 2023 -->

<!--te-->

## [#](t=0:00:00) Introduction

Section Overview: In this section, the speaker introduces the topic of the tutorial and explains that they will be
demonstrating how to program in Rust to create a visualization of the Mandelbrot set.

- The speaker acknowledges that they may struggle with creating an interesting visualization but promises to take things
  slow for beginners.
- The Mandelbrot set is introduced as a simple yet fascinating mathematical concept that can produce beautiful
  visualizations.
- The goal of the tutorial is to provide viewers with the skills necessary to create their own visualizations and
  explore the patterns within the Mandelbrot set.

## [#](t=0:01:11) Creating a Basic Visualization

Section Overview: In this section, the speaker demonstrates how to create a basic visualization of the Mandelbrot set
using Rust programming language.

- The speaker shows an example image of what they aim to recreate using Rust programming language.
- By altering parameters, such as zooming in or out, users can change what they see on their screen.
- The speaker demonstrates how changing parameters alters output by increasing all values by 10.
- To increase resolution, more pixels are added.

## [#](t=0:03:13) Shifting Viewport

Section Overview: In this section, the speaker explains how shifting viewport changes what is visible on screen when
rendering images.

- The viewport is shifted within the Mandelbrot set so only certain parts are visible on screen.
- A classic diagram is visible within negative two five and one one five for x values and y values are less constrained.
- Moving around viewport allows users to explore different parts of Mandelbrot set.

## [#](t=0:04:18) Changing Parameters

Section Overview: In this section, viewers learn about changing parameters in Rust programming language and how it
affects output.

- Four parameters are used to create a bounding box for the visualization.
- The x and y values determine the minimum and maximum values for left, right, up, and down movements.
- Two integer parameters determine the number of pixels or characters printed on screen.

## [#](t=0:06:13s) Introduction to Mandelbrot Set

Section Overview: In this section, the speaker introduces the topic of the Mandelbrot set and explains how it uses
complex numbers.

### Understanding Complex Numbers

- [](t=0:06:26s) The Mandelbrot set uses complex numbers instead of real numbers.
- [](t=0:06:55s) A complex number exists in a number plane, with an imaginary part representing the y-axis along the
  number line.

### Implementing Complex Numbers in Rust

- [](t=0:07:17s) Rust does not include complex numbers itself but can pull them in through the num crate.
- [](t=0:08:07s) The memory layer is compatible with an array of type two, enabling us to put in other scalar values.

## [#](t=0:09:20s) Calculating the Mandelbrot Set

Section Overview: In this section, the speaker explains how to calculate whether a point is inside or outside of the
Mandelbrot set using an iterative function.

### Iterative Function for Calculating Points

- [](t=0:09:48s) The iterative function checks whether a pixel represents something that's inside or outside of the set.
- [](t=0:10:14s) The z value and cx/cy represent each pixel as a complex number.
- [](t=0:10.27s) Norm is used to calculate whether a point has escaped from within the Mandelbrot set.

### Using Unsigned Integers and Pointers

- [](t=0.11.22s) Size is used as an unsigned integer in Rust, representing the width most familiar to your CPU.
- [](t=0.11.49s) Pointers are used to access arrays in C.

### Checking Whether a Point is Inside or Outside the Set

- [](t=0:11:54s) The vector between the origin and the place that this number exists inside the complex plane is over
  two.

## [#](t=0:12:45s) Introduction to Complex Numbers

Section Overview: In this section, the speaker introduces complex numbers and explains how they relate to vectors and
origins.

### Complex Numbers and Vectors

- [](t=0:12:45s) The origin is just the zero zero vector.
- [](t=0:13:03s) The imaginary part of a complex number represents how far it has drifted away as it goes through a
  series of transformations.
- [](t=0:13:30s) The square root of the sum of squares of real and imaginary parts can be used to determine if a complex
  number is greater than four.

## [#](t=0:14:15s) Mandelbrot Set

Section Overview: In this section, the speaker discusses the Mandelbrot set and demonstrates how to calculate it using
Python.

### Calculating the Mandelbrot Set

- [](t=0:14:15s) The speaker demonstrates how to calculate the Mandelbrot set using Python.
- [](t=0:15:25s) Different conditions can be used to calculate the Mandelbrot set, with some being more mathematically
  pleasing while others are more aesthetically interesting.
- [](t=0:16:54s) The speaker shows an example function for calculating the Mandelbrot set with various parameters.

## [#](t=0:17.32s) Mapping Between Rows and Columns

Section Overview: In this section, the speaker discusses mapping between rows and columns when calculating the
Mandelbrot set.

### Mapping Between Rows and Columns

- [](t=0.17.32s) Two output values are produced when calculating the Mandelbrot set - number of columns and number of
  rows.
- [](t=0.18.27s) The speaker demonstrates how to map between rows and columns when calculating the Mandelbrot set.

## Introduction to Rust Vectors

Section Overview: In this section, the speaker introduces Rust vectors and explains how they work.

### Creating a Vector with Capacity

- A vector is created using the syntax `vec![type; capacity]`.
- The capacity specifies the number of elements that can be stored in the vector.
- Using this syntax ensures that only one memory allocation is made at a given time.

### Common Approach to Creating a Vector

- The common approach to creating a vector uses simple syntax but results in several allocations.
- In Rust, it's important to care about when you're asking for memory and allocate only when necessary.

### GitHub Gist of Code

- The speaker provides a link to the code being used so viewers can modify it themselves.

## Understanding Rust Syntax for Vectors

Section Overview: In this section, the speaker discusses Rust syntax for vectors and explains why certain types are
needed.

### Static Method for Type Vec

- The static method `with_capacity` is used to specify what type of data will be contained in the vector.
- Rust can guess what type of data will be contained in the vector but it's better to explicitly state it.

### Numeric Types in Rust

- Rust is very picky about numeric types and requires them to be cast specifically.
- All values need to be converted into floats because calculations require percentages and column/row numbers.

## Creating a Point that Best Represents the Pixel

Section Overview: In this section, the speaker explains how they are iterating through the columns and rows in the pixel
output to create a point that best represents that pixel inside the Mandelbrot space.

### Iterating Through Columns and Rows

- The speaker is iterating through the columns and rows in the pixel output.
- They are creating a point that best represents that pixel inside the Mandelbrot space using cx and cy which stand for
  complex x and complex y.
- The point is passed through to this Mandelbrot point calculation where we get this value escaped at.

### Understanding Escaped Points

- If a point escapes very quickly, it doesn't exist within the Mandelbrot set.
- If it stays there, it's definitely inside the set.
- The stuff at the borders is where all of complexity lies.

## Iteration Calculation

Section Overview: In this section, we learn about iteration calculations used to calculate values for every single
pixel.

### Calculating Values for Every Single Pixel

- We have an iteration calculation that took to get out of bounds again.
- We perform an update on some z thing that relates to our position as input to that transformation.
- When it balances out, we return so and say once you've hit two and bounced out.

## Value Between Zero and One Thousand

Section Overview: In this section, we learn about values between zero and one thousand and how they are used to
translate the value to a character.

### Understanding Values Between Zero and One Thousand

- The value here is between zero and one thousand.
- These are all values between zero and a thousand.
- A thousand comes from the fact that we specify a thousand in here to translate the value to a character.

## Changing Output

Section Overview: In this section, we learn about changing the output by going through each row in each column.

### Changing Output

- To change the output, we need to go through each row in each column.
- We need to go row first and then column because the row responds to a line so I can change that one into a question
  mark if I want.
- We can encourage you to play with these outputs.

## Generated by Video Highlight

https://videohighlight.com/video/summary/xDrAncijBq4