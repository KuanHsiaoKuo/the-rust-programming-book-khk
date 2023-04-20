# Setting up CI and property testing for a Rust crate

<!--ts-->
<!--te-->

## [#](t=0:00:00) Introduction

Section Overview: In this stream, the presenter plans to take an arbitrary crate and look at what steps to take after
writing code that works. The focus is on polishing the crate by setting up CI, adding tests, creating a readme file,
structuring documentation, and getting coverage testing.

### Purpose of the Stream

- [](t=0:00:16) The goal of the stream is to go through things that are done when there's a crate with one commit and no
  documentation.
- [](t=0:02:25) The length of the video will be around two hours or shorter depending on how far they get.

## [#](t=0:03:07) Setting Up CI

Section Overview: This section covers setting up Continuous Integration (CI).

### Checking Out Crate

- [](t=0:03:07) The presenter has a checkout of the crate with only one diff patch made in the previous stream.
- [](t=0:03:24) They want to remove it from being a path because otherwise, CI won't run.

### Fixing Cargo Config File

- [](t=0:04:49) There were two cargo config files which were causing issues. They removed one.

### Building Crate

- [](t=0:05:05) Running `cargo check` builds successfully without any errors.

## [#](t=0:06:10) Adding Tests

Section Overview: This section covers adding tests to the crate.

### Creating Test Suite

- [](t=0:06.10) A test suite was created in the previous stream but wasn't advanced.

### Writing Tests for Functions

- [](t=0.06.50) Presenter writes tests for functions in their code.

### Running Tests

- [](t= 00m08s) Running `cargo test` runs all the tests successfully.

## [#](t=0:09:00) Creating a Readme File

Section Overview: This section covers creating a readme file for the crate.

### Importance of Readme

- [](t=0:09:00) A readme file is important because it's the first thing people see when they visit your repository.

### What to Include in Readme

- [](t=0:09:30) The presenter suggests including information about what the crate does, how to use it, and how to
  contribute.

### Writing Readme

- [](t=0:10.20) Presenter writes a readme file for their crate.

## [#](t=0:12.50) Structuring Documentation

Section Overview: This section covers structuring documentation for the crate.

### Importance of Documentation

- [](t=0:12.50) Good documentation helps users understand how to use your code and can save time in answering questions.

### Types of Documentation

- [](t=0:13.20) The presenter suggests having an overview, getting started guide, API reference, and examples.

### Using Rustdoc

- [](t= 00m14s) Rustdoc is used to generate documentation from comments in code.

## [#](t=0:16.40) Getting Coverage Testing

Section Overview: This section covers getting coverage testing for the crate.

### Importance of Coverage Testing

- [](t=0:16.40) Coverage testing helps ensure that all parts of your code are being tested.

### Using Tarpaulin Crate

- [](t= 00m17s) Tarpaulin is used as a coverage testing tool for Rust code.

### Running Tarpaulin on Codebase

- [](t= 00m18s) Running `cargo tarpaulin` generates a coverage report for the codebase.

## [#](t=0:20.00) Conclusion

Section Overview: The presenter concludes the stream by summarizing what was covered and thanking viewers.

### Summary

- [](t=0:20.00) The presenter covered setting up CI, adding tests, creating a readme file, structuring documentation,
  and getting coverage testing.

### Thanking Viewers

- [](t=0:21.10) Presenter thanks viewers for watching and answering questions in the chat.

## [0:05:24](t=324s) Cargo Versioning

Section Overview: In this section, the speaker explains the versioning system for the cargo crate and why it is a little
weird.

### Cargo Versioning

- The versioning for the cargo crate is based on the current Rust version.
- For Rust 1.x, the cargo version is 0.x plus 1.
- The reason for this system is unknown.

## [0:06:01](t=361s) Reference Point

Section Overview: In this section, the speaker suggests keeping information about cargo Tamil as a reference point.

### Reference Point

- It's suggested to keep information about cargo Tamil as a reference point.

## [0:06:06](t=366s) Committing Changes

Section Overview: In this section, the speaker discusses committing changes to fix an issue with patch overrides in
Cargo.

### Committing Changes

- To fix an issue with patch overrides in Cargo, it's suggested to turn it into 70 or use rev equals one commit.
- This will ensure that patches are picked up correctly.

## [0:06:32](t=392s) Skipping Duplicate Package Issue

Section Overview: In this section, the speaker talks about an issue with skipping duplicate packages when taking a git
dependency on Cargo.

### Skipping Duplicate Package Issue

- There seems to be an issue with skipping duplicate packages when taking a git dependency on Cargo.
- This needs further investigation.

