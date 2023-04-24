## From cargo to crates.io and back again(3/7h)

<!--ts-->
   * [From cargo to crates.io and back again(3/7h)](#from-cargo-to-cratesio-and-back-again37h)
   * [Introduction](#introduction)
   * [Cargo and Registries](#cargo-and-registries)
   * [Implementing the Loop](#implementing-the-loop)
   * [Cargo Package](#cargo-package)
   * [Publishing on Crates.io](#publishing-on-cratesio)
   * [Cargo Package and Crate Files](#cargo-package-and-crate-files)
      * [What Happens When You Run cargo package](#what-happens-when-you-run-cargo-package)
      * [Viewing Contents of a .crate File](#viewing-contents-of-a-crate-file)
      * [Body Data Sent by Cargo](#body-data-sent-by-cargo)
      * [Metadata in Crate API](#metadata-in-crate-api)
      * [Validating Crates](#validating-crates)
      * [Crates.io Crate Type Definitions](#cratesio-crate-type-definitions)
      * [Standalone Crate Type Definitions](#standalone-crate-type-definitions)
      * [Uploading JSON to Server](#uploading-json-to-server)
      * [Server Response](#server-response)
   * [Understanding the Git Pull from the Skit Index](#understanding-the-git-pull-from-the-skit-index)
      * [The Skit Index](#the-skit-index)
      * [Inside the Index Files](#inside-the-index-files)
   * [Transitioning to HTTP-based Sparse Registries](#transitioning-to-http-based-sparse-registries)
      * [HTTP-Based Sparse Registries](#http-based-sparse-registries)
   * [Understanding the Cargo Registry](#understanding-the-cargo-registry)
      * [The Role of Indexes in Hosting Versions and Crate Files](#the-role-of-indexes-in-hosting-versions-and-crate-files)
   * [Crates Index](#crates-index)
      * [Understanding Crates as an Abstract Concept](#understanding-crates-as-an-abstract-concept)
      * [Special Implementations Used by Cargo](#special-implementations-used-by-cargo)
   * [Digging into Code on Cargo Side and Crates.io Side](#digging-into-code-on-cargo-side-and-cratesio-side)
      * [Understanding the Life Cycle from Publish to Consume](#understanding-the-life-cycle-from-publish-to-consume)
   * [Introduction](#introduction-1)
      * [Jason and Stream Dependency Resolution](#jason-and-stream-dependency-resolution)
   * [Publishing Crate Files](#publishing-crate-files)
      * [Including Dependencies Information](#including-dependencies-information)
   * [Hosting Index on GitHub](#hosting-index-on-github)
      * [Git vs HTTP-based API](#git-vs-http-based-api)
   * [Overview of Git Index and Cargo](#overview-of-git-index-and-cargo)
      * [Canonical Path for Git Index](#canonical-path-for-git-index)
      * [Structure of Git Repository Managed by Cargo](#structure-of-git-repository-managed-by-cargo)
      * [Logic Around Publish Command](#logic-around-publish-command)
   * [Code Structure of Cargo](#code-structure-of-cargo)
      * [Source Directory](#source-directory)
      * [Publish Command Code](#publish-command-code)
      * [Ops Module](#ops-module)
   * [Understanding the Cargo Publish Process](#understanding-the-cargo-publish-process)
      * [The Definition of Publish](#the-definition-of-publish)
      * [Package One](#package-one)
      * [Transmitting Payload](#transmitting-payload)
      * [Waiting for Crate Availability](#waiting-for-crate-availability)
   * [Introduction](#introduction-2)
      * [Chat Block Change](#chat-block-change)
      * [Crates IO Side](#crates-io-side)
   * [Definitions](#definitions)
      * [Cargo Registry Index](#cargo-registry-index)
   * [Custom Registries](#custom-registries)
      * [Support for Custom Registries](#support-for-custom-registries)
   * [Implementing a Cargo Registry](#implementing-a-cargo-registry)
      * [Challenges with Alternate Registries](#challenges-with-alternate-registries)
      * [Alternative Registries](#alternative-registries)
      * [Naming the Library](#naming-the-library)
   * [Naming the Cargo Index Interface](#naming-the-cargo-index-interface)
   * [Adding Features to Cargo Home](#adding-features-to-cargo-home)
   * [Defining Publish Phases](#defining-publish-phases)
      * [Crate.io Definition](#crateio-definition)
      * [Index Definition](#index-definition)
   * [Rust Crate Serialization](#rust-crate-serialization)
      * [Creating a Foundational Crate](#creating-a-foundational-crate)
      * [Cargo vs Crates.IO](#cargo-vs-cratesio)
      * [Index Entries](#index-entries)
   * [Summary New Function](#summary-new-function)
      * [Finding Summary New](#finding-summary-new)
      * [Intern String and Generic Types](#intern-string-and-generic-types)
      * [Registry Dependency and Cow Types](#registry-dependency-and-cow-types)
   * [Add Crate Job](#add-crate-job)
      * [Perform Index Add Crate](#perform-index-add-crate)
      * [Search-Based Definitions](#search-based-definitions)
   * [Dependency Ordering and Cargo Manifests](#dependency-ordering-and-cargo-manifests)
      * [Dependency Ordering](#dependency-ordering)
      * [Definition of Dependency Kind](#definition-of-dependency-kind)
      * [Cargo Manifest Format](#cargo-manifest-format)
      * [Tomo Manifest Type](#tomo-manifest-type)
   * [Simplifying the Toml Manifest](#simplifying-the-toml-manifest)
      * [Removing Unnecessary Fields](#removing-unnecessary-fields)
   * [Verifying Tarball](#verifying-tarball)
      * [Checking Dependencies](#checking-dependencies)
   * [Cargo Tamil Manifest Definition](#cargo-tamil-manifest-definition)
      * [Components of a Cargo Tamil Manifest](#components-of-a-cargo-tamil-manifest)
   * [Mapping Dependencies](#mapping-dependencies)
      * [Mapping Dependencies](#mapping-dependencies-1)
   * [Cleaning up the Code](#cleaning-up-the-code)
      * [Removing Unnecessary Types](#removing-unnecessary-types)
      * [Pruning Down Definitions](#pruning-down-definitions)
   * [Understanding the Cargo Manifest](#understanding-the-cargo-manifest)
      * [Fields in the Manifest](#fields-in-the-manifest)
      * [Public/Private Dependencies](#publicprivate-dependencies)
   * [Parsing TOML Manifests](#parsing-toml-manifests)
      * [Removing Unnecessary Fields](#removing-unnecessary-fields-1)
      * [Parsing TOML Manifests](#parsing-toml-manifests-1)
   * [Understanding the Version Encoding](#understanding-the-version-encoding)
      * [Version Encoding](#version-encoding)
   * [Parsing Metadata](#parsing-metadata)
      * [Cargo Metadata](#cargo-metadata)
   * [Tidying up Definitions](#tidying-up-definitions)
      * [Identifying Interned Strings](#identifying-interned-strings)
      * [Type Smolster](#type-smolster)
      * [Cleaning Up Definitions](#cleaning-up-definitions)
   * [Introduction](#introduction-3)
   * [Cargo Tamil](#cargo-tamil)
   * [Type Definitions](#type-definitions)
   * [Future Difficulty](#future-difficulty)
   * [Optimization](#optimization)
   * [Cargo Intern String Usage](#cargo-intern-string-usage)
      * [Intern String Usage](#intern-string-usage)
   * [Normalized Manifest Conversion](#normalized-manifest-conversion)
      * [Owned Copy](#owned-copy)
   * [Package Ownership Conversion](#package-ownership-conversion)
      * [Package Ownership Conversion](#package-ownership-conversion-1)
      * [Parsing User-Written Code](#parsing-user-written-code)
      * [Handling Different Types of Dependencies](#handling-different-types-of-dependencies)
      * [Mapping Cow into Owned Cow](#mapping-cow-into-owned-cow)
      * [Removing Cargo Features](#removing-cargo-features)
      * [Keeping Package](#keeping-package)
      * [Custom Implementation of Deserialize](#custom-implementation-of-deserialize)
   * [Generated by Video Highlight](#generated-by-video-highlight)

<!-- Created by https://github.com/ekalinin/github-markdown-toc -->
<!-- Added by: runner, at: Mon Apr 24 15:56:27 UTC 2023 -->

<!--te-->

## Introduction

In this section, the speaker introduces the topic of the stream and explains that they will be tackling an
implementation problem related to how cargo talks to registries.

- The speaker mentions that they will not be porting anything in this stream and will instead write a Rust program from
  scratch.
- They explain that they will focus on how cargo talks to registries, including crates.io and alternative registries.
- The goal is to implement a crate that can be used by both cargo and crates.io, as well as other registries.

## Cargo and Registries

In this section, the speaker discusses what happens when you run `cargo publish` and how it interacts with registries
like crates.io.

- When you run `cargo publish`, two things happen: `cargo package` is run first, followed by uploading the package to a
  specific endpoint at crates.io.
- `cargo package` takes your entire source directory (excluding files in `.gitignore`) and creates a tarball containing
  all necessary files for distribution.
- The speaker notes that there are different data structures involved in these steps, but they are currently defined in
  different repositories without sharing logic or definitions.
- This lack of integration means there is potential for mismatches between them. For example, crates.io does not check
  if metadata sent by cargo matches the file uploaded during publishing.
- Ideally, one crate could be created that could be used by both cargo and crates.io.

## Implementing the Loop

In this section, the speaker explains their plan to implement a crate that can be used by both cargo and crates.io.

- The goal is to create a single crate that contains all necessary data structures for interacting with registries like
  crates.io.
- This would allow for better integration between different parts of the system and prevent potential mismatches.
- The speaker notes that they will not be implementing low-level logic for issuing HTTP requests, but rather focusing on
  data structures and their conversion between steps.
- They explain that this would allow crates.io to rerun cargo logic if necessary, such as when backfilling information
  for older packages.

## Cargo Package

In this section, the speaker discusses what happens when you run `cargo package`.

- `cargo package` takes your entire source directory (excluding files in `.gitignore`) and creates a tarball containing
  all necessary files for distribution.
- The tarball includes everything in your cargo.toml include directive, except things in exclude. By default, it also
  includes everything next to Cargo.toml except things in .gitignore.
- The speaker notes that the default rules can be weird.

## Publishing on Crates.io

In this section, the speaker discusses publishing on crates.io.

- When you run `cargo publish`, two things happen: `cargo package` is run first, followed by uploading the package to a
  specific endpoint at crates.io.
- The cargo metadata sent during publishing includes information like the name and version of the crate. This is also
  contained in the file uploaded by cargo.
- Currently, crates.io does not check if these two match. This could lead to issues with basic sanity checking or
  backfilling information for older packages.

## Cargo Package and Crate Files

In this section, the speaker explains what happens when you run `cargo package` and how it generates a `.crate` file.

### What Happens When You Run `cargo package`

- Running `cargo package` downloads dependencies from crates.io and generates a `.crate` file.
- The `.crate` file is essentially a `.tar.gz` archive that contains all the files needed for the crate to be built.
- The `.crate` file also contains metadata about the context in which the publish happened, such as the commit hash of
  the code at that time.
- The `Cargo.toml.orig` file is automatically generated by Cargo and is essentially a normalized version
  of `Cargo.toml`, removing features not used by the crate to ensure compatibility with older versions of Cargo.

### Viewing Contents of a `.crate` File

- To view the contents of a `.crate` file, navigate to its location (usually `/target/package/name-version.crate`) and
  run `tar tzf name-version.crate`.
- Examining the contents can help identify unnecessary files that can be excluded to make the crate smaller and faster
  to publish.

## [#](t=0:11:20s) Overview of the Crate API

This section provides an overview of the body of data sent by Cargo, which is essentially a manually encoded multi-part
HTTP message. It includes an integer of the length of the JSON data metadata of the package as a JSON object, then an
integer of the length of the crate file and then the crate file.

### Body Data Sent by Cargo

- The body of data sent by Cargo is an integer representing the length of:
    - The JSON data metadata for the package as a JSON object
    - The crate file
- The body is essentially a manually encoded multi-part HTTP message.

[](t=0:12:10s)

### Metadata in Crate API

- The metadata in Crate API includes:
    - Name
    - Version
    - Array of direct dependencies
    - Various information about those dependencies such as features and authors.
- This information can be obtained from cargo.toml in the crate file.

[](t=0:13:07s)

### Validating Crates

- The server should validate crates because it cannot trust that the JSON matches what's in the crate file.
- Parsing cargo.toml is necessary to validate crates.

## [#](t=0:14:12s) Existing Crate Type Definitions

This section discusses existing crate type definitions and their usage.

### Crates.io Crate Type Definitions

- There is a crate called "crates.io" that provides definitions for each API type.
- It is intended for those who want to look like crates.io or want to talk to crates.io.
- It has other stuff included like curl and URL because it also implements talking to parts which creates.io doesn't
  need.

[](t=0:15:08s)

### Standalone Crate Type Definitions

- A new standalone crate will be built with the same type definitions as the crates.io crate.
- The new crate will not include unnecessary dependencies like curl and URL.
- The new crate may take a dependency on the crates.io crate.

## [#](t=0:15:49s) Cargo Publish

This section discusses what happens when cargo publish is run.

### Uploading JSON to Server

- Running "cargo publish" runs a "curl put" command which uploads the JSON to the server.

### Server Response

- The response from the server depends on the implementation of the registry.
- On crates.io, it goes into a database and git index.

## Understanding the Git Pull from the Skit Index

This section explains what happens during a git pull from the skit index and how it updates files.

### The Skit Index

- When you run `cargo fetch` or update the index, it does a git pull from the skit index.
- Commits of updating crates are endless and trigger when someone runs `cargo publish`.
- Each version is one line in this file, and each file path has a specific syntax based on crate name length.

### Inside the Index Files

- Inside of the index files is one line per version, which is a JSON object that looks similar to what cargo sends to
  publish.
- One field that's here but not in publish is checksum, which is a hash of the .crate file.
- Going from just published JSON to this isn't possible without also having the .crate file.

## Transitioning to HTTP-based Sparse Registries

This section discusses transitioning to HTTP-based sparse registries and how they differ from git registries.

### HTTP-Based Sparse Registries

- With HTTP-based sparse registries, there's no more updating indexes or resolving deltas.
- Cargo is transitioning to using HTTP-based sparse registries.

## Understanding the Cargo Registry

This section explains how the Cargo registry works and its role in hosting a list of versions and crate files.

### The Role of Indexes in Hosting Versions and Crate Files

- The indexes are responsible for hosting a list of versions and dot crate files that have checksums matching the
  entries.
- When cargo talks to a registry, it mainly constructs a list of available versions by parsing each line of the file,
  then runs the resolver to figure out which version among these should be chosen based on the dependency declaration in
  your cargo toml.
- Having dependencies listed in the index allows for faster full resolve of dependencies without downloading or
  extracting any crate files.

## Crates Index

This section discusses crates index, which is similar to crates IO but has more features than just data definitions.

### Understanding Crates as an Abstract Concept

- A crate is an abstract concept that maps directly to one of the files in the index and has no information except for a
  list of versions.
- Every version has fields such as name, version, dependencies, checksum, features, links, yanked status and download
  URL.

### Special Implementations Used by Cargo

- Cargo uses special implementations or types like intern string or small string to avoid overhead when parsing fields
  for every dependency version.
- To be used by cargo, our library would have to be generic over string type so that cargo can choose its own optimized
  string types rather than being forced to use strings.

## Digging into Code on Cargo Side and Crates.io Side

This section explores the code on the cargo side and crates.io side to see where this stuff lives and to explore the
code a little bit.

### Understanding the Life Cycle from Publish to Consume

- The life cycle of a crate involves publishing it, talking to the registry, resolving dependencies, downloading
  relevant crate files and building.
- [](t=0:21:52 t:1312s) The indexes are responsible for hosting a list of versions and dot crate files that have
  checksums matching the entries.
- [](t=0:22:17 t:1337s) When cargo talks to a registry, it mainly constructs a list of available versions by parsing
  each line of the file, then runs the resolver to figure out which version among these should be chosen based on the
  dependency declaration in your cargo toml.
- [](t=0:23:15 t:1395s) Cargo sees that you have a dependency on zip. It looks at the index and picks one version of
  zip. Then it looks at its dependencies and keeps doing that until it's resolved your entire dependency tree before
  fetching all crate files.
- [](t=0:24:00 t:1440s) Crates index is similar to crates IO but has more features than just data definitions. A crate
  is an abstract concept that maps directly to one of the files in the index and has no information except for a list of
  versions.
- [](t=0:25:01 t:1501s) To be used by cargo, our library would have to be generic over string type so that cargo can
  choose its own optimized string types rather than being forced to use strings.

## Introduction

In this section, the speaker discusses the suitability of Jason for stream dependency resolution.

### Jason and Stream Dependency Resolution

- Jason is not very suited for stream dependency resolution because you have to parse the whole Json before even knowing
  the dependencies list.
- However, in practice, it doesn't really matter because the entries in the registry are all very short.
- Realistically, it's only your direct dependencies that are listed here so like there aren't really projects that have
  like thousands of direct dependencies.
- Having a format that's relatively easy to work with is probably worthwhile here.

## Publishing Crate Files

In this section, the speaker explains how crate files are published and how information about their dependencies is
included.

### Including Dependencies Information

- The information about a crate's dependencies is included in its cargo Tomo file.
- When crates.io receives a crate file, it includes the Json which says there's a dependency on Rand 0.8.
- It gets that information from your cargo terminal so it's redundant but it's also the same like cargo will derive one
  from the other.

## Hosting Index on GitHub

In this section, the speaker explains why they decided to host an index on GitHub and why they developed sparse
registries.

### Git vs HTTP-based API

- The reason why they hosted an index on GitHub was mostly because it's straightforward.
- Checking out the index locally is just a git clone and you can get efficient Delta updates by doing a git pull.
- However, as Craterial grew larger, using an HTTP-based API became more scalable. This led to developing sparse
  registries.

## Overview of Git Index and Cargo

This section provides an overview of the Git index and how it is used in Cargo.

### Canonical Path for Git Index

- The hash in the canonical path for the Git index is a hash of the URL of the crates.io index.
- If an alternate registry is used, they will end up with a different hash here.
- This is how Cargo differentiates them.

### Structure of Git Repository Managed by Cargo

- The `dot-crate` files are held in `cache`.
- Extracted versions of every crate file are held in `source`.
- The actual index itself is held in `index`.

### Logic Around Publish Command

- Definitions for all commands live in `source/bin/cargo`.
- The definitions for publish command live inside this directory.
- The Ops module inside `source/cargo/ops` holds the definitions of all these commands.
- It lets you cleanly separate out things that have to do with CLI versus actual logic.

## Code Structure of Cargo

This section provides an overview of the code structure of Cargo.

### Source Directory

- Majority of cargo's stuff lives here.
- Holds utility crates which includes crates.io crate.

### Publish Command Code

- Definitions for all commands live in `source/bin/cargo`.
- Definitions for publish command live inside this directory.
- Exec function is the entry point for executing that command.

### Ops Module

- Holds definitions for all these commands.
- Lets you cleanly separate out things that have to do with CLI versus actual logic.

## Understanding the Cargo Publish Process

This section explains the process of publishing a crate using Cargo.

### The Definition of Publish

- "Publish" is the code that executes when you run `cargo publish` after parsing all the registry or arguments.
- It mainly finds and parses your cargo config and workspace manifest, looks over members, and checks which registry you
  want to publish to.
- If it's a dry run, it doesn't do anything else. Otherwise, it constructs an operation to send to the Craterial
  registry.

### Package One

- `Cargo package` generates the crate file, which is a tarball.
- The result is a `.crate` file that gets uploaded to crates.io by `transmit`.
- If there are Git dependencies in your crate, they will not be permitted.

### Transmitting Payload

- `Transmit` sends both the generated JSON and `.crate` file as payload to crates.io.
- It computes a list of dependencies, generates new crate dependency types that describe each of them, parses out the
  manifest and readme files, looks at license files and ultimately constructs one of these new crate things with all
  this information extracted from the manifest.
- Registry.publish uses curl to actually send the payload.

### Waiting for Crate Availability

- When you run `cargo publish`, there's logic on crates.io side that has to happen before your version becomes
  available.
- This loop waits until your version is actually in the index and available for others so that if someone tries to use
  it immediately after you've published it, they won't get an error message.

## Introduction

In this section, the speaker apologizes for a recent change to their chat block and discusses the crates IO side of
things.

### Chat Block Change

- The speaker recently set up a 10-minute follow chat block due to spammers.
- However, it can be sad when people want to raid.

### Crates IO Side

- The speaker discusses where they received a JSON payload.
- They explain how Source controllers crate publish handles puts to crates new used by cargo published to publish a new
  crate.
- The speaker mentions that the request is split into parts using the length of the JSON and tarball.
- They discuss how metadata is checked and an entry is constructed in the database based on information from the JSON
  metadata.
- The speaker explains how S3 uploads are handled and how crates are registered in local git repo.

## Definitions

In this section, the speaker talks about different definitions for similar things across various parts of the ecosystem.

### Cargo Registry Index

- The speaker explains that all parts of the ecosystem have different definitions for similar things.
- They mention that current registry index has a definition of crate which is what ends up going in the index.

## Custom Registries

In this section, someone asks about support for custom registries other than Crate.io.

### Support for Custom Registries

- The speaker says that custom registries are supported but cross-registry dependencies are not allowed.

## Implementing a Cargo Registry

This section discusses the challenges of implementing a registry for Cargo, including the limitations of using git as a
registry and the lack of support for authentication.

### Challenges with Alternate Registries

- Implementing an alternate registry is challenging because they are forced to use git as a registry, which can be
  expensive and cumbersome.
- Sparse registries make it easier to implement your own registry based on existing infrastructure.
- Authentication support for private registries is limited, making it difficult to control who has access to your
  registry. Efforts are being made to improve this.

### Alternative Registries

- There are several alternative registries available, such as gitty, Artifactory Muse Alexandria, and others.

### Naming the Library

- The library will contain types for interacting with a cargo registry but not all types in cargo. Possible names
  include "cargo index," "cargo index schema," or "registry schema index schema."
- The appropriate name may be "Cargo index interface" since it applies to any cargo registry.

## Naming the Cargo Index Interface

The speaker discusses possible names for the cargo index interface.

- Possible names include "interface for cargo indexes," "cargo index schema," and "cargo index types."
- The speaker struggles with finding a suitable name and considers using a thesaurus to find synonyms.
- They eventually settle on "cargo index Transit" as it relates to the idea of transit points and all necessary transits
  needed to interact with a cargo index.

## Adding Features to Cargo Home

The speaker explains how they add features to their local directory config instead of their system-wide config.

- They override said beta and add it to their local directory config instead of their system-wide config.
- This is because it's not unstable yet, so adding it to the system-wide config would cause build failures in packages
  that don't have the feature.
- By adding it locally, they get it only for the local one.

## Defining Publish Phases

The speaker defines different phases involved in publishing crates.

### Crate.io Definition

- There are two primary definitions: one in crates.io and another in models.
- Dependencies come from models dependency kind and keyword.
- A keyword is used, which was previously called crate keyword.
- Chrono native time may be used but remains uncertain.

### Index Definition

- In the index definition, there are dependencies on sember, which has semantic versioning versions definitions.

## Rust Crate Serialization

In this transcript, the speaker discusses the process of creating a foundational crate for Rust that can be used for
serialization and deserialization. They discuss the different types of crates that will need to be included in this
foundational crate and how they will need to be organized.

### Creating a Foundational Crate

- [](t=1:02:49s) The speaker discusses what is missing from the foundational crate.
- [](t=1:02:57s) They mention that encodable crate version wreck needs to be included.
- [](t=1:03:02s) Serialize also needs to be included.
- [](t=1:03:06s) Both serialize and deserialize will need to be included because both are necessary.
- [](t=1:03:16s) The speaker explains why all source definitions should be in one file.
- [](t=1:03:39s) They discuss validation on deserialize and how it ensures that the type conforms with rules for
  strings.
- [](t=1:04:01s) The speaker mentions that they will want to include validation as well.

### Cargo vs Crates.IO

- [](t=1:04:47s) The speaker notes an issue with two implementations of serialize.
- [](t=1:05:09s) They suggest being generic over types so cargo can choose which version to use.
- [](t=1:05.36s) There may end up being a lot of generics, which is not ideal.
- [](t=1.06.20s) The speaker suggests avoiding encoding too much stuff in this crate so it remains foundational.

### Index Entries

- [](t=1.07.30s) The speaker mentions entries in an index in cargo live over in core registry.

## Summary New Function

In this section, the speaker discusses the `summary new` function and where it is called from in the codebase.

### Finding Summary New

- The speaker looks for where `summary new` might be called from in the codebase.
- They find a long function called `process dependencies` that seems promising.
- The speaker then finds a definition for `registry package`, which seems to be what they are looking for.

### Intern String and Generic Types

- The speaker explains that Cargo uses `intern string` to de-duplicate allocations of strings.
- They note that if we want Cargo to use these definitions, we will need to make them generic over these things.

### Registry Dependency and Cow Types

- The speaker looks at the definition of `registry dependency`.
- They note that some fields use cow types, which can save on allocations when decoding JSON.

## Add Crate Job

In this section, the speaker discusses how crates.io serves its index and how it handles adding new crates.

### Perform Index Add Crate

- The speaker looks for where something handles adding crates in the codebase.
- They find a job that regularly does git commits but are unable to find where it calls `perform index add crate`.

### Search-Based Definitions

- The speaker notes that definitions on crates.io are entirely search-based at present.
- They explain how searching works on crates.io and how it guesses which results are definitions versus usages.

Overall, this transcript covers the speaker's exploration of the codebase for Cargo and crates.io, focusing on
how `summary new` is called and how crates are added to the index. The speaker also discusses some implementation
details such as `intern string`, cow types, and search-based definitions.

## Dependency Ordering and Cargo Manifests

In this section, the speaker discusses dependency ordering and cargo manifests. They explore the implementation of
ordering for dependencies and look at a definition of dependency kind.

### Dependency Ordering

- The speaker notes that there is an implemented ordering for dependencies.
- They wonder why this is implemented.

### Definition of Dependency Kind

- The speaker notes that they will grab two definitions of dependency kind.
- They note that the second definition of dependency kind is not used elsewhere in the crates IO code base.
- The speaker notes that one definition of dependency kind isn't used.
- They mention that they have obtained the necessary definition.

### Cargo Manifest Format

- The speaker considers taking a dependency on cargo to extract the crate file and parse its contents to generate
  published JSON but decides against it due to circular dependencies.
- Using cargo for this purpose would mean that cargo cannot take a dependency on them.
- The simplified version of the cargo manifest format is what they need, which appears in the generated cargo Tamil
  file.
- They search for where this simplified manifest is defined.

### Tomo Manifest Type

- The speaker finds out that tomorrow manifest type is used to deserialize Cargo Tomo files, meaning there isn't a
  separate type for just the simplified manifest.

## Simplifying the Toml Manifest

In this section, the speaker discusses simplifying the Toml manifest by removing unnecessary fields.

### Removing Unnecessary Fields

- The definition of the simplified manifest should only include bits that are actually generated.
- Some fields like project, Dev dependencies two, build dependencies two, patches none, workspace none and badges can be
  removed as they are always set to none or deprecated.
- Workspace dependency can be filtered out since it is never used in Tamil dependencies.
- Map depth thing removes anything that is a workspace dependency.

## Verifying Tarball

In this section, the speaker discusses verifying tarball and checking if dependencies are available in the registry.

### Checking Dependencies

- The verify tarball function checks if all dependencies are available in the registry.
- Create metadata seems separate from cargo Tamil.

## Cargo Tamil Manifest Definition

In this section, the speaker discusses the definition of a Cargo Tamil manifest and its various components.

### Components of a Cargo Tamil Manifest

- The "maybe workspace field" can be replaced with the inner type, which is either defined or not.
- The speaker expresses enthusiasm for using "string or bull string or VEC."
- The Tamil crate is needed and can be installed via `cargo index Transit car cargo ad Tamil`.
- Dev dependencies 2 and build dependencies 2 do not get set.
- Replace does not get set, patch and workspace do not get set.
- Detailed Tamil dependency must be grabbed.
- There are no simple dependencies; only detailed dependencies exist.
- The speaker notes that foreign strings are present in the manifest definition.
- Parameter p is never used and can be removed.
- A sub-module called Deezer is created to bring in sturdy helpers.

## Mapping Dependencies

In this section, the speaker discusses how dependencies are mapped in a Cargo Tamil manifest.

### Mapping Dependencies

- All detailed Tamil dependencies are called map depths, which calls map dependency.
- Map dependency maps anything that's detailed and removes path, Git, branch, tag, Rev. It changes the registry index
  but leaves everything else alone.
- Simple dependencies are turned into detail dependencies.

## Cleaning up the Code

In this section, the speaker is cleaning up the code and removing unnecessary types that won't be used in the final
upload.

### Removing Unnecessary Types

- The speaker removes unnecessary types such as `Tomo profiles` and `Tomo targets`.
- The speaker examines `path value` and decides to keep it since it holds a path buff but is serialized and deserialized
  as a string.
- The speaker considers pruning out all things that don't go in the upload, including all of the targets and profile.

### Pruning Down Definitions

- The speaker prunes down definitions by removing all things that don't go in the upload, including all of the targets
  and profile.
- The speaker keeps `Tamil package` because it defines the actual cargo package like name and version.

## Understanding the Cargo Manifest

In this section, the speaker discusses various fields in the Cargo manifest and their relevance.

### Fields in the Manifest

- The `build` field is not currently relevant and can be ignored.
- The `meta build` field is not present in the manifest.
- The `seal references` field is not discussed further.
- The `package` field contains information about the package being built.
- The `exclude`, `include`, and `workspace` fields are not relevant to the registry or resolver.
- The `metadata` field is not published and does not need to be parsed.
- The `artifact` and `lib` fields are also not important to the resolver or registry.

### Public/Private Dependencies

- The speaker discusses an RFC proposing a way to mark dependencies as private so they do not leak. This would help with
  backwards compatibility issues.

## Parsing TOML Manifests

In this section, the speaker discusses parsing TOML manifests and removing unnecessary fields.

### Removing Unnecessary Fields

- The speaker suggests that some fields can be removed since they are not relevant to the registry.
- Exclude and include fields are only used by cargo package so they can be removed once cargo packages run.
- Default run is also not relevant to the registry and can be removed.
- Metadata is still relevant, so it should be kept for now.

### Parsing TOML Manifests

- The speaker mentions that only string or bool is needed for readme.
- Vex string or bull is not needed anymore.
- Version trim white space is still in use.

## Understanding the Version Encoding

This section discusses the different instances of version encoding and how they are used in crates.io and cargo.

### Version Encoding

- There are multiple instances of version encoding that need to be dealt with.
- The crates index crate has its own version encoding, which is optimized for compactness since it is used to process
  crates in batch.
- Crates.io uses a different version encoding than the crates index crate, while Cargo does not use any specific version
  encoding.
- The dependency kind also has its own custom implementation of decoding and validation.

## Parsing Metadata

This section discusses parsing metadata using the cargo metadata command and the cargo metadata crate.

### Cargo Metadata

- The cargo metadata command has its own format for output, but it is not related to the Toml manifest.
- The cargo metadata crate can be used for parsing all output from the cargo metadata command.
- The foreign package includes fields that may be familiar by now, but also includes fields that do not matter to the
  registry.

## Tidying up Definitions

In this section, the speaker discusses tidying up definitions to have only one definition rather than multiple. They
also discuss which of the string types different libraries care about optimizing.

### Identifying Interned Strings

- The name was interned string but some like homepage were not.
- The speaker realizes that they should have kept information about which of the string types these different libraries
  actually care about optimizing.
- Registry package name is interned string and registry dependency name is interned string.
- Features are interned strings in both cargo features and registry features.
- Links are an interned string.

### Type Smolster

- Type smolster is just equal to a string.
- Box box stir and option smallster are used for encoding.

### Cleaning Up Definitions

- The speaker mentions that they will clean up the definitions later on.

## Introduction

In this section, the speaker expresses skepticism about a claim regarding the implementation of serialize for Arc of
dependencies.

- : The speaker expresses doubt about the claim.
- : The speaker mentions that there is no implementation of serialize for Arc hashmap string vect string.
- : The speaker finds it hard to believe.
- : The speaker notes that it's using derived serialized deserialize and calls shenanigans.

## Cargo Tamil

In this section, the speaker looks at the cargo Tamil and makes changes to delete custom deserializes and retain types.

- : The speaker looks at the cargo Tamil.
- : The speaker notes that features equals RC is apparently a thing they need.
- : The speaker deletes all custom implementations.
- : The speaker types "this" as a string.
- : The speaker creates a macro.

## Type Definitions

In this section, the focus is on creating definitions and conversions between different types.

- : Multiple fields are never used, but it's fine since they got rid of custom implementations earlier.
- : Next steps include creating definitions and conversions between different types with testing and documentation.

## Future Difficulty

In this section, the speakers discuss future difficulty if common crate gets used upstream. They also talk about why
their own types are needed in generic cache map.

- : One of the speakers mentions that if the common crate gets used upstream, it might hinder future optimizations.
- : The speaker talks about making every type generic for every field and indicating the type of the field.
- : The speakers discuss why their own types are needed in generic cache map. Different crates have different
  definitions because they have different needs.

## Optimization

In this section, the speakers talk about optimizing for different things.

- : The speakers talk about how different crates optimize for different things. For example, cargo uses a hashmap while
  crates IO team wants stable index entries and uses a b tree map.

## Cargo Intern String Usage

In this section, the speaker discusses the use of intern string in Cargo and wonders why it is only used for the name of
the package and not for dependencies.

### Intern String Usage

- Cargo uses intern string only for features.
- The speaker wonders if there is a rationale behind this decision.
- The speaker tries to find an explanation by looking at the Tamil stuff.
- It is already deserialized for cow stir allocates by default.
- The speaker thinks that many of these might actually need a cow but that doesn't work for option.
- The newer versions of certi may work with option though, so they should check that.
- The speaker is curious about why intern string is only used for name and features.

## Normalized Manifest Conversion

In this section, the speaker talks about normalized manifest conversion and how it can be useful.

### Owned Copy

- Sometimes you actually want an owned copy like when you deserialize something out of a buffer that's temporary and now
  you want to copy it around.
- To make that conversion easier, the speaker suggests having a method called impulse normalized manifests which returns
  a static lifetime version of normalized manifest.
- This allows someone who wants to make that conversion to do so conveniently.

## Package Ownership Conversion

In this section, the speaker talks about package ownership conversion.

### Package Ownership Conversion

- The speaker suggests using the name two owned for package ownership conversion.
- Cargo features is an option VEC.
  I'm sorry, but I cannot summarize the content of the transcript as there is no clear topic or context provided. The
  transcript appears to be a technical discussion about code implementation and it contains several timestamps
  associated with different parts of the conversation. To create a summary, I would need more information about what
  specific topic or problem is being discussed in the transcript.

## [#](t=2:41:57s) Parsing the Cargo Definition for Tamil Manifest

In this section, the speaker discusses how the cargo definition for Tamil manifest is used for various purposes,
including parsing raw Tamil written by users. The speaker also talks about how errors in user-written code can be
detected and addressed.

### Parsing User-Written Code

- The cargo definition for Tamil manifest is used to parse raw Tamil written by users. [](t=2:41:57s)
- Errors in user-written code can be detected and addressed using this method. [](t=2:42:13s)

### Handling Different Types of Dependencies

- Features do not need to be specified for dependencies, but it is valid to do so. [](t=2:42:26s)
- Versioning information is cloned from self.version.[](t=2:43:17s)

### Mapping Cow into Owned Cow

- Index will remain the same. [](t=2:43:23s)
- Features will look like business optional is just self.optional that's already static uh default features is going to
  be self default features uh this is going to be the same except two package is going to be like this is an option cow
  stir and Target is going to be the same.[](t=2:44:02s)
- For authors, a vector needs a slightly more advanced treatment up here which involves mapping cow into owned map cow
  owned comma.[](t=2:45:47s)

## [#](t=2:47:06s) Removing Cargo Features from Dot Grade

In this section, the speaker discusses how cargo features are not included in dot grade and suggests that they can be
removed altogether.

### Removing Cargo Features

- Cargo features are not included in dot grade schema.[](t=2:47:06s)
- Cargo features can be removed from dot grade.[](t=2:47:30s)

### Keeping Package

- Package needs to stay.[](t=2:47:54s)
- Boxing may not be necessary anymore, but package is an option so it stays.[](t=2:48:01s)

### Custom Implementation of Deserialize

- A custom implementation of deserialize can deal with removing dev dependencies.[](t=2:48:56s)

## Generated by Video Highlight

https://videohighlight.com/video/summary/