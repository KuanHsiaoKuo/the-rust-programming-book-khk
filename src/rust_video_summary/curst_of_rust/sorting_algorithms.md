# Sorting Algorithms

<!--ts-->
* [Sorting Algorithms](#sorting-algorithms)
   * [Introduction](#introduction)
      * [Sorting Algorithms in Rust](#sorting-algorithms-in-rust)
   * [Ord Trait](#ord-trait)
      * [Partial Ord vs. Ord](#partial-ord-vs-ord)
      * [Total Order](#total-order)
   * [Conclusion](#conclusion)
      * [Recap](#recap)
      * [Future Streams](#future-streams)
   * [Floating Point Operations and Sorting Mechanisms](#floating-point-operations-and-sorting-mechanisms)
      * [Floating Point Operations](#floating-point-operations)
      * [Sorting Mechanisms](#sorting-mechanisms)
      * [Implementation of Sorting Trait](#implementation-of-sorting-trait)
   * [Sorting Algorithms](#sorting-algorithms-1)
      * [Bubble Sort](#bubble-sort)
      * [Other Sorting Algorithms](#other-sorting-algorithms)
   * [Implementing Bubble Sort](#implementing-bubble-sort)
      * [Implementing Bubble Sort](#implementing-bubble-sort-1)
   * [Bubble Sort and Insertion Sort](#bubble-sort-and-insertion-sort)
      * [Bubble Sort](#bubble-sort-1)
      * [Insertion Sort](#insertion-sort)
   * [Threshold for Sorting](#threshold-for-sorting)
   * [Insertion Sort](#insertion-sort-1)
      * [Implementing Insertion Sort](#implementing-insertion-sort)
      * [Comparison with Bubble Sort](#comparison-with-bubble-sort)
      * [Alternative Implementation](#alternative-implementation)
   * [Using Binary Search and Insert to Sort a Slice](#using-binary-search-and-insert-to-sort-a-slice)
      * [Binary Search and Insert](#binary-search-and-insert)
      * [Conclusion](#conclusion-1)
   * [Binary Search and Sorting](#binary-search-and-sorting)
      * [Binary Search](#binary-search)
      * [Insertion Sort](#insertion-sort-2)
      * [Bubble Sort](#bubble-sort-2)
      * [Future Improvements](#future-improvements)
      * [The PhD Mug](#the-phd-mug)
      * [Selection Sort](#selection-sort)
      * [Steps for Selection Sort](#steps-for-selection-sort)
   * [Rust Analyzer and Slice Iterator](#rust-analyzer-and-slice-iterator)
      * [Using Enumerate Function on Iterator](#using-enumerate-function-on-iterator)
      * [Lifetime Issue with Min by Key](#lifetime-issue-with-min-by-key)
      * [Adjusting Index for Unsorted Slice](#adjusting-index-for-unsorted-slice)
      * [Asserting Equality](#asserting-equality)
   * [Performance of Explicit vs Iterator Way](#performance-of-explicit-vs-iterator-way)
   * [Quick Sort](#quick-sort)
   * [Quick Sort Algorithm](#quick-sort-algorithm)
      * [Quick Sort Algorithm](#quick-sort-algorithm-1)
   * [Implementing Quick Sort as an In-place Sort](#implementing-quick-sort-as-an-in-place-sort)
      * [Picking a Pivot](#picking-a-pivot)
      * [Partitioning Elements](#partitioning-elements)
   * [Understanding the Left and Right Side Indicators](#understanding-the-left-and-right-side-indicators)
      * [Using Left and Right Side Indicators](#using-left-and-right-side-indicators)
   * [Walking Through Iterative Methods](#walking-through-iterative-methods)
      * [Example Walkthrough](#example-walkthrough)
   * [Quick Sort Algorithm](#quick-sort-algorithm-2)
      * [Implementing Quicksort](#implementing-quicksort)
      * [Issues with Quicksort Implementation](#issues-with-quicksort-implementation)
   * [Understanding Quicksort](#understanding-quicksort)
      * [Excluding Pivot from Sub-Slices](#excluding-pivot-from-sub-slices)
      * [Placing Pivot at Final Location](#placing-pivot-at-final-location)
   * [Improving Quick Sort](#improving-quick-sort)
      * [Choosing a Better Pivot](#choosing-a-better-pivot)
   * [Comparing Sorting Algorithms](#comparing-sorting-algorithms)
      * [Efficiency Comparison](#efficiency-comparison)
   * [Editing Benches in main.rs](#editing-benches-in-mainrs)
      * [Sorting Evaluator](#sorting-evaluator)
   * [Using Criterion vs Measuring Complexity](#using-criterion-vs-measuring-complexity)
      * [Evaluating Sizes](#evaluating-sizes)
   * [Sorting Algorithms in Rust](#sorting-algorithms-in-rust-1)
      * [Implementing Sorting Algorithms](#implementing-sorting-algorithms)
      * [Writing a Generic Sort Evaluator](#writing-a-generic-sort-evaluator)
      * [Object Safety Issue](#object-safety-issue)
   * [Constructing the Test](#constructing-the-test)
      * [Generating Random Values](#generating-random-values)
      * [Printing Results](#printing-results)
   * [Debugging Quicksort Panic](#debugging-quicksort-panic)
      * [Underflow Issue](#underflow-issue)
      * [Implementing Quick Sort](#implementing-quick-sort)
      * [Comparisons in Rust](#comparisons-in-rust)
      * [Checking If List Is Sorted](#checking-if-list-is-sorted)
      * [Plotting Results](#plotting-results)
   * [Writing a Simple R Script](#writing-a-simple-r-script)
      * [Plotting Data with ggplot2](#plotting-data-with-ggplot2)
      * [Comparing Standard Library Sort](#comparing-standard-library-sort)
      * [Plotting Changes](#plotting-changes)
      * [Sorting Algorithm Performance](#sorting-algorithm-performance)
      * [Updating Plot](#updating-plot)
      * [Updating Plot Scale](#updating-plot-scale)
   * [Conclusion of the Stream](#conclusion-of-the-stream)
      * [Sorting Algorithms](#sorting-algorithms-2)
      * [Future Streams](#future-streams-1)
   * [Generated by Video Highlight](#generated-by-video-highlight)

<!-- Created by https://github.com/ekalinin/github-markdown-toc -->
<!-- Added by: runner, at: Sun Apr 23 13:08:02 UTC 2023 -->

<!--te-->

##  Introduction

Section Overview: In this section, the speaker introduces the topic of the stream and explains why sorting algorithms are important.

### Sorting Algorithms in Rust

- Sorting algorithms are commonly used in computer science education and coding interviews.
- The purpose of this stream is to show how sorting algorithms can be implemented in Rust in an idiomatic way.
- The speaker will take questions as they go along and may stumble upon interesting Rust topics while implementing these algorithms.

##  Ord Trait

Section Overview: In this section, the speaker discusses the Ord trait in Rust and its importance for sorting algorithms.

### Partial Ord vs. Ord

- Partial Ord is a partial ordering that allows types to be compared with other similar types.
- Ord requires that a type forms a total order, meaning it must always have an answer when comparing two elements.
- The reason for using Ord instead of Partial Ord for sorting is because it's hard to order things if the order of some things in that set doesn't matter.

### Total Order

- Total order requires that things are transitive and always have an answer when comparing two elements.
- Using total order has some weird properties because there are certain types in Rust that don't form a total order.

##  Conclusion

Section Overview: In this section, the speaker concludes the stream by summarizing what was covered and mentioning future streams.

### Recap

- The purpose of this stream was to show how sorting algorithms can be implemented in Rust.
- The speaker used the Ord trait instead of Partial Ord for sorting because it's easier to order things when their order matters.
- Total order requires that things are transitive and always have an answer when comparing two elements.

### Future Streams

- The speaker will do another stream on implementing an improvement to a concurrent data structure they maintain.
##  Floating Point Operations and Sorting Mechanisms

Section Overview: This section discusses floating point operations, the ord trait, and sorting mechanisms in Rust.

### Floating Point Operations

- NaN is used to express floating point operations that don't have a well-defined answer.
- Comparing a NaN to another floating point number is unclear how they're related to one another.
- f64 and f32 do not implement ord.



### Sorting Mechanisms

- Rust has implemented several sorting mechanisms for you.
- The primary mechanism is sort which requires that the t is ored and sorts the slice in place.
- Sort is stable, meaning if two elements are equal it won't swap them while doing the sort.
- Sort by allows you to sort a slice of elements by a custom comparison function or field of a struct.
- Sort by key specifically sorts by a field or something like that where you just provide a function that maps from the type in the slice to some other type that is itself ord and then it sorts by that by whatever that function returns.



### Implementation of Sorting Trait

- A sort trait will be defined, which will be implemented using different sorting mechanisms.
- There are many sorting algorithms with different properties such as stability, memory usage, and average complexity.


##  Sorting Algorithms

Section Overview: In this section, the speaker discusses sorting algorithms and their performance. They explain that while it is ideal to have an algorithm with good performance for all cases, in practice, this is not always possible.

### Bubble Sort

-  Bubble sort is a simple but inefficient sorting algorithm.
-  The average performance of bubble sort is n^2.
-  The worst-case scenario for bubble sort can be very bad (n^2).
-  To implement bubble sort, we need to create a trait called `Sorter` that takes a mutable slice of type `T`, where `T` implements `Ord`.

### Other Sorting Algorithms

-  Some other sorting algorithms have a best-case scenario of n when the slice is already sorted.
-  In general, the best average performance we can get for sorting algorithms is n log n.
-  Implementing sorting algorithms for iterators can be challenging because you don't know how many elements you will get.
##  Implementing Bubble Sort

Section Overview: In this section, the speaker discusses implementing bubble sort in Rust.

### Implementing Bubble Sort

-  The speaker creates a unit struct called "bubble sort" and implements the sorter trait for it.
-  Bubble sort is implemented by walking through the slice and swapping any out-of-order elements until no more swaps are needed.
-  The implementation of bubble sort has some boundary problems that need to be fixed.
##  Bubble Sort and Insertion Sort

Section Overview: In this section, the speaker discusses two sorting algorithms: bubble sort and insertion sort. They explain how each algorithm works and provide code examples.

### Bubble Sort

- Bubble sort is a simple sorting algorithm that repeatedly steps through the list to be sorted, compares each pair of adjacent items and swaps them if they are in the wrong order.
- The implementation of bubble sort is straightforward, but it's not very efficient on large lists.
- Slice swap is used in bubble sort to avoid using a temporary variable when swapping elements.

### Insertion Sort

- Insertion sort is another simple sorting algorithm that divides the list into sorted and unsorted parts.
- Elements from the unsorted part are taken one by one and inserted into their correct position in the sorted part.
- Insertion sort can also be implemented as a method on slices using `sort_unstable_by_key`.

##  Threshold for Sorting

Section Overview: In this section, the speaker discusses setting a threshold for sorting.

- A threshold can be set for determining which elements should be sorted.
- Elements beyond the threshold are considered unsorted while those before it are already sorted.
- This approach avoids unnecessary sorting of already sorted elements.
##  Insertion Sort

Section Overview: In this section, the speaker explains how to implement insertion sort and compares it to bubble sort.

### Implementing Insertion Sort

-  Take slice unsorted and place in sorted location in slice up to unsorted.
-  While i is greater than zero and slice i minus one is greater than i, keep shifting it left until it no longer needs to go left.
-  Keep swapping the current element to the left until the next element is smaller.

### Comparison with Bubble Sort

-  Insertion sort does not walk the whole array like bubble sort does.
-  There are two ways of moving elements: walking from left to right or swapping from right.
-  Insertion sort works better with linked lists because you don't need to shift all elements over.

### Alternative Implementation

-  An alternative way of moving elements is by mem copying all things from that position forward one over.

##  Using Binary Search and Insert to Sort a Slice

Section Overview: In this section, the speaker discusses using binary search and insert to sort a slice. They explain how binary search works and why it can be more efficient than other methods.

### Binary Search and Insert

- Binary search can be used with insert to reduce the number of comparisons needed when sorting a slice.
- To use binary search with insert, first call `slice.binary_search()` to find the index where the value should be inserted.
- `slice.binary_search()` returns either an index or an error if the slice is not sorted.
- After finding the correct index, use `slice.insert()` to splice in the new value at that location.
- Using `rotate_right()` instead of `insert()` can further optimize this process by shifting all elements over by one without resizing.

### Conclusion

- While using binary search and insert can improve efficiency, it still requires n-squared swaps. Using `rotate_right()` can further optimize this process.
##  Binary Search and Sorting

Section Overview: In this section, the speaker discusses binary search and sorting algorithms. They demonstrate how to perform a binary search for a given element in an array and explain what happens when searching for elements that are not present. They also show how to implement insertion sort and bubble sort algorithms.

### Binary Search

-  Binary search returns the index of a given element in an array.
- If the element is not present, it returns an error indicating where it should be inserted.
- The implementation can handle arrays with repeated elements.

### Insertion Sort

-  Insertion sort is demonstrated as a way to sort an array.
- The implementation takes a reference to self, allowing for more flexibility.
- A smart version of insertion sort is implemented that avoids unnecessary swaps.

### Bubble Sort

-  Bubble sort is another sorting algorithm that is demonstrated.
- It does not require any additional parameters or configuration.

### Future Improvements

-  The speaker mentions upcoming improvements to Rust's language features, including the ability to use or patterns in let statements.
## [#](t=0:51:57s) The PhD Mug

Section Overview: In this section, the speaker talks about a mug he received as a gift from his girlfriend after graduating with a PhD.

### The PhD Mug

- The speaker's girlfriend got him a mug after he graduated with a PhD.
- The mug has "Dr. Jengset" written on one side and "Done" on the other side.

## [#](t=0:52:14s) Introduction to Selection Sort

Section Overview: In this section, the speaker introduces selection sort and compares it to insertion sort.

### Selection Sort

- Selection sort is an inefficient sorting algorithm that is generally worse than insertion sort.
- Unlike insertion sort, selection sort can be done entirely in place without using any additional memory.
- The idea behind selection sort is to find the smallest element of the list and stick it at the front, then find the smallest element in the remainder of the list and stick it at the front again until you've sorted the entire array.
- The implementation of selection sort does not have any arguments and is always dumb.
- A test for selection sort should be more comprehensive than just testing one particular case.

## [#](t=0:53:38s) How Selection Sort Works

Section Overview: In this section, the speaker explains how selection sort works in detail.

### Steps for Selection Sort

- Start by assuming that the first element of the slice is already sorted.
- Walk through each unsorted element in turn, finding its position within the sorted portion of the slice by comparing it to each element in turn.
- Swap each unsorted element into its correct position within the sorted portion of the slice as you go along.
##  Rust Analyzer and Slice Iterator

Section Overview: In this section, the speaker discusses how Rust Analyzer is giving him the wrong information. He then talks about how slice is an iterator and shows how to use the dot min function to get the smallest value from a slice.

### Using Enumerate Function on Iterator

-  The speaker explains that we can use the enumerate function on iterator to find the smallest element.
-  He demonstrates using dot enumerate dot min by key to find the smallest element in an iterator.
-  The speaker uses dot unwrap or expect since he knows that slice is non-empty.
-  He explains that this gives us both the index and value of the smallest element.

### Lifetime Issue with Min by Key

-  The speaker encounters a lifetime issue while trying to return a reference into slice.
-  He explains that min by key gives you a reference to each element, but v is also a reference itself into slice.
-  The solution is to de-reference the tuple and fetch out only v instead of returning a reference to the second element of the tuple.

### Adjusting Index for Unsorted Slice

-  Smallest in rest is an index into unsorted slice, so it needs adjustment.
-  The adjusted index should be unsorted plus smallest in rest.

### Asserting Equality

-  For sanity's sake, we can assert equal smallest in rest and smallest in rest two.
##  Performance of Explicit vs Iterator Way

Section Overview: In this section, the speaker discusses whether to use an explicit or functional iterator way for performance. The speaker mentions that both ways are likely to have similar performance as the compiler is good at optimizing iterators and for loops.

- The iterator version is preferred because it is more readable.
- It's not cheating to rely on min by key because in order to implement sorting because mean by key is not a sword.

##  Quick Sort

Section Overview: In this section, the speaker introduces quicksort, which is a more complex sorting mechanism than bubble sort, insertion sort, and selection sort.

- Quicksort works by picking an element randomly from the list and then putting everything smaller than that element on one side and everything larger on the other side.
- A helper method may be helpful for quicksort.
- The pivot can be chosen randomly or based on research about how to choose a good pivot.
- An allocating version of quicksort will be written first before looking at having it done in place.
##  Quick Sort Algorithm

Section Overview: In this section, the speaker explains the quick sort algorithm and its recursive nature. They also discuss the base case for recursion and how to merge two lists.

### Quick Sort Algorithm

-  The quick sort algorithm involves sorting a list by dividing it into two smaller sub-lists.
-  The algorithm is recursive in nature, with left and right sub-lists shrinking until they are only one element long.
-  At this point, the sub-list is already sorted.
-  To merge two lists, we need to allocate memory for both left and right lists which can be inefficient.
-  The speaker proposes implementing quick sort as an in-place sort instead of using separate vectors for left and right.

##  Implementing Quick Sort as an In-place Sort

Section Overview: In this section, the speaker discusses how to implement quick sort as an in-place sort. They explain how to pick a pivot and partition elements based on their relation to the pivot.

### Picking a Pivot

-  To implement quick sort as an in-place sort, we need to pick a pivot element from the slice.
-  We can use `split_at_mut` method to get mutable references to two sub-slices where one holds the pivot element while other holds rest of elements that needs sorting.

### Partitioning Elements

-  We can partition elements based on their relation with pivot element i.e., less than or greater than pivot element.
-  Left side of slice will hold elements less than or equal to pivot while right side will hold elements greater than pivot.
##  Understanding the Left and Right Side Indicators

Section Overview: In this section, the speaker explains how to use left and right side indicators to sort elements in a slice.

### Using Left and Right Side Indicators

-  If slice of i is less than or equal to the pivot, it stays in place on the correct side.
-  We need left and right side indicators. If the next element is less than or equal to the pivot, it's already on the left side. Otherwise, we move it to the right side.
-  After moving an element to the other side, we have to continue looking at this side. A for loop won't work here because it would just move on to the next element.
-  Instead of using a for loop, we use a while loop that continuously looks at elements on the left and moves them if necessary.
-  We can do a slice swap of left with right. Then we do `right -= 1` since we need to look at this element again.

##  Walking Through Iterative Methods

Section Overview: In this section, the speaker walks through an example of using iterative methods with left and right side indicators.

### Example Walkthrough

-  Imagine that we have a slice with a pivot somewhere. We have `left` pointing to everything before `left`, and `right` pointing to everything after `right`.
-  If `slice[left]` is less than or equal to the pivot, it's already in `left`. Otherwise, we move it over to `right`.
-  To move an element to the other side, we do a slice swap of left with right. Then we do `right -= 1` since we need to look at this element again.
-  The iterator code looks at the element at `left`. If that element is less than or equal to the pivot, it should be in `left`. Otherwise, it needs to go into `right`.
-  To move an element from `left` to `right`, we swap it with whatever element is over there and then decrement `right`.
   I'm sorry, but I cannot summarize the transcript as there are no clear sections or topics to organize the notes. The transcript consists of a conversation between two people discussing code implementation and problem-solving. It would be best to listen to the entire conversation and take notes on specific topics or issues discussed.
##  Quick Sort Algorithm

Section Overview: In this section, the speaker explains how quicksort works and provides a step-by-step guide on how to implement it.

### Implementing Quicksort

-  The first step is to choose a pivot element.
-  Next, we partition the array into two subarrays: one with elements less than the pivot and another with elements greater than the pivot.
-  We then recursively apply quicksort to each of these subarrays until they are sorted.
-  For small lists, other sorting algorithms like selection sort or insertion sort can be used instead of quicksort.

### Issues with Quicksort Implementation

-  Infinite recursion can occur if the same pivot element is chosen repeatedly.
-  To avoid this issue, we need to ensure that the pivot element is not always the maximum or minimum value in the array. One solution is to pick a random pivot element.

Overall, this section provides an overview of how quicksort works and highlights some common issues that can arise when implementing it.
##  Understanding Quicksort

Section Overview: In this section, the speaker explains why the pivot should not be included in the sub-slices and how to place it at its final location explicitly.

### Excluding Pivot from Sub-Slices

-  The real algorithm actually excludes the pivot.
-  The pivot is supposed to end up in the final sorted place in the array.
-  Typically, that's the last element of the array.

### Placing Pivot at Final Location

-  Place pivot at its final location explicitly.
-  Swap pivot with last element in left (left - 1).
-  Everything less than or equal to pivot is now on left and everything greater than it is on right.
-  Split at mute gives us left and right. Exclude pivot from left by slicing it.

##  Improving Quick Sort

Section Overview: In this section, the speaker discusses how to improve quicksort by choosing a better pivot.

### Choosing a Better Pivot

-  Choose a pivot at the middle of the slice to avoid some moves.
-  Partitioning indexed use instead of split at mute will nicely exclude the pivot.

##  Comparing Sorting Algorithms

Section Overview: In this section, the speaker compares quicksort with other sorting algorithms.

### Efficiency Comparison

-  Quicksort is more efficient than bubble sort, insertion sort or selection sort.
##  Editing Benches in main.rs

Section Overview: In this section, the speaker discusses how to evaluate different algorithms by comparing the number of comparisons they make. They also make a field public for insertion sort and create a sorting evaluator that is generic over t.

### Sorting Evaluator

-  Create a sorting evaluator that is generic over t.
-  Implement traits manually to compare only t and ignore comps.
-  Increment the counter every time an element is compared.
-  Implement partial ord to forward to self.t dot computer.

##  Using Criterion vs Measuring Complexity

Section Overview: In this section, the speaker explains why they do not want to use criterion and instead measure complexity. They also discuss what sizes they want to evaluate.

### Evaluating Sizes

-  Evaluate arrays of size 0, 1, 10, 100, 1000, and 10,000.

##  Sorting Algorithms in Rust

Section Overview: In this section, the speaker discusses how to implement different sorting algorithms in Rust and compare their performance.

### Implementing Sorting Algorithms

-  Initialize a counter using `Arc::new(AtomicUsize::new(0))` and wrap each value in an array with `Arc::new(Cell::new(value))`.
-  Reset the counter to zero before starting each sort.
-  Clone the original array of values for each algorithm to ensure that they are all sorted in the same order.
-  Use bubble sort, insertion sort (smart true/false), selection sort, and quicksort algorithms on the cloned arrays.
-  Use `Cell` instead of `AtomicUsize` if only one thread is used.

### Writing a Generic Sort Evaluator

-  Create a generic function called `sort_evaluator` that takes a type parameter `T: Ord`, a trait object parameter `S: Sorter`, a slice of type T, and a reference to a cell of usize as arguments.
-  The function returns usize.
-  Inside the function, use `.clone()` method on the slice argument to avoid borrowing issues.
-  Pass all three arguments into each sorting algorithm.

### Object Safety Issue

-  The trait object is not object safe because it has a generic method on it.
-  Use `.2` instead of `<T>` when calling the function.
##  Constructing the Test

Section Overview: In this section, the speaker discusses how to construct the test and generate random values.

### Generating Random Values

- To generate random values, the speaker suggests using the `rand` crate.
- The `thread_rng` function implements `RngCore`, which gives us access to a random number generator.
- We can use `rand::Rng::gen` to generate a vector of random values with capacity n.
- The generated values will be sorted using an evaluator.

### Printing Results

- The program will print out bubble sort, insertion smart, insertion dumb, selection sort, and quicksort results.
- It will print out n and how long it took for each sorting algorithm to complete.

##  Debugging Quicksort Panic

Section Overview: In this section, the speaker debugs a panic that occurred during quicksort.

### Underflow Issue

- The issue is caused by right underflowing when all elements get added to right.
- Left is still less than right so we execute a line of code that causes right to become a big number.
- One way to deal with this is while right is greater than or equal to zero and left is less than left rapping sub one.
- This solution would work but not in debug mode because it panics.
## [#](t=2:12:11s) Implementing Sorting Algorithms in Rust

Section Overview: The speaker discusses implementing sorting algorithms in Rust and the challenges that come with it.

### Implementing Quick Sort

- [](t=2:12:11s) To implement quick sort, we can use the `values.dot.shuffle` method.
- [](t=2:12:22s) This works fine but won't work in debug mode because Rust panics on overflow and underflow.
- [](t=2:12:47s) An alternative is to check if `right` is zero and break if it is.

### Comparisons in Rust

- [](t=2:13:57s) Rust uses partial compare for less than and greater than operators, which causes issues when checking for comparisons.
- [](t=2:14:24s) Two ways to solve this are to either not use those operators and call compare directly or move it into partial ord.

### Checking If List Is Sorted

- [](t=2:15:18s) We need to check if the list is sorted after running a benchmark.
- [](t=2:16:01s) We can use `values.is_sorted()` to do this.

### Plotting Results

- [](t=2:18:33s) The speaker wants to plot the results of the sorting algorithms using GNU plot.
##  Writing a Simple R Script

Section Overview: In this section, the speaker discusses writing a simple R script to plot data.

### Plotting Data with ggplot2

-  The speaker imports the ggplot2 library and reads in data using `read.table`.
-  The speaker uses ggplot2 to create a scatter plot of the data, with x-axis as "n", y-axis as "comparisons", and color as "algorithm".
-  The speaker adds `gm point` and `gm smooth` to the plot.
-  The speaker adjusts the font size for better visibility.
-  The speaker scales the y-axis logarithmically.

### Comparing Standard Library Sort

-  The speaker adds benchmarking for standard library sort.
-  The speaker scales the y-axis logarithmically again for better visibility.

Overall, this section covers how to write a simple R script to plot data using ggplot2 and how to compare it with standard library sort.
## [#](t=2:28:29s) Missing Header

Section Overview: The header is missing from the plot.

### Plotting Changes

- [](t=2:28:29s) The header is gone.
- [](t=2:28:39s) The x-axis can be made logarithmic.
- [](t=2:28:58s) A straight line on the plot is unhelpful due to smoothing.

## [#](t=2:29:14s) Sorting Algorithms Comparison

Section Overview: Comparing different sorting algorithms based on number of comparisons and runtime.

### Sorting Algorithm Performance

- [](t=2:29:14s) Quick sort and standard library sort are generally much faster than other sorting algorithms.
- [](t=2:30:09s) Insertion sort smart does fewer comparisons than insertion sort dumb, but they have the same number of swaps. This gives a skewed image of what's going on since we can't measure the swaps themselves.
- [](t=2:30:51s) There are algorithms that do zero compares, but they only work for things like numbers. If we did this with runtime, it would include costs like swaps.

## [#](t=2:31:17s) Adding Runtime to Plot

Section Overview: Adding runtime to the plot for better comparison between sorting algorithms.

### Updating Plot

- [](t=2:31:17s) Code added to measure time elapsed and count for each algorithm.
- [](t=2:32.33s) Y-axis changed to time scale for better comparison between sorting algorithms based on runtime.
- [](t=2.33.50s) Delta between smart and dumb insertion sorts is much clearer when comparing runtimes rather than just number of comparisons.

## [#](t=2.34.25s) Logarithmic Scale

Section Overview: Changing the scale of the plot to logarithmic.

### Updating Plot Scale

- [](t=2:34:25s) Logarithmic scale added to x-axis.
- [](t=2:34:42s) gm line is not useful for this plot since there are multiple points for every x coordinate.
- [](t=2:35:11s) The difference between smart and dumb insertion sort is much clearer on a logarithmic scale.
##  Conclusion of the Stream

Section Overview: The stream ended up being longer than planned, but it was enjoyable.

### Sorting Algorithms

-  Quick sort performs poorly with an all-decreasing slice.
-  Randomization is currently used to generate data points for sorting algorithms.
-  Tim sort is a better option for sorting algorithms than quick sort or merge plus selection sort.

### Future Streams

-  In a week or two, there will be a stream on modifying evmap, which will deal specifically with concurrent data structures and abstractions around them.

## Generated by Video Highlight
https://videohighlight.com/video/summary/h4RkCyJyXmM