## [0:07:10](t=430s) Building CI Pipeline

Section Overview: In this section, the speaker discusses building a CI pipeline and getting packages to build somewhere
other than their own laptop.

### Building CI Pipeline

- The first step in building a CI pipeline is ensuring that packages can build somewhere other than their own laptop.
- This was not possible before due to a patch path override.
- The speaker has a collection of CI scripts that they use and keep in a separate GitHub repo.
- Configurations for dependabot, code coverage, and workflows are kept in this repo.
- By adding another git remote to the current git repository and fetching it, the history of that other repo can be
  merged with the history of this repo.
- This makes it easy to keep the CI up to date.

## [0:08:50](t=530s) Workflows

Section Overview: In this section, the speaker talks about different workflows for different kinds of things that may or
may not be relevant for a given crate.

### Workflows

- Different workflows are used for different kinds of things that may or may not be relevant for a given crate.
- Examples include checks scheduled and test ammo.

## [#](t=0:11:10s) Notifications for GitHub Actions

Section Overview: The speaker discusses how they use notifications for GitHub actions to update outdated versions of
actions used in their CI.

### Using Versioned Actions

- [](t=0:11:10s) The speaker uses versioned actions in their CI.
- [](t=0:11:10s) If an outdated version of an action is being used, a PR is filed to update it.

### Cargo Ecosystem

- [](t=0:11:30s) The speaker has a daily job set up for the cargo ecosystem.
- [](t=0:11:30s) This job runs at the root of the crate and ignores all dependencies if the update is a patch or minor
  update.
- [](t=0:11:52s) If there's a major release, the speaker wants to know so they can take action and update their crates.

## [#](t=0:12:33s) Codecov Configuration

Section Overview: The speaker talks about configuring codecov for code coverage reporting.

### Configuring Code Coverage Reporting

- [](t=0:12:33s) The speaker configures codecov for code coverage reporting.
- [](t=0:12:54s) They ignore the test directory because they don't want coverage for tests.
- [](t=0:12.54s) They make the code comments on PRS and stuff less verbose.

## [#](t=0.13.10s) Real Jobs

Section Overview: The speaker goes through the actual jobs in their workflow.

### Check Workflow

- [](t=0.13.31s) The check workflow runs when pushing to main branch or creating a pull request.
- [](t=0.13.51s) It includes format, Clippy, dock, hack, and msrv jobs.
- [](t=0.14.13s) The format job runs cargo format check on the stable compiler.
- [](t=0.14.37s) The Clippy job runs both stable and beta versions.

### Fail Fast

- [](t=0:15:19s) The speaker sets fail fast for the Clippy job to avoid unnecessary CI runs.
- [](t=0:15:37s) If something fails in one step, it will likely fail in other steps as well.

## [#](t=0:16:12s) Cargo Check and Cargo Hack

Section Overview: In this section, the speaker talks about the importance of running `cargo doc` to check for correct
introdoc links. They also discuss using nightly documentation features with `cargo doc`. The speaker then explains how
to use `cargo hack` to check combinations of feature flags.

### Running cargo doc

- [](t=0:16:12s) When running `cargo doc`, checks are performed that don't run when you run any other command.
- [](t=0:16:33s) To check that your introdoc links are correct and not dangling, you need to run a `cargo doc`.
- [](t=0:16:52s) Running `cargo doc` on nightly allows you to use some nice features like config doc or doc config which
  means that you can mark a particular function or type as only being available if a certain configuration is true.

### Using cargo hack

- [](t=0:17:14s) Cargo requires that your features are all additive so that means your crate should compile with any
  combination of your features.
- [](t=0:17:37s) Cargo hack allows you to check a particular combination of features or a set of combinations of
  features.
- [](t=0:18:38s) Power set of the features is used in cargo hack which means every possible combination is checked.

## [#](t=0:19:02s) MSRV and Safety Checks

Section Overview: In this section, the speaker talks about minimum supported rust version (MSRV), what it is used for,
and how it works. They also discuss safety checks such as Miri, leak sanitizer, thread sanitizer, address sanitizer,
Loom and no_std.

### Minimum Supported Rust Version (MSRV)

- [](t=0:19:02s) MSRV is used to figure out whether the crate continues to build with the version of rust that we claim
  it builds with.
- [](t=0:19:26s) It checks out, installs and then runs cargo check against MSRV.

### Safety Checks

- [](t=0:20:09s) Miri, leak sanitizer, thread sanitizer, address sanitizer and Loom are safety checks that ensure your
  code is safe.
