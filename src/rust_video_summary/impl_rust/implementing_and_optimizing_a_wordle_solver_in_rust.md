# (3/6h)Implementing and Optimizing a Wordle Solver in Rust

<!--ts-->
* [(3/6h)Implementing and Optimizing a Wordle Solver in Rust](#36himplementing-and-optimizing-a-wordle-solver-in-rust)
      * [Playing Wordle](#playing-wordle)
      * [What to Expect](#what-to-expect)
   * [Solving Wordle with Information Theory](#solving-wordle-with-information-theory)
      * [Algorithm Explanation](#algorithm-explanation)
      * [Using Hyperfine for Performance Optimization](#using-hyperfine-for-performance-optimization)
      * [Multithreading Considerations](#multithreading-considerations)
   * [Introduction](#introduction)
      * [Algorithmic Approach](#algorithmic-approach)
   * [Getting Started](#getting-started)
      * [Obtaining Wordle Dictionary](#obtaining-wordle-dictionary)
      * [Obtaining Google Books N-Gram Dataset](#obtaining-google-books-n-gram-dataset)
   * [Conclusion](#conclusion)
      * [Summary](#summary)
      * [How the Data is Constructed](#how-the-data-is-constructed)
      * [Using Ripgrep to Search for Five-Letter Words](#using-ripgrep-to-search-for-five-letter-words)
      * [Extracting Occurrence Count for Each Word](#extracting-occurrence-count-for-each-word)
      * [Normalizing Data](#normalizing-data)
   * [Using AWK to Combine Consecutive Records](#using-awk-to-combine-consecutive-records)
      * [Combining Consecutive Records](#combining-consecutive-records)
      * [Computing Total Count and Probability](#computing-total-count-and-probability)
   * [Creating a Wordle Dictionary File](#creating-a-wordle-dictionary-file)
      * [Importing Wordle Dictionary as JSON File](#importing-wordle-dictionary-as-json-file)
      * [Joining Dictionary with Frequency Count File](#joining-dictionary-with-frequency-count-file)
      * [Handling Words Without Counts](#handling-words-without-counts)
   * [Introduction to Wordle Solver](#introduction-to-wordle-solver)
      * [Naming the Dev Streams](#naming-the-dev-streams)
      * [Writing Rust Code](#writing-rust-code)
      * [Creating Functions](#creating-functions)
      * [Guessing Functionality](#guessing-functionality)
      * [Program Structure](#program-structure)
      * [Implementing Algorithms](#implementing-algorithms)
      * [Playing Wordle](#playing-wordle-1)
      * [Computing Correctness](#computing-correctness)
      * [Pushing Guesses into History](#pushing-guesses-into-history)
      * [Returning Iteration Number](#returning-iteration-number)
      * [Setting Maximum Number of Guesses](#setting-maximum-number-of-guesses)
      * [Handling Infinite Loop](#handling-infinite-loop)
      * [Error Message](#error-message)
   * [Implementing a Guess Checker](#implementing-a-guess-checker)
      * [Implementing the Guess Checker](#implementing-the-guess-checker)
      * [Optimizing Code](#optimizing-code)
      * [Marking Correctness](#marking-correctness)
      * [Marking Correctness and Misplaced Characters](#marking-correctness-and-misplaced-characters)
      * [Writing Tests for Marking Correctness and Misplaced Characters](#writing-tests-for-marking-correctness-and-misplaced-characters)
      * [Implementing Naive Functionality](#implementing-naive-functionality)
      * [Creating a Macro for Writing Correctness Things](#creating-a-macro-for-writing-correctness-things)
   * [Wordle Algorithm](#wordle-algorithm)
      * [Developing the Play Algorithm](#developing-the-play-algorithm)
      * [Checking for Valid Guesses](#checking-for-valid-guesses)
   * [Creating a Dictionary](#creating-a-dictionary)
      * [Creating a Dictionary](#creating-a-dictionary-1)
   * [Implementing Play Function](#implementing-play-function)
      * [Implementing Play Function](#implementing-play-function-1)
   * [Guess to String](#guess-to-string)
      * [Implementing Closures](#implementing-closures)
      * [Troubleshooting Errors](#troubleshooting-errors)
   * [Fixing Code and Testing Guessing Game](#fixing-code-and-testing-guessing-game)
      * [Fixing Code Issues](#fixing-code-issues)
      * [Testing Guessing Game](#testing-guessing-game)
   * [Checking Naive Implementation](#checking-naive-implementation)
      * [Checking Naive Implementation](#checking-naive-implementation-1)
   * [[t=5463s] Parsing and Computing Word Count](#t5463s-parsing-and-computing-word-count)
      * [Parsing Words](#parsing-words)
      * [Computing Word Count](#computing-word-count)
   * [[t=5541s] Computing Goodness of Guesses](#t5541s-computing-goodness-of-guesses)
      * [Looping Over Candidate Words](#looping-over-candidate-words)
      * [Comparing Goodness of Guesses](#comparing-goodness-of-guesses)
   * [[t=5671s] Updating Remaining Based on History](#t5671s-updating-remaining-based-on-history)
      * [Retaining Matching Information](#retaining-matching-information)
   * [[t=5873s] Retaining Words Based on Guesses](#t5873s-retaining-words-based-on-guesses)
      * [Matching Words with Guesses](#matching-words-with-guesses)
      * [Logic for Matching Words](#logic-for-matching-words)
   * [[t=6020s] Checking Misplaced Letters](#t6020s-checking-misplaced-letters)
      * [Checking Greens](#checking-greens)
      * [Sorting Used Letters](#sorting-used-letters)
      * [Looping through characters](#looping-through-characters)
      * [Searching for Unused Letters](#searching-for-unused-letters)
      * [Finding Occurrences of Characters](#finding-occurrences-of-characters)
   * [Matching Algorithm](#matching-algorithm)
      * [Matching on Previous Guess](#matching-on-previous-guess)
      * [Handling Misplaced Characters](#handling-misplaced-characters)
      * [Plausible Matches](#plausible-matches)
   * [Testing the Guess Matcher](#testing-the-guess-matcher)
      * [Constructing a Guess](#constructing-a-guess)
      * [Writing Macros for Testing](#writing-macros-for-testing)
      * [Incremental Solver](#incremental-solver)
   * [Pruning Logic](#pruning-logic)
      * [Incremental Pruning](#incremental-pruning)
      * [Eliminating Words](#eliminating-words)
   * [Guessing Different Letters](#guessing-different-letters)
   * [Disallowing Incorrect Patterns](#disallowing-incorrect-patterns)
   * [Debugging the Code](#debugging-the-code)
      * [Debugging the Disallowed Inputs](#debugging-the-disallowed-inputs)
   * [The input "rita bond a b c d e plus w w disallows b c d e a" is being disallowed.](#the-input-rita-bond-a-b-c-d-e-plus-w-w-disallows-b-c-d-e-a-is-being-disallowed)
   * [The speaker questions why this input should be disallowed since all letters are present but in different arrangements.](#the-speaker-questions-why-this-input-should-be-disallowed-since-all-letters-are-present-but-in-different-arrangements)
   * [Upon further inspection, it is discovered that all of the letters are grayed out, indicating that they are incorrect.](#upon-further-inspection-it-is-discovered-that-all-of-the-letters-are-grayed-out-indicating-that-they-are-incorrect)
      * [Identifying the Issue with Early Returns](#identifying-the-issue-with-early-returns)
   * [The speaker examines early returns in the code to determine which one is being taken.](#the-speaker-examines-early-returns-in-the-code-to-determine-which-one-is-being-taken)
   * [It is determined that an incorrect return statement is being taken twice.](#it-is-determined-that-an-incorrect-return-statement-is-being-taken-twice)
   * [Further investigation reveals that previously correct letters were being re-evaluated, causing errors in matching letters.](#further-investigation-reveals-that-previously-correct-letters-were-being-re-evaluated-causing-errors-in-matching-letters)
      * [Optimizing Code](#optimizing-code-1)
   * [A suggestion is made to optimize the code by comparing candidate words to answers element by element for correctness.](#a-suggestion-is-made-to-optimize-the-code-by-comparing-candidate-words-to-answers-element-by-element-for-correctness)
   * [The speaker considers this proposal but ultimately decides against it due to excessive complexity.](#the-speaker-considers-this-proposal-but-ultimately-decides-against-it-due-to-excessive-complexity)
   * [Refactoring Code for Optimization](#refactoring-code-for-optimization)
      * [Simplifying Code with Zip Function](#simplifying-code-with-zip-function)
   * [The speaker simplifies some of the code by using Python's zip function instead of manually iterating through lists.](#the-speaker-simplifies-some-of-the-code-by-using-pythons-zip-function-instead-of-manually-iterating-through-lists)
      * [Refactoring for Efficiency](#refactoring-for-efficiency)
   * [The speaker discusses refactoring parts of the code for efficiency, such as using sets instead of lists.](#the-speaker-discusses-refactoring-parts-of-the-code-for-efficiency-such-as-using-sets-instead-of-lists)
   * [Finalizing Code](#finalizing-code)
      * [Testing Final Code](#testing-final-code)
   * [The speaker tests the final version of the code with various inputs to ensure that it is working correctly.](#the-speaker-tests-the-final-version-of-the-code-with-various-inputs-to-ensure-that-it-is-working-correctly)
      * [Submitting Code](#submitting-code)
   * [The speaker submits the final version of the code for review.](#the-speaker-submits-the-final-version-of-the-code-for-review)
   * [Introduction](#introduction-1)
      * [Naive Algorithm](#naive-algorithm)
      * [Improved Algorithm](#improved-algorithm)
   * [Computing Correctness](#computing-correctness-1)
      * [Pruning Possibilities](#pruning-possibilities)
      * [Exploiting Equivalence](#exploiting-equivalence)
      * [Computing Goodness](#computing-goodness)
   * [Probability of Words](#probability-of-words)
      * [Total Count](#total-count)
      * [Probability Calculation](#probability-calculation)
   * [Conclusion](#conclusion-1)
      * [Summary](#summary-1)
      * [Efficiency](#efficiency)
   * [Computing the Goodness of Words](#computing-the-goodness-of-words)
      * [Formula for Computing Goodness](#formula-for-computing-goodness)
      * [Intuition Behind the Formula](#intuition-behind-the-formula)
      * [Helper Function for Permutations](#helper-function-for-permutations)
   * [Permutations](#permutations)
      * [Using Cartesian Product for Power Sets](#using-cartesian-product-for-power-sets)
   * [Computing Patterns](#computing-patterns)
      * [Looping Over Possible Patterns](#looping-over-possible-patterns)
      * [Computing What Is Left After Each Guess](#computing-what-is-left-after-each-guess)
   * [Understanding the Code](#understanding-the-code)
      * [Probability of Getting a Pattern](#probability-of-getting-a-pattern)
      * [Computing Entropy](#computing-entropy)
      * [Improving Code Efficiency](#improving-code-efficiency)
   * [Generated by Video Highlight](#generated-by-video-highlight)

<!-- Created by https://github.com/ekalinin/github-markdown-toc -->
<!-- Added by: runner, at: Sat Apr 22 15:51:29 UTC 2023 -->

<!--te-->

## [0:00:01](t=1s) Introduction to Wordle

Section Overview: In this section, the speaker introduces the topic of the video, which is playing Wordle. They explain
that they will be writing a program to play the game and optimizing it for performance.

### Playing Wordle

- [](t=0:01:22s) Wordle is a simple word game where players try to guess a five-letter English word.
- [](t=0:02:02s) When guessing, players are told which letters they have guessed correctly, which letters aren't in the
  word, and any letters that are in the word but not where they put them.
- [](t=0:03:23s) The goal is to guess the correct word in as few guesses as possible. The average number of guesses is
  around four.
- [](t=0:04:24s) There is a dictionary of allowed words and a dictionary of answer words. The speaker will only use
  knowledge available to players when writing their program.

## [0:00:19](t=19s) Overview of Video

Section Overview: In this section, the speaker provides an overview of what viewers can expect from this video.

### What to Expect

- [](t=0:00:41s) This video will focus on writing Rust code and performance profiling/optimization.
- [](t=0:01:43s) The speaker hopes that this video will be more approachable than previous ImpulRust videos since it
  won't involve complex technical concepts like unsafe pointer manipulation.
- [](t=0:02:46s) The goal is to write a program that plays Wordle as fast as possible.

## Solving Wordle with Information Theory

Section Overview: In this video, the speaker explains how to solve Wordle using information theory. The algorithm from
this video will be followed and implemented in Rust. The program's performance will be optimized using a tool called
hyperfine.

### Algorithm Explanation

- The algorithm is not explained in detail as it is expected that viewers have watched the video.
- A bug was found in Grant's implementation, but it does not affect the algorithm much.
- This video is a great candidate for performance profiling and optimization because of its large search space.
- The program will be implemented, run, and iteratively optimized to make it faster using hyperfine.

### Using Hyperfine for Performance Optimization

- Hyperfine is a tool that measures the performance of multiple commands that do ostensibly the same thing.
- It gathers variance and execution time data to give you some measurement of statistical significance of performance
  differences.
- Optimizations will be measured to see which ones make a difference, and then they will be iteratively applied to make
  the program faster.

### Multithreading Considerations

- Multithreading may or may not be important for optimization since it tends to hide true program performance.

## Introduction

Section Overview: In this section, the speaker introduces the topic of the video and explains that they will be
implementing an algorithm for playing Wordle using Rust.

### Algorithmic Approach

- The speaker explains that at the core of their approach lies the idea of information theory.
- They plan to use this theory to compute how much information they can expect to get from a given guess in Wordle.
- By picking the word that is likely to give them the highest expected amount of information for each guess, they hope
  to whittle down the list of possible words as quickly as possible.

## Getting Started

Section Overview: In this section, the speaker explains what resources are needed to implement their algorithm and how
to obtain them.

### Obtaining Wordle Dictionary

- The first resource needed is the Wordle dictionary.
- The dictionary can be found in a JavaScript file located in the application sources.
- The `ma` array contains all target words for each day, while `oa` contains all allowed words but not `ma`.
- Concatenating these two arrays gives us all dictionary words needed for our implementation.

### Obtaining Google Books N-Gram Dataset

- To compute which words are more likely to be correct guesses without relying on true answers, we need access to a
  large dataset.
- The Google Books N-Gram Dataset provides counts of n-grams (consecutive sequences of n items) across all books scanned
  by Google Books over many years.
- We will use one gram data set which has single word counts.

## Conclusion

Section Overview: In this section, the speaker concludes by summarizing what was covered in the video and emphasizing
that their focus was on Rust rather than algorithmic optimization.

### Summary

- The speaker implemented an algorithm for playing Wordle using Rust.
- Their approach was based on information theory and involved picking the word that would give them the highest expected
  amount of information for each guess.
- They obtained the Wordle dictionary from a JavaScript file in the application sources and used the Google Books N-Gram
  Dataset to compute which words were more likely to be correct guesses.
- The focus of their implementation was on Rust rather than algorithmic optimization.

## [English](t=902s) Introduction to the Google Books Ngram Viewer

Section Overview: In this section, the speaker introduces the Google Books Ngram Viewer and explains how it works.

### How the Data is Constructed

- The data is constructed such that every line starts with a word, followed by the year, number of occurrences of that
  word in that year, and the number of distinct books that word appears in.
- The left number is always higher than the right number. There are tab-separated entries for each line.

### Using Ripgrep to Search for Five-Letter Words

- Ripgrep can be used to search for all five-letter words across all files with a specific suffix using `-z` flag to
  scan compressed files and `-i` flag to print multiple files without printing out their names.
- The results are saved in `five letters.text`.

### Extracting Occurrence Count for Each Word

- Awk is used to extract occurrence count for each word from `five letters.text`. Field separator is set as tab and
  first field (word) and second-to-last field (occurrence count) are printed. Underscores are substituted with commas
  before running awk with comma as separator. Results are saved in `five letters occur file`.

### Normalizing Data

- Capital letters are replaced with lowercase letters using tr command before sorting data alphabetically. Multiple
  entries exist for same words due to case sensitivity. Sorted list provides normalized data.

## Using AWK to Combine Consecutive Records

Section Overview: In this section, the speaker explains how they used AWK to combine consecutive records with the same
word and wrote it into a combined file. They also show how to sort by count and compute the total count.

### Combining Consecutive Records

- The speaker uses AWK to combine consecutive records with the same word.
- The resulting combined file has about 900,000 distinct words.
- The speaker shows how to sort by count and display the top 10 most frequently occurring words in the data set.

### Computing Total Count and Probability

- The speaker computes the total number of occurrences in the data set using a calculator.
- They then divide each word's occurrence count by the total count to get its probability of occurring.
- Finally, they print out the top 10 words ranked by probability.

## Creating a Wordle Dictionary File

Section Overview: In this section, the speaker explains how they created a dictionary file containing all possible legal
guesses in Wordle, including answers.

### Importing Wordle Dictionary as JSON File

- The speaker imported a JSON file containing all possible legal guesses in Wordle, including answers.

### Joining Dictionary with Frequency Count File

- They used `jq` to pull out every entry from the concatenated two lists and sorted them out.
- Then they joined this dictionary file with a frequency count file using `join`.
- This resulted in a list of legal Wordle guesses with their occurrence count.

### Handling Words Without Counts

- The speaker explains how to handle words without counts by looking for anything that ends with a letter.
- They add these words to the dictionary file with a count of one since they didn't appear in the engram's data set.

## Introduction to Wordle Solver

Section Overview: In this section, the speaker introduces the purpose of the project and explains why they need a list
of possible words to run their bot against.

### Naming the Dev Streams

- The speaker suggests naming the dev streams and has already come up with a name for one: "roget".
- The name is inspired by Roget's Thesaurus, which allows users to explore related words and find better words to
  accurately represent what they're trying to say.

### Writing Rust Code

- The speaker starts writing Rust code for the Wordle solver.
- They copy in "wordle_answers.txt" and rename "wordle_dictionary_with_counts" as "dictionary.txt".

### Creating Functions

- Two functions are needed: one that lets you play Wordle and another that tries to do your guesses.
- The function "play" takes an answer (a static string) and a guesser (generic).
- It plays six rounds where it invokes the guesser each round.

### Guessing Functionality

- The guesser needs to return a word.
- It is allowed to take mute itself in order to guess.
- It will be told all of its past guesses.
- A guess consists of the combination of the word guessed and its correctness mask.
    - Correctness can be green (correct), yellow (present but misplaced), or gray (missing).

## [#](t=0:32:43s) Basic Structure

Section Overview: In this section, the speaker discusses the basic structure of the program and how it will work.

### Program Structure

- The program will have a history and a target answer.
- The next guest to make is going to be string given this history.
- The program will include games that are stored in answers.txt.
- The program will play against the guesser, but we don't know who the guesser is yet.
- A convenience function for implementing at mu t where t implements guesser is created.
- Naive new algorithm is instantiated.

### Implementing Algorithms

- Multiple implementations can be implemented in one binary and switched based on a flag or something so that it's easy
  to benchmark multiple ones at once.
- A mod algorithms is created with naive as its first implementation.

### Playing Wordle

- Initially, let mute history be vector for i in...
- There won't be any limit on the number of times you can guess because we want to know whether the bot succeeded or
  failed.

## [English](t=2327s) Overview of the Code

Section Overview: In this section, the speaker discusses how to compute the correctness based on a guess and how to push
it into history. They also mention that if the guess is equal to the answer, they will return which iteration it
succeeded at.

### Computing Correctness

- The correctness is an array of length five.
- We need something that computes the correctness so we're going to need a check which takes the answer and a guess and
  gives you back a correctness.
- We're going to have it be called compute.
- Here we're gonna assert that answer.len equals five and assert that guess.len equals five.

### Pushing Guesses into History

- Initially, the history is empty and it's gonna grow over time.
- We do something in order to compute the correctness based on that guess.
- Then we're going to do history.push(guess), which is going to be the guess and its corresponding correctness.

### Returning Iteration Number

- If the guess is equal to answer then we return i otherwise we compute the correctness and keep going.
- We want to say starting with one because if you guessed correctly on your first guess then we should return one and
  not zero because you did one guess.

## [English](t=2428s) Limiting Guesses

Section Overview: In this section, they discuss limiting guesses for Wordle game.

### Setting Maximum Number of Guesses

- Wordle only allows six guesses but they allow more guesses than that in order not to chop off score distribution for
  stats purposes.
- They set 32 as maximum number of guesses allowed.

### Handling Infinite Loop

- They could bound this loop if they really wanted to but realistically wordle only allows six guesses so there
  shouldn't be any infinite loops.
- However, they could guard against something like a user implementing a guesser that just never succeeds.

### Error Message

- They added an error message to handle the case where the program did not finish within the maximum number of guesses
  allowed.

## Implementing a Guess Checker

Section Overview: In this section, the speaker discusses how to implement a guess checker that always gives the correct
answer. They explore different ways of implementing this and discuss potential bugs in the code.

### Implementing the Guess Checker

- The guesser should always be able to give a guess right.
- If you know statically that it might not be able to find the answer, that seems like its own kind of problem.
- There are a couple of ways we could implement this.
- We could do something like for i in zero to five if answer i is equal to guess i then c i is correctness.

### Optimizing Code

- This is where the annoying thing about using utf-8 comes in; you can't easily index into this.
- An optimization plan for later is to make this be a five length byte array but not yet.
- The code needs to be as naive as possible in the initial iteration.

### Marking Correctness

- For each character in both answers and guesses, mark them green if they are correct.
- Otherwise, check whether there are any other occurrences of this letter in the guess so that we can mark it yellow or
  red accordingly.
- We're going to have an inner iteration here because we want to check whether this character occurs anywhere else and
  we haven't already marked it as misplaced.

## [#](t=0:53:17s) Marking Correctness and Misplaced Characters

Section Overview: In this section, the speaker discusses how to mark correctness and misplaced characters in a guessing
game. They walk through the logic of marking each character as green or yellow based on whether it has been used to mark
something as correct or misplaced.

### Marking Correctness and Misplaced Characters

- The function `compute` is used to mark correctness and misplaced characters.
- Use `if c is correctness` instead of `marked` to check if an answer is correct.
- For each letter in the answer, mark whether that character has been used to mark something green or yellow.
- Walk through all of the characters of the guess. If they're already correct, there's nothing to do. Otherwise, if
  there are any characters in the answer that are equal to the guest character and haven't already been used to mark
  something as yellow then we mark it as used and then we say yup this is a yellow character otherwise it is not and you
  need to keep looking.
- Return `c`.

## [#](t=0:55:08s) Writing Tests for Marking Correctness and Misplaced Characters

Section Overview: In this section, the speaker writes tests for marking correctness and misplaced characters using
the `compute` function.

### Writing Tests for Marking Correctness and Misplaced Characters

- Write tests for basic functionality using `assertEqual`.
- Create a sub-module for compute tests.
- Test with different inputs such as all green, all gray, all yellow, repeat green.

## [#](t=0:58:27s) Implementing Naive Functionality

Section Overview: In this section, the speaker implements naive functionality using pub.

### Implementing Naive Functionality

- Implement default functionality using pub.
- Arguably just be implemented default really.

## [#](t=1:00:24s) Creating a Macro for Writing Correctness Things

Section Overview: In this section, the speaker creates a macro to make writing correctness things easier.

### Creating a Macro for Writing Correctness Things

- Create a macro to make writing correctness things easier.
- Use the syntax `m is going to be turned into correctness misplaced`.
- This will turn into something funky but it works.

## Wordle Algorithm

Section Overview: In this section, the speaker discusses the development of the Wordle algorithm and how it marks
correctness appropriately.

### Developing the Play Algorithm

- The speaker explains that each character in Wordle is repeated by calling a macro and separating by a
  comma. [](t=1m31s)
- They make some corrections to their code to ensure that things look nice. [](t=1m57s)
- The speaker tests different use cases from chat to ensure that their algorithm is correct. [](t=1m04s)

### Checking for Valid Guesses

- The speaker realizes they need to check whether a guess is valid and considers using a perfect hash phf
  set. [](t=1m06s)
- They explore different options for creating a dictionary, including using a const function or constructor. [](t=1m07s)

Overall, this section covers the development of the Wordle algorithm and checking for valid guesses. The speaker tests
different use cases from chat to ensure that their algorithm is correct and explores different options for creating a
dictionary.

## Creating a Dictionary

Section Overview: In this section, the speaker discusses creating a dictionary for the Wordle game.

### Creating a Dictionary

- The dictionary will be measured by the benchmark.
- The speaker is sad about it but it's fine for now.
- A constant variable called "dictionary" is created as a sorted vector.
- The hash set from collections is used to create the frequency count of words in the dictionary.

## Implementing Play Function

Section Overview: In this section, the speaker discusses implementing the play function for Wordle.

### Implementing Play Function

- The speaker considers using a build script to emit what phf requires.
- Using dot step by two because every other value is the frequency count from eater.
- Lines are used to split every element and then advanced by two.
- Alec suggests using lines instead of line.split once by space.
- Expect that every line is word plus space plus frequency count and dot one.
- Assert that self.dictionary contains guess.
- Debug assert could be used but not necessary since there isn't a human player who doesn't know what the dictionary is.

## Guess to String

Section Overview: In this section, the speaker discusses writing test implementations and closures. They also encounter
an error while trying to implement a closure.

### Implementing Closures

- The speaker wants to write test implementations that don't need state and will use closures.
- They create a closure called "moved" that returns "moved to string".
- They want to assert equal that they should get a score of one using crate wordle.
- The closure doesn't implement fn, so the speaker renames it as "guess".
- The guess function won't play ball, so the speaker creates a helper called "guess sir" which takes history as an ident
  and then takes a block.
- The guesser is always going to be like this but with the ability for the guesser to contain state that it might mutate
  over time.

### Troubleshooting Errors

- There's some lifetime not matching up in the implementation of guesser for g.
- The trait needs to be in scope.
- The trade guesser is not implemented for that, even though there is an implementation found.

## Fixing Code and Testing Guessing Game

Section Overview: In this section, the speaker fixes some code issues and tests a guessing game.

### Fixing Code Issues

- The speaker mentions that something needs to be fixed in the code.
- They mention that they need to guess something.
- The speaker plans to make a config test to get rid of a warning.
- Main is complaining about something, so the speaker needs to do roget wordle new and then w dot play.

### Testing Guessing Game

- The speaker mentions that every line is word plus space plus frequency is a lie it claims.
- They say "really".
- The speaker realizes they missed one thing - dictionary.
- They don't understand which word they are missing - add view.
- The search was entirely wrong, and the speaker doesn't know why they wrote it like that.
- The speaker says "great".
- They say "fantastic" because now they can play a game and see if they immediately guess the right word.
- The speaker suggests making this fancier by adding history.len == 1 for right guesses.
- If history.len == 1, return right; otherwise, return wrong.
- This should now take two guesses.
  -[]( t =5172 s )The words genius, impressive, magnificent, splendid, great are all used interchangeably in the game.
- The speaker says "oh".
- They say "really" again.
- Something is right or wrong and not in the dictionary.
- The speaker confirms that they are all right.
- They say something is wrong.
- The speaker realizes that guess wrong is not in the dictionary.
- They need to update some two, some three, some four, some five, and some six.
- The speaker asks how "wrong" got there.
- They ask for a hash set because something isn't right.
- It has all the numbers because this needs to be dot zero and they realize their mistake.
  -[]( t =5339 s )The speaker says "great," and everything works fine now.

## Checking Naive Implementation

Section Overview: In this section, the speaker checks a naive implementation.

### Checking Naive Implementation

-[]( t =5374 s )The speaker mentions that they want to check if something only ever guesses wrong then it eventually
terminates.
-[]( t =5400 s )They mention that naive is still claimed not to be there even though it should be there.

## [t=5463s] Parsing and Computing Word Count

Section Overview: In this section, the speaker discusses parsing and computing word count. They mention bringing in the
dictionary and lib, parsing that could happen just once, and computing word count using `count.parse`.

### Parsing Words

- The speaker mentions that parsing is something that could happen just once.
- They suggest bringing in one cell or something to cache this.
- However, they decide to keep it as is for now since it's named "naive" for a reason.

### Computing Word Count

- The speaker explains that they will compute word count using `count.parse`.
- They expect every count to be a number and then return word and count.

## [t=5541s] Computing Goodness of Guesses

Section Overview: In this section, the speaker talks about computing how good of a guess each candidate word is. They
loop over every word and count in self.remaining.

### Looping Over Candidate Words

- The speaker explains that for each remaining word (candidate), they will compute how good of a guess it is.
- They loop over every word and count in self.remaining.

### Comparing Goodness of Guesses

- The speaker asks if the current candidate is better than the best one.
- To determine which candidate is better, they need a score or goodness.
- This score will be a floating-point number for now.

## [t=5671s] Updating Remaining Based on History

Section Overview: In this section, the speaker discusses updating self.remaining based on history. If we previously made
a guess, we just learned which ones we got correct. We need to update remaining based on that new information.

### Retaining Matching Information

- The speaker explains that we have some mask in last and want to only retain things in remaining that match the updated
  information we got.
- We will do this using `self.remaining.retain()`.

## [t=5873s] Retaining Words Based on Guesses

Section Overview: In this section, the speaker discusses how to retain a word based on guesses and previous information.
They explain that the count is irrelevant for retaining words and that they only want to retain a word if it matches the
last guess or mask.

### Matching Words with Guesses

- The speaker explains that they need to figure out whether a given piece of information matches a word.
- They discuss how to match a word with previous guesses by using zip and asserting that the length of the word is five.
- The speaker mentions creating a "matches" method to determine whether a given guess matches with a word.

### Logic for Matching Words

- The speaker discusses how to check if a letter in the previous guess was green and doesn't match with the current
  letter in the word.
- They mention that if g is not equal to w, then we can return false because we know that this cannot be the right
  answer.
- If it's wrong and g is equal, then we also know that it can't match because you guessed the letter that the previous
  guest told you was wrong.
- If it's misplaced, then we don't know what to do yet.

## [t=6020s] Checking Misplaced Letters

Section Overview: In this section, the speaker discusses checking misplaced letters when matching words with guesses.

### Checking Greens

- The speaker explains how they first check greens when matching words with guesses.

### Sorting Used Letters

- The logic for computing correctness is similar to computing misplaced letters but reusing it may not be possible.
- The speaker mentions needing some sort of used thing here too.

## [#](t=1:44:23s) Looping through characters

Section Overview: In this section, the speaker discusses how to loop through the characters of a word and search for an
unused letter that is yellow.

### Looping through characters

- [](t=1:44:49s) The speaker suggests that they can do this in one loop by walking all of the characters of just the
  word.
- [](t=1:45:16s) The second loop might be all they need.
- [](t=1:45:25s) They discuss if g is equal to if the previous guess is equal to the word.
- [](t=1:46:19s) They suggest searching for an unused letter that is yellow.

## [#](t=1:47:07s) Searching for Unused Letters

Section Overview: In this section, the speaker discusses how to search for unused letters and find any other characters.

### Searching for Unused Letters

- [](t=1:47:12s) The speaker suggests searching for unused letters.
- [](t=1:48:11s) They suggest using map to find any unused indices.
- [](t=1:48:56s) If there are any other characters, they look for the first unused one.

### Finding Occurrences of Characters

- [](t=1:50.02s) If the previous guess was correct in that location then if the characters are different then we know
  that there's nothing to do otherwise used of i is true and we continue.
- [](t=1.51.01s) They look for the first occurrence of that character if any in the previous guess that is unused so if
  unused of...

## Matching Algorithm

Section Overview: In this section, the speaker discusses the matching algorithm used to determine if a guess is correct
or not.

### Matching on Previous Guess

- The color of the previous guess will tell us whether this guess might be okay.
- We're going to match on what that previous guess was.
- If the previous guess was correct, then it should already be marked as used or we should have returned so this should
  be unreachable.

### Handling Misplaced Characters

- If a character is misplaced, then used of j is true and we can return some of j. This might be the same case as we did
  earlier so maybe this is just if any.
- You guessed an x in some position and in the previous guess you were told that there is no x well then your whole
  guess must be wrong so this is sort of a return false kind of situation which really is a return falls from the whole
  thing.

### Plausible Matches

- If we found one letter we can use, then word might still match otherwise plausible is false. I suppose plausible is
  initially true. If we get here then plausible is false.
- If in the previous guess we got a yellow here that means that this cannot be the answer so w was yellow in the same
  position it was yellow in the same position last time around which means that word cannot be the answer that match
  cannot be the answer so plausible is false and we return false.

## Testing the Guess Matcher

Section Overview: In this section, the speaker discusses testing the guess matcher and constructing a guess.

### Constructing a Guess

- To construct a guess, we need to use a word and mask.
- The mask can be created using the masker made earlier.
- A simple check is performed to ensure that the constructed guess works.

### Writing Macros for Testing

- Macros are used to write tests more efficiently.
- A macro called "guess" is created to take in a word and mask.
- Another macro called "check" is created to test if certain patterns preclude guesses from being considered correct.

### Incremental Solver

- The solver only needs to solve incrementally for one step at a time based on previous guesses.
- If yellow is received four times for the same character, then its location can be determined.

## Pruning Logic

Section Overview: The speakers discuss whether or not pruning is necessary and if doing it incrementally is sufficient.

### Incremental Pruning

- One speaker suggests that incremental pruning may be insufficient.
- They agree that checking the whole history may be necessary, but it will be a bit sad.

### Eliminating Words

- A chat message suggests that scanning incrementally cannot determine if a letter appears in a certain position.
- Another speaker explains that eliminating words with each guess makes incremental scanning sufficient.

## Guessing Different Letters

Section Overview: The speakers discuss how the algorithm should handle guesses with completely different letters.

- If all letters are incorrect, the algorithm should allow a guess with only different letters.
- Shifting all letters by one should also be allowed.

## Disallowing Incorrect Patterns

Section Overview: The speakers discuss how the algorithm should disallow incorrect patterns.

- If a pattern indicates a letter is yellow, but it is guessed again and found to be incorrect, then it must be
  disallowed.
- There seems to be a bug in the algorithm where some correct patterns are being disallowed.

## Debugging the Code

Section Overview: In this section, the speaker is debugging the code and trying to figure out why certain inputs are not
working as expected.

### Debugging the Disallowed Inputs

- The input "rita bond a b c d e plus w w disallows b c d e a" is being disallowed.
  - 
- The speaker questions why this input should be disallowed since all letters are present but in different arrangements.
  - 
- Upon further inspection, it is discovered that all of the letters are grayed out, indicating that they are incorrect.
  - 

### Identifying the Issue with Early Returns

- The speaker examines early returns in the code to determine which one is being taken.
  - 
- It is determined that an incorrect return statement is being taken twice.
  - 
- Further investigation reveals that previously correct letters were being re-evaluated, causing errors in matching letters.
  - 

### Optimizing Code

- A suggestion is made to optimize the code by comparing candidate words to answers element by element for correctness.
  - 
- The speaker considers this proposal but ultimately decides against it due to excessive complexity.
  - 

## Refactoring Code for Optimization

Section Overview: In this section, the speaker discusses refactoring the code for optimization purposes.

### Simplifying Code with Zip Function

- The speaker simplifies some of the code by using Python's zip function instead of manually iterating through lists.
  - 

### Refactoring for Efficiency

- The speaker discusses refactoring parts of the code for efficiency, such as using sets instead of lists.
  - 

## Finalizing Code

Section Overview: In this section, the speaker finalizes the code and prepares to submit it.

### Testing Final Code

- The speaker tests the final version of the code with various inputs to ensure that it is working correctly.
  - 

### Submitting Code

- The speaker submits the final version of the code for review.
  - 

## Introduction

Section Overview: In this video, the speaker discusses how to create a program that can guess a word in a game of
Hangman. They explain the algorithm and the math behind it.

### Naive Algorithm

- The naive algorithm is to guess each letter in alphabetical order until you find the correct answer.
- This algorithm is not efficient and can take a long time to find the answer.

### Improved Algorithm

- The improved algorithm involves using information theory to make more informed guesses.
- The program starts by guessing the most common letters in English words, such as "e" or "a".
- It then uses the feedback from incorrect guesses to eliminate possibilities and make better guesses.
- This algorithm is much more efficient than the naive approach.

## Computing Correctness

Section Overview: In this section, the speaker explains how to compute correctness for each guess.

### Pruning Possibilities

- To compute correctness, we need to prune down the list of possible words based on previous guesses and their
  corresponding feedback.
- We can use set intersection to find words that match all previous guesses and set difference to remove words that do
  not match any previous guesses.

### Exploiting Equivalence

- The speaker discusses an insight where we can assume that if our current guess were actually the answer, then our
  previous guess should have given us the same pattern of correctnesses.
- By exploiting this equivalence, we can further prune down our list of possible words.

### Computing Goodness

- To compute goodness for each guess, we need to balance how likely it is to be correct with how much information we
  gain if it's incorrect.
- We use information theory and a formula involving probabilities and logarithms to compute goodness for each guess.

## Probability of Words

Section Overview: In this section, the speaker explains how to compute the probability of each word being the correct
answer.

### Total Count

- To compute the probability of a word, we need to know the total count of remaining possibilities.
- This involves considering all possible patterns and their corresponding counts.

### Probability Calculation

- The probability of a word being correct is based on its relative likelihood compared to other possible words.
- We also need to consider the possible patterns that match previous guesses and their feedback.

## Conclusion

Section Overview: In this section, the speaker concludes by summarizing the algorithm and its efficiency.

### Summary

- The improved algorithm for Hangman involves using information theory to make more informed guesses.
- It prunes down possibilities based on previous guesses and computes goodness for each guess using probabilities and
  logarithms.
- The program also considers the total count of remaining possibilities and their corresponding probabilities.

### Efficiency

- The improved algorithm is much more efficient than the naive approach, as it can quickly eliminate unlikely
  possibilities and make better guesses.

## Computing the Goodness of Words

Section Overview: In this section, the speaker discusses how to compute the goodness of words using a formula that
involves probabilities and patterns.

### Formula for Computing Goodness

- The formula for computing the goodness of a word is based on probabilities and patterns.
- To avoid losing precision, the computation should be done as floating point.
- The sum in the formula is over all possible patterns.

### Intuition Behind the Formula

- The goodness of a word is defined as the sum of the goodness for each possible pattern that might result from guessing
  it.
- The probability of a word is equal to the sum of probabilities for all possible patterns that we might see.

### Helper Function for Permutations

- A helper function can be used to generate permutations for all possible patterns.
- It's not necessary to exclude impossible patterns since they would have zero probability anyway.

## Permutations

Section Overview: In this section, the speaker discusses permutations and power sets. They mention wanting a power set
of a certain length and use the i product macro to achieve this.

### Using Cartesian Product for Power Sets

- The speaker mentions wanting a power set of a certain length.
- They use the i product macro to achieve this.
- The i product macro is actually the cartesian product.
- They use `cargo.toml` to get `itertools`.
- They then use `itertools.product()`.

## Computing Patterns

Section Overview: In this section, the speaker discusses computing patterns. They loop over all possible patterns and
compute what is left after each guess.

### Looping Over Possible Patterns

- The speaker loops over all possible patterns.
- They call these "correctness patterns".
- They filter on pattern using `filter()`.
- They filter on guess of word using `matches()`.

### Computing What Is Left After Each Guess

- For every pattern, they find all of the remaining words that would exist if they guessed that word and that pattern
  matched.
- Inside each item of this loop, they consider a world where they did guess word and got pattern as the match as the
  correctness.
- Then they compute what is left.

## Understanding the Code

Section Overview: In this section, the speaker is discussing a code and explaining how it works.

### Probability of Getting a Pattern

- The probability of getting a pattern is calculated by dividing the total count of words in that pattern by the total
  count of words outside that pattern. [](t=2:44:57s)
- The remaining count is divided by the remaining candidates. [](t=2:45:43s)

### Computing Entropy

- To compute entropy, we sum together all probabilities for each candidate word. [](t=2:46:36s)
- We use this to compute information amount in guessing a word. [](t=2:48:12s)

### Improving Code Efficiency

- The current computation is very slow and single-threaded. [](t=2:49:25s)
- Debugging shows that all values are NaN, which means there's an issue with the code. [](t=2:51:59s)
- Starting with goodness can help improve efficiency.[](t=2:51:43s)
  [_CUTOFF_LIMIT_]

## Generated by Video Highlight

https://videohighlight.com/video/summary/doFowk4xj7Q