- [](t=0:20:33s) No_std makes sure that your crate still builds against targets that have no standard library and no
  allocator or targets that have an allocator but not a standard Library.

## Schedule Jobs

Section Overview: In this section, the speaker talks about the schedule jobs that are run on nightly and beta versions
of Rust.

### Running a Build on Nightly

- Run a build on nightly or at least fairly regularly.
- The job checks out installs nightly and runs cargo test with Dash locked.

### Building on Beta

- This job builds on beta and runs cargo update to ensure that if you run with newer versions of dependencies, your test
  suite still passes.
- Checking for new deprecated items in dependencies is important.

## Running Test Suite

Section Overview: In this section, the speaker talks about running the actual test suite.

### Testing Stable and Beta Versions

- Runs your test suite on stable and beta versions.
- The same little bit is used for both stable and beta versions.

### GitHub CI Magic

- If a file does not exist, then run cargo generate log file.
- Checking in lock files allows people to bisect your crate like they can check out an old version of your crate and it
  should still be able to build by virtue of using the exact dependencies that were present at the time.

### Minimal Versions

- Minimal only runs on stable but it happens to also install lightly because minimal versions is an unstable flag.
- It runs cargo update with the dash Z minimal versions flag which chooses the oldest version that's still subject to
  the dependency requirements that you set in your cargo Tamil.

## Minimal Versions

Section Overview: In this section, the speaker discusses the importance of using full version numbers when taking a
dependency to ensure that consumers don't end up in weird states. They also explain how minimal versions work and their
limitations.

### Importance of Full Version Numbers

- When taking a dependency, it is recommended to use the full version number instead of just the current major version.
- This ensures that consumers don't end up in weird states.
- It is better to look back at the version list and find exactly the minimal version needed.
- This may be time-consuming but it's better than picking the current version which has everything you're going to use.

### Limitations of Minimal Versions

- Minimal versions choose minimal versions transitively, which means if one of your dependencies has an incorrectly
  specified minimum version, it can cause problems.
- If a dependency needs a specific version but hasn't specified it themselves, we can force that minimal requirement by
  specifying it ourselves.
- There is another proposal called Dash Z direct minimal versions which chooses the minimal versions of your direct
  dependencies but the highest version of all transitive dependencies.

### OS Check and Coverage Testing

- The next step after resolving dependencies is OS check to ensure that crate test suite runs on Mac OS and Windows
  using stable compiler.
- Coverage testing is done using cargo lvm Cub which uses instrumented coverage for more accurate reporting.
- Codecov.io is used for outputting coverage results as it's free for open source projects.

## [#](t=0:31:52s) Oregon Origin and Git Merge

Section Overview: In this section, the speaker discusses merging a repository from Rusty Iconf into their own
repository. They discuss whether to preserve history or do a squash merge and why they chose to preserve history.

### Preserving History vs Squash Merge

- [](t=0:33:05s) The speaker chooses to preserve history for the merge because they plan on merging from Rusty Iconf
  again in the future.
- [](t=0:33:35s) If they did a squash merge, there would be more merge conflicts because git wouldn't know about the
  existing connections and merge history with that other repo.

### Checking CI and Pushing to Origin

- [](t=0:31:57s) The speaker checks CI before pushing it to origin.
- [](t=0:32:01s) They push CI to origin so that they can do a PR of this later on.

### Adding Cargo Lock

- [](t=0:34:17s) The speaker adds cargo lock by using "git add cargo lock" and "Ben dot get ignore".
- [](t=0:34:47s) They check in their cargo lock.

### Merging Repositories

- [](t=0:33:48s) After merging repositories, all commits from the other repo are brought in as well.
- [](t=0:32:33s) A PR is created from this one by adding CI and then putting the link there.

## [#](t=0:35.01s) Troubleshooting OpenSSL Sys Dependency Issue

Section Overview: In this section, the speaker troubleshoots an issue with OpenSSL Sys dependency version.

### Open SSL Sys Dependency Issue

- [](t=0.36.58s) Minimum supported rust version failed because of an issue with OpenSSL Sys dependency version.
- [](t=0.37.13s) The speaker goes to the GitHub repo for OpenSSL Sys and checks its cargo Tomo.
- [](t=0.37.36s) They find that OpenSSL Sys has moved away from using Rust C underscore version dependency and now uses
  Auto config.

### Bumping in MSRV for Crates Index

- [](t=0:35:24s) The speaker bumps in MSRV for crates index to fix the issue with minimum supported rust version
  failing.

## [#](t=0:32:33s) Checking CI and Running Tests

Section Overview: In this section, the speaker checks CI and runs tests to ensure everything is working properly.

### Checking CI

- [](t=0:31:57s) The speaker checks CI before pushing it to origin.
- [](t=0:32:01s) They push CI to origin so that they can do a PR of this later on.

### Running Tests

- [](t=0:32:33s) After merging repositories, all commits from the other repo are brought in as well.
- [](t=0:32:48s) Jobs are kicked off and it will be interesting to see if they pass or fail.
- [](t=0:34:36s) Minimum supported rust version failed but was fixed by bumping in MSRV for crates index.
- [](t=0.36.58s) Minimum versions failed but was not surprising since no new version was found for it.
- Other tests were run such as nightly dock works fine, cargo update works fine, etc.

## [t=0:38:39s] Solving Dependency Problems

Section Overview: In this section, the speaker discusses how to solve dependency problems in Rust.

### Bumping Minimal Versions

- The speaker's first instinct is to bump minimal versions.
- However, this can be annoying because it may not address the problem in the specification.
- Pinning a much newer version of OpenSSL can be a solution.

### Binary Search for OpenSSL Version

- The current version of OpenSSL being used is 0.10.45.
- A binary search is performed to find a newer version of OpenSSL that works.
- Version 0.8.5 is found to work.

### Updating Package Config

- Package config version 0.3.11 does not have the required method for range version.
- Updating package config to version 0.3.26 solves this issue.

### Fixing Rand Isaac and HKDF

- Rand Isaac version 0.1.1 is needed but was not specified in the code.
- HKDF through P384 has an issue with its dependencies.
- Finding compatible versions of these dependencies can be very tedious and annoying.

### Conclusion

Solving dependency problems in Rust can be challenging and time-consuming due to transitive dependencies and
compatibility issues between different versions of packages and libraries.

## Updating Dependencies

Section Overview: In this section, the speaker discusses updating dependencies and how to get rid of old versions of
packages.

### Getting Rid of Old Versions

- Crossbeam Channel can be used to update Ran.
- The old version of Rand is completely gone after using Crossbeam Channel.

### Testing New Versions

- Binary search can be used to test new versions.
- OpenSSL was tested and found to build successfully.

### Updating Cargo Lock

- Running `cargo check` updates the Cargo lock file.
- Approval rules are added before merging changes.

## Setting Up CI

Section Overview: In this section, the speaker sets up continuous integration (CI).

### Creating a Repository

- A repository is created for Code Curve.
- Workflows are run to ensure everything works as expected.

### Troubleshooting Nightly Build Failure

- Checking nightly builds reveals that the lock file needs to be updated.
- Multiple versions of FF are brought in due to specified minimal versions.

## [#](t=0:52:46s) Starting the CI

Section Overview: In this section, the speaker starts the Continuous Integration (CI) process for an open-source project
on GitHub. They explain how GitHub limits the number of jobs that can run in parallel for open-source projects and how
canceling previous runs can free up resources to run the current one faster.

### Starting CI

- The speaker starts the CI process for an open-source project on GitHub. [](t=0:52:50s)
- They mention that there were three rockets before, but now there are only two, which makes them sad. [](t=0:52:50s)
- The speaker wishes that all tests would run in parallel and fast. However, they acknowledge that GitHub is donating CI
  for free for open source things. [](t=0:53:08s)
- The speaker cancels some runs from a previous push to free up more resources to run this one faster or more parallel
  because GitHub limits the number of jobs that can run in parallel for open-source projects. [](t=0:54:12s)

## [#](t=0:55:06s) Value of Minimal Versions

Section Overview: In this section, the speaker discusses whether it's worth spending time on minimal versions and
checking them through CI.

### Minimal Versions

- The speaker explains why having a correct minimal version check through CI is valuable as it can save time down the
  road when consumers come with issues related to dependencies specified in cargo.toml file.[](t=0:55:22s)
- They also mention that catching errors in one place is better than many places and building tooling could help reduce
  waste of time required to get to that point.[](t=0:55:58s)

## [#](t=0:56:09s) Streaming as Educational Content

Section Overview: In this section, the speaker discusses the value of streaming as educational content.

### Streaming

- The speaker explains that streaming can be valuable if it's educational and useful. They mention that they try to
  build things that are useful and hope that their streams are educational.[](t=0:57:31s)
- They also mention that some streams may not have any value, but others like Wordle stream were just fun to
  build.[](t=0:57:12s)

## [#](t=0:58:11s) Introduction

Section Overview: In this section, the speaker talks about the purpose of their streams and hopes that they are not a
waste of time for viewers. They also discuss the value of their time compared to others.

### Purpose of Streams

- The speaker hopes that their streams have a meaningful impact on viewers.
- They hope that viewers learn new things during the stream.
- The trade-off between the speaker's time and viewer's time is reasonable.

## [#](t=0:59:03s) Technical Issue with Windows

Section Overview: In this section, the speaker encounters a technical issue with Windows and discusses how to fix it.

### Fixing OpenSSL on Windows

- The speaker encounters an issue with OpenSSL on Windows.
- They search for a solution in their repositories.
- To fix it, they need to install OpenSSL specifically for Windows.
- This step should be added to their standard setup process.

## [#](t=1:00:07s) Testing and CI

Section Overview: In this section, the speaker discusses testing and continuous integration (CI).

### Coverage Report

- The coverage report shows that there is terrible testing of index.
- There is no test for features or dependencies in index.

### Pushing Changes and Merging Branches

- The speaker pushes changes and merges branches.
- They confirm merge and delete branch afterward.

### Updating Readme File

- The readme file needs updating after merging branches.
- Badges need to be updated as well.

## Improving Test Suite with Property-Based Testing

Section Overview: In this section, the speaker discusses the benefits of property-based testing and compares two main
tools for it: Prop test and Quick Check.

### Property-Based Testing

- Property-based testing is a way to write tests by defining patterns for test cases instead of writing individual test
  cases.
- Two main tools for property-based testing are Prop test and Quick Check. The recommendation is to generally favor Prop
  test because it has a more elaborate way of doing property-based testing.
- Property-based testing generates different inputs based on defined constraints to explore the space more efficiently
  than traditional fuzz testing.
- Property-based testing is less deterministic than traditional fuzz testing due to exploring an infinite or very large
  space. However, there are mechanisms in place to make it more deterministic.

### Example

- An example of using property-based testing is checking that a parse date function doesn't panic when given any string
  that matches a regular expression pattern.
- Another example is checking that all strings matching a pattern should be valid dates or at least parsable as dates.

Overall, property-based testing provides an efficient way to generate inputs based on defined constraints and explore
the space intelligently. It can be used as an alternative or complement to traditional fuzz testing.

## [t=4203s] Generating Inputs for Testing

Section Overview: In this section, the speaker discusses how to generate inputs for testing and the benefits of
generating inputs from small domains. The speaker also explains how to use prop testing to test if things work
correctly.

### Benefits of Generating Inputs from Small Domains

- Generating inputs from relatively small domains allows you to fully explore the space and do actual ranges.
- It generates a bunch of valid potential inputs that semi-intelligently explore the space.
- Prop testing runs a bunch of inputs and tells you what input it might fail for.

### Using Prop Testing

- It is not recommended to use prop testing to test performance because it randomly explores the input space.
- Benchmarking based on prop testing can be weird because you may end up benchmarking the harness just as much as you're
  going to test the actual online code.
- Prop testing generates a bunch of things that match a pattern and tries to be more intelligent than doing a linear
  scan.
- Ultimately, it refines down to a test case that fails.

## [t=4360s] Adding Dependencies with Prop Tests

Section Overview: In this section, the speaker talks about adding dependencies with prop tests and modifying cargo
projects.

### Round Trip Function

- The round trip function constructs a cargo project, expects modifications in cargo Tamil, packages it up, and runs it
  through all different steps.
- Currently, round trips are super simple; one does not modify anything while another was not implemented yet.

### Adding Dependencies with Prop Tests

- To add prop tests, we need to add dependencies first.
- The speaker wonders whether they want this in the same file or not.
- They make some changes but encounter an error.

## Interesting

Section Overview: In this section, the speaker expresses interest in something.

### Speaker's Interest

- The speaker says "interesting."

## Checking Out Cargo Lock

Section Overview: In this section, the speaker mentions checking out cargo lock.

### Checking Out Cargo Lock

- The speaker says "check out cargo lock."

## Huh?

Section Overview: In this section, the speaker expresses confusion.

### Confusion

- The speaker says "huh?"

## Error Message Explanation

Section Overview: In this section, the speaker tries to understand an error message.

### Error Message Explanation

- The speaker wonders why there is an error message.
- They speculate that it might be because they added prop test um but it's not running with minimal versions.
- They say that shouldn't make a difference.

## Build Issue

Section Overview: In this section, the speaker encounters a build issue.

### Build Issue

- The speaker says that it doesn't even build with the latest versions of everything.
- They mention finding crate bytes compiled by an incompatible version of rust compiled to their Rusty beta.

## Interesting Again!

Section Overview: In this section, the speaker expresses interest again.

### Interest

- The speaker says "interesting" again.

## Environment Variable Issue

Section Overview: In this section, the speaker realizes there is an issue with their environment variable.

### Environment Variable Issue

- The speaker realizes that their environment variable is no longer being set.
- They mention that it's causing issues.

## Avoiding Cargo Clean

Section Overview: In this section, the speaker tries to avoid running cargo clean.

### Avoiding Cargo Clean

- The speaker says they specifically didn't want to run cargo clean because of a weird Target directory setup.
- They mention that it wouldn't have made a difference anyway.

## Successful Build

Section Overview: In this section, the speaker successfully builds something.

### Successful Build

- The speaker says that it looks like it builds and asks the listener to check out cargo lock and cargo check.

## Strange Behavior

Section Overview: In this section, the speaker encounters strange behavior.

### Strange Behavior

- The speaker says "that's very strange."
- They wonder why they got into some weird fingerprinting issue here.

## Via Cargo as Standard Integration Test

Section Overview: In this section, the speaker discusses using Via cargo as a standard integration test.

### Using Via Cargo as Standard Integration Test

- The speaker wants via cargo to be a standard integration test file where they have imperative tests and prop tests
  separate.
- They mention wanting the round trip function to be usable in both of these tests but are unsure if it will work
  without running them twice.

## Running Tests Twice

Section Overview: In this section, the speaker discusses running tests twice.

### Running Tests Twice

- The speaker says that they think cargo test is actually now going to run the unit testing via cargo twice.
- They mention other ways to do this but find the auto-detection mechanism of cargo for test suites annoying.

## Dependencies

Section Overview: In this section, the speaker mentions dependencies.

### Dependencies

- The speaker comments on how many dependencies there are.
- They realize that they changed their cargo config and it caused confusion when trying to reuse old artifacts.

## Output Issue

Section Overview: In this section, the speaker discusses an issue with output.

### Output Issue

- The speaker realizes that it prints the output from cargo as a part of a test and mentions that it's another thing
  they need to fix.

## Cargo Test Suite

Section Overview: In this section, the speaker discusses the Rust test suite and what it uses from the cargo. They also
discuss modifying the workspace before packaging and checking output structs.

### Modifying Workspace Before Packaging

- The speaker suggests starting with one of the examples given in Via Cargo.
- They suggest stealing a whole test from Via Cargo.
- The goal is to check various transit structs.

### Prop Testing Over Dependencies

- The speaker suggests prop testing over dependencies.
- They want to ensure that when dependencies are added, appropriate version specifier optionality is passed through.
- Prop testing over whether it's dependencies Dev dependencies or build dependencies seems reasonable.
- A vector of dependencies is needed for this.

### Generating Dependencies

- The inputs required for generating dependencies are discussed.
- A function that generates dependencies is needed.
- It's suggested to generate a structured representation and then turn it into a string.
- Struct dependency has kind, name, version, and features.
- Optionality is also included in dependency struct.

## Generating Arbitrary Dependencies

Section Overview: In this section, the speaker discusses how to generate arbitrary dependencies and the challenges
associated with it.

### Deriving Arbitrary Trait

- The `arbitrary` trait is used to generate an arbitrary version of a struct.
- It can be used in Quick Check.
- The `prop` macro can derive it, but it's experimental.
- Deriving arbitrary requires taking a dev dependency on arbitrary for the crate, which doesn't feel right.

### Implementing Arbitrary Dependency

- Instead of deriving arbitrary, we need to implement an `ARB dep` strategy for generating a dependency.
- We want to generate a normalized manifest for the dependency.
- We need to use `prop compose` for generating recursive things.
- An arbitrary depth kind is going to be any in kind in zero u8 to 3.
- We have a way to generate an arbitrary dependency kind using `prop compose`.

## Generating Dependency Specifiers

Section Overview: In this section, the speaker discusses how to generate dependency specifiers and names for
dependencies.

### Generating a Version Specifier

- To generate a version specifier, use a regular expression for semantic versioning specifiers.
- The regex can include a carrot or an equals sign (optional), followed by zero to nine numbers (at least one), followed
  by an actual dot, followed by zero to nine numbers (at least one).
- Build specifiers can also be included with a dash followed by arbitrary ASCII characters (limited to ASCII).

### Generating a Name

- The name of the dependency is separate from the dependency specifier and needs to be generated separately.
- The name can be any string consisting of lowercase letters, numbers 0-9, dashes, and underscores.
- Capital letters are not allowed, but it must start with a letter.
- It's allowed to be a single character.

### Additional Considerations

- When generating arbitrary depth, there needs to be a separate name specified for each dependency.
- If the name is equal to the listing, it changes the logic and needs to be encoded differently.

## Creating a Package

Section Overview: In this section, the speaker discusses creating a package and setting its features.

### Setting Features

- The speaker mentions that there must be an easier way to set features than using any bull.
- They suggest using default features instead.
- The speaker sets the public to true and optional to option of this.
- They set the package to none.

### Defining Package

- The speaker defines the package as arbitrary and says it will take an option string.

## Testing Package

Section Overview: In this section, the speaker discusses testing the created package.

### Testing with Different Packages

- The speaker tests both cases where the package is set to something different from the name of the crate and when it's
  not set at all.

### Errors in Testing

- There is an error message about expecting a closure that returns dependency string but it returns result.
- The speaker realizes they can only use prop assume inside of prop tests.

## Arbitrary Depth

Section Overview: In this section, the speaker discusses defining arbitrary depth for ARB depths.

### Defining ARB Depths

- The speaker defines ARB Depp zero to some number.
- They map over it and turn it into a string using format.

## Creating a String from Depth

Section Overview: In this section, the speaker discusses creating a string from depth.

### Creating a String from Depth

- The function returns a string. [](t=1:53:02)
- The speaker wants to create something like map. [](t=1:53:24)
- Vex strategy is not an iterator prop. [](t=1:53:29)
- ER returns string. [](t=1:53:48)
- The speaker suggests using stir takes a depth which is going to be any depth and returns a string and it just formats
  that.[](t=1:54:21)

## Writing Out Other Props

Section Overview: In this section, the speaker discusses writing out other props.

### Writing Out Other Props

- The speaker wants to write out other props including getting comma right.[](t=1:58:10)
- They read in the existing Tamil and push a new line.[](t=1:58:46)
- They use standard format right as well and then they'll do stud right to P dot join cargo Tamil and they'll write back
  out the modify Tamil.[](t=1:59:35)

## Prop Tests

Section Overview: In this section, the speaker discusses prop tests.

### Prop Tests

- There's also a warning saying on line 69 that we don't unwrap this and for our own sanity print out what it actually
  does so what is the list of depths that it's trying to inject?[](t=2:00:42)
- It doesn't find libar as our main RS oh actually I wonder whether the prop tests are supposed to not be integration
  tests but actually be test binaries uh oh I guess it is [Music].[](t=2:00:52)

## [#](t=2:01:35s) Understanding Dependency Specifiers

Section Overview: In this section, the speaker discusses dependency specifiers and how to generate them. They also
encounter errors with regular expressions and work to fix them.

### Generating Dependency Specifiers

- [](t=2:04:24s) The speaker generates a list of dependencies, including their versions.
- [](t=2:05:15s) Regular expressions are used to ensure that version numbers are valid.
- [](t=2:06:25s) The speaker notes that they are not currently writing out all properties for the dependencies.

### Writing Out Dependency Properties

- [](t=2:06:57s) The speaker writes out optional, default features, and package properties for the dependencies.
- [](t=2:07:53s) A dependency specifier is generated with version, optional, and default features properties.

### Further Customization of Dependency Specifiers

- [](t=2:08:03s) The speaker notes that dependency specifiers can be further customized with less than or greater than
  symbols.

## [#](t=2:08:51s) Testing Dependencies

Section Overview: In this section, the speaker discusses testing dependencies and ensuring that the final dependencies
are present in the input dependencies.

### Checking for Dependency Presence

- [](t=2:09:29s) Loop over the dependencies listed in the index and check that each one is present in the input.
- [](t=2:11:11s) Check that the dependency that ended up in the final index entry is actually one of the ones given in
  the input.
- [](t=2:14:22s) Assert that all of the input dependencies were found in the output dependencies.

### Helper Functions

- [](t=2:12:33s) Create a helper function to generate a vector of dependencies instead of strings.
- [](t=2:13:43s) Create another helper function to check if a field zero on string is equal to another field zero on
  string.

### Other Notes

- [](t=2:10:35s) The regex for minor versions is wrong and should be 1 to 9 or 0 or 2.
- [](t=2:15:05s) The assertion here is that we actually found all of the input dependencies in the output dependencies,
  but we're not checking if they are correct beyond just their name being present.

## Understanding Dependency Transformation

Section Overview: In this section, the speaker discusses the final depth of type index and its dependencies. They also
talk about various checks that need to be added for completeness.

### Index Dependencies

- The final depth is of type index and has two dependencies: registry dependency and a registered dependency.
- A registered dependency has a name and requirements.

### Checks for Completeness

- Various checks need to be added for completeness, such as checking if optional matches, default features made it all
  through, and package made it all through.
- Optional in the index is not optional because it defaults to false.
- Default features defaults to true in the package.
- Derefence both of them so they are matchable.
- Convert cow string to rep so they are comparable using dot map s.

### Limitations

- This will break if it generates two dependencies by the same name, which hasn't been addressed yet.

## Running Prop Test Job

Section Overview: In this section, the speaker talks about running prop test job separately from other tests since it
takes longer to run.

### Ignoring Slow Tests

- The prop test job is slow and should be ignored during normal testing.
- It should only run on stable tool chain.
- We want a separate job that runs prop test only if basic test suite succeeds.

### Configuring Prop Test

- Prop tests are slow; 256 executions have to execute for a Tesla so hold to pass now 256 for us is actually going to
  take a while like you saw how slowly they executed but I think it's probably okay I don't worry too much about this um
  I think we can also skip this print.

## Practice Regressions

Section Overview: The speaker discusses the importance of checking in a file that replicates over time and recommends
doing so. They also mention running prop tests explicitly.

### Checking in Files

- It is recommended to check in files that replicate over time.
- The speaker suggests not checking in the initial file but doing so over time.
- Checking in files can be useful for running prop tests.

### Running Prop Tests

- Prop tests only generate files on failure.
- To run prop tests, use git add Dot commit and git push.

### Opening a Pull Request

- After pushing changes, open a pull request to initiate CI rules.
- CI rules will kick off different workflows, including basic testing and prop testing.

### Configuring Branch Protection

- Configure branch protection by requiring a pull request.
- Require status checks to pass, including stable formatting and Ubuntu stable test suite.
- Do not require minimum supported rust version or nightly/beta versions.

## Improving Build Times with Caching

Section Overview: In this section, the speaker discusses how caching can be used to improve build times and reduce the
time it takes to run tests.

### Using Caching for Dependency Trees

- A chunky dependency tree can take a long time to build, so copying binary bits using caching may be faster.

### Defining Cache Keys

- Defining cache keys is necessary when building with minimum supported Rust versions (MSRV).
- The cache key should be set up as follows: `Target cash restore`, `Target`, and `key = matrix.tool chain`.

### Saving and Restoring Cache

- After running cargo tests, save the cache by running `steps.dot Target cash restore outputs`.
- To restore the cache for minimal versions, use `stable Target`.

### Prop Tests

- Prop tests take a long time to run because they explore a large input space.
- The prop test only took 51 seconds to run despite exploring 256 different combinations of feature combinations.

## Conclusion

Section Overview: In this section, the speaker concludes by noting that more work needs to be done in taking advantage
of dependency kinds.

### Taking Advantage of Dependency Kinds

- More work needs to be done in taking advantage of dependency kinds.

## Configuring CI for Prop Tests

Section Overview: In this section, the speaker discusses configuring CI for prop tests and the trade-offs involved.

### Trade-Offs of Adding Instrumentation for Coverage

- Adding instrumentation for coverage executes more slowly.
- It is unclear whether it's worth the trade-off to have prop tests run more slowly to get coverage.

### Documentation

- The topic of documentation is not discussed in detail but may be a topic for another day.

### Follow-Up Questions

- The speaker asks if there are any follow-up questions from what was covered today.
- A question is asked about accessing the restore cache correctly and referencing that key from different steps. The
  speaker believes it is allowed because one of the points of the cache is that it is cross-job and possibly even
  cross-workflow.

## Saving Target File and Restoring Cache

Section Overview: In this section, the speaker discusses saving target files and restoring cache during CI
configuration.

### Saving Target File

- The speaker discusses how saving target files works during CI configuration.

### Restoring Cache

- The speaker explains how cache restoration works during CI configuration.
- The cache was restored quickly after being downloaded.

## Updating Git Repository

Section Overview: In this section, the speaker talks about updating Git repositories during CI configuration.

### Pinning Revision in Cargo Tamil

- Speaker mentions that they pinned revision in cargo Tamil so git repository shouldn't need to be updated.

## Sparse Registries

Section Overview: In this section, the speaker discusses sparse registries and how they can be used to make CI faster.

### Injecting Configuration

- The speaker mentions that they could inject configuration to make CI faster but it's about to land anyway.
  [_CUTOFF_LIMIT_]

## Generated by Video Highlight

https://videohighlight.com/video/summary/xUH-4y92jPg