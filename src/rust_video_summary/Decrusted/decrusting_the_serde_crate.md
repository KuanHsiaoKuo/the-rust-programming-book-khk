# Decrusting the serde crate

<!--ts-->
* [Decrusting the serde crate](#decrusting-the-serde-crate)
   * [Introduction](#introduction)
      * [Purpose of the Stream](#purpose-of-the-stream)
      * [Streaming Schedule](#streaming-schedule)
   * [Understanding Serde](#understanding-serde)
      * [What is Serde?](#what-is-serde)
      * [Serde Data Model](#serde-data-model)
   * [Introduction to the 30 Data Model](#introduction-to-the-30-data-model)
      * [The Purpose of the 30 Data Model](#the-purpose-of-the-30-data-model)
      * [The Visitor Type](#the-visitor-type)
      * [What is the Sturdy Data Model?](#what-is-the-sturdy-data-model)
      * [Mapping from Rust Types to Sturdy Data Model](#mapping-from-rust-types-to-sturdy-data-model)
   * [Introduction to serde](#introduction-to-serde)
      * [Serde's Versatility](#serdes-versatility)
   * [Performance Implications of Serde](#performance-implications-of-serde)
      * [Performance Implications](#performance-implications)
   * [Implementing Serialize/Deserialize with Serde](#implementing-serializedeserialize-with-serde)
      * [Implementing Serialize/Deserialize](#implementing-serializedeserialize)
   * [Overview of Serialize Implementation](#overview-of-serialize-implementation)
      * [Serializing Structs](#serializing-structs)
   * [Code Walkthrough](#code-walkthrough)
      * [Serialized Struct Constructor](#serialized-struct-constructor)
      * [Serializing Fields](#serializing-fields)
   * [Overview of Serialization](#overview-of-serialization)
      * [Implementing Serialize for a Custom Data Type](#implementing-serialize-for-a-custom-data-type)
   * [More on Serialization](#more-on-serialization)
      * [Using Question Marks with Serialize](#using-question-marks-with-serialize)
      * [Additional Attributes for Serialization](#additional-attributes-for-serialization)
   * [Deserialization Basics](#deserialization-basics)
      * [Deriving Deserialize Trait](#deriving-deserialize-trait)
      * [Differences Between Serialization and Deserialization](#differences-between-serialization-and-deserialization)
   * [Deserialization and Visitor Pattern](#deserialization-and-visitor-pattern)
      * [Deserializer Lifetime](#deserializer-lifetime)
      * [Visitor Pattern](#visitor-pattern)
   * [Deserialization Implementation](#deserialization-implementation)
      * [Field Visitor](#field-visitor)
      * [Ignore Fields](#ignore-fields)
      * [Expecting Method](#expecting-method)
      * [Implementing a Field Visitor](#implementing-a-field-visitor)
      * [Implementing a Visitor for Foo](#implementing-a-visitor-for-foo)
   * [Deserializing Structs with Serde](#deserializing-structs-with-serde)
      * [Deserializing a Struct](#deserializing-a-struct)
      * [Tying Visitor Implementation into Deserializer Call](#tying-visitor-implementation-into-deserializer-call)
   * [Container and Enum Attributes](#container-and-enum-attributes)
      * [Container Attributes](#container-attributes)
      * [Enum Attributes](#enum-attributes)
   * [Deserializing Enums in Rust](#deserializing-enums-in-rust)
      * [Enum Deserialization](#enum-deserialization)
      * [Enum Encoding](#enum-encoding)
      * [Switching to Internally Tagged Encoding](#switching-to-internally-tagged-encoding)
   * [Internal Tagging](#internal-tagging)
      * [Serializing a Struct with an Additional Field](#serializing-a-struct-with-an-additional-field)
      * [Deserializing a Struct with an Additional Field](#deserializing-a-struct-with-an-additional-field)
      * [Tagged Content Visitor](#tagged-content-visitor)
   * [Deserializer for Buffered List of Fields](#deserializer-for-buffered-list-of-fields)
      * [Implementing Deserializer for Buffered Lists of Fields](#implementing-deserializer-for-buffered-lists-of-fields)
   * [Deserializing Untagged Enums](#deserializing-untagged-enums)
      * [Deserializing Untagged Enums](#deserializing-untagged-enums-1)
   * [Deserializing Tagged Enums](#deserializing-tagged-enums)
      * [Deserializing Tagged Enums](#deserializing-tagged-enums-1)
   * [Enums and Serialization](#enums-and-serialization)
      * [Implementation of Deserialize](#implementation-of-deserialize)
      * [Adjacently Tagged](#adjacently-tagged)
      * [Untagged](#untagged)
   * [Variant Attributes](#variant-attributes)
      * [Rename and Alias](#rename-and-alias)
      * [Skip](#skip)
      * [Serialize With and Deserialize With](#serialize-with-and-deserialize-with)
      * [Bound](#bound)
   * [Understanding Cow and Deserialize](#understanding-cow-and-deserialize)
      * [Using Deserialize Symbol](#using-deserialize-symbol)
      * [Cow Special Case](#cow-special-case)
      * [Adding Bound for D Outlives Take A](#adding-bound-for-d-outlives-take-a)
   * [Understanding the Data Format](#understanding-the-data-format)
      * [Deserializing Fields](#deserializing-fields)
   * [Writing a Data Format](#writing-a-data-format)
      * [Implementing Serializer and Deserializer Traits](#implementing-serializer-and-deserializer-traits)
      * [Serializer Implementation](#serializer-implementation)
   * [Serializing and Deserializing Data](#serializing-and-deserializing-data)
      * [Serialization](#serialization)
      * [Deserialization](#deserialization)
      * [Sequences and Maps](#sequences-and-maps)
         * [Deserializing Sequences](#deserializing-sequences)
         * [Deserializing Maps](#deserializing-maps)
   * [Understanding Serde](#understanding-serde-1)
      * [Implementation of Serializer and Deserializer](#implementation-of-serializer-and-deserializer)
      * [Mental Model and Basic Understanding](#mental-model-and-basic-understanding)
   * [Conclusion](#conclusion)
      * [Thank You Note](#thank-you-note)
   * [Generated by Video Highlight](#generated-by-video-highlight)

<!-- Created by https://github.com/ekalinin/github-markdown-toc -->
<!-- Added by: runner, at: Tue Apr 11 15:21:43 UTC 2023 -->

<!--te-->

## Introduction

Section Overview: In this section, the speaker introduces the purpose of the stream and what they hope to achieve.

### Purpose of the Stream

- The stream aims to take a look at core crates in the Rust ecosystem and understand how they work.
- The goal is not necessarily to read through all the code but rather to have a better understanding of what goes on
  when using certain crates.
- This will help users derive, serialize, deserialize or implement their own data formats.

### Streaming Schedule

- The speaker has committed to a regular streaming schedule every fourth Friday at 6 p.m. UTC time.
- Users can subscribe to a calendar link provided on Mastodon and co-host.

## Understanding Serde

Section Overview: In this section, the speaker explains what Serde is and its goal.

### What is Serde?

- Serde is short for serialization and deserialization.
- Its goal is not to provide any particular serialization or deserialization format but rather infrastructure for doing
  so with Rust data structures.
- It provides high efficiency and performance in most cases.

### Serde Data Model

- The Serde data model consists of three parts: data format, data type, and the Serde data model.
- The rust data type in use represents the data type in Serde.

## Introduction to the 30 Data Model

Section Overview: This section introduces the 30 data model and its purpose in providing a mapping between serializers
and deserializers.

### The Purpose of the 30 Data Model

- The goal of the 30 data model is to provide a mapping between serializers and deserializers.
- The data model provides a layer of encapsulation that separates concerns between the data format and data type.

### The Visitor Type

- The visitor type is used by both serializers and deserializers, but it's implemented by the data type side of things.
- It's essentially part of the interface which serialize and deserialize too.

### What is the Sturdy Data Model?

- The sturdy data model consists mostly of types chosen to represent primitives for data such as numeric types, string
  types, byte arrays, options units, structs, enums, sequences, tuples, and maps.

### Mapping from Rust Types to Sturdy Data Model

- For serialization purposes, you need to take rust types stored internally in your program and turn them into one of
  these sturdy types.
- For deserialization purposes, you need to take stuff that you're getting out of the underlying data format and turn
  them into sturdy types using visit methods through surdy data models.

## Introduction to serde

Section Overview: In this section, the speaker introduces serde and explains how it can be used to mix and match
different data formats.

### Serde's Versatility

- Serde is versatile because it allows mixing and matching of different data formats.
- Derive deserialize on a Rust type allows deserialization from both Taml and JSON.
- When using the JSON deserializer, the deserialized method is called on the derived DC release type.
- The same applies when using the Taml deserializer.

## Performance Implications of Serde

Section Overview: This section discusses whether there are any performance implications when using serde.

### Performance Implications

- There might be some performance implications when using serde since it is not a zero-cost abstraction.
- Deserializers generally do not do allocation unless necessary.
- If you give a string reference to the JSON deserializer, it walks that string and gives a slice of that into the data
  model without doing any allocation.
- However, if there are escape values in the string, then you have to allocate a new string.

## Implementing Serialize/Deserialize with Serde

Section Overview: In this section, we dive into implementing serialize/deserialize with serde.

### Implementing Serialize/Deserialize

- We create a new library by running `cargo new lib`.
- We pull in derived features so that we can derive deserialize on our Rust type.
- We use `#[derive(Serialize, Deserialize)]` on our struct Foo which has an `a` field of type u64 and a `b` field of
  type string.
- We expand all macros by running `cargo expand`.
- The generated code includes an implementation of serialized for Foo.

## Overview of Serialize Implementation

Section Overview: This section provides an overview of the implementation of serialize. The goal is to call the
appropriate serialize method on the serializer that you're passed.

### Serializing Structs

- When serializing a struct, you will probably call the serialized struct method and pass in the name of the struct.
- There is an Associated type for serialized struct which implements the serialized struct trait which has the method
  serialized field and end.
- To implement serialis and serialize a struct, you will call the serialized struct method on the serializer you're
  given, giving it the name of the struct and number of fields. You will then call serialize field repeatedly for each
  field you want to serialize and then call end when finished.

## Code Walkthrough

Section Overview: This section provides a walkthrough of generated code for serialization.

### Serialized Struct Constructor

- The generated code calls serialized struct with Foo as its argument, which returns a constructor that we'll use to
  call serialize field.
- If serialized struct completes successfully, we assign its return value (the constructor) to sturdy state. If it
  errors, we return using question mark operator.

### Serializing Fields

- We then call serialize field on sturdy state to serialize each field by giving a as key (field name), and reference to
  value of a as second argument.

## Overview of Serialization

Section Overview: This section covers the basics of serialization in Rust, including how to implement it for a custom
data type.

### Implementing Serialize for a Custom Data Type

- To implement serialization for a custom data type, you need to derive the `Serialize` trait.
- The `Serialize` trait requires that you define how each field should be serialized using the `serialize_field` method.
- You can use the `serialized_field` macro to generate code that implements this method for each field in your struct.
- If you need more control over how a field is serialized, you can use attributes like `skip_serializing_if`, which
  allows you to skip serializing a field if it meets certain conditions.

## More on Serialization

Section Overview: This section covers additional details about serialization in Rust, including how to customize it
further and why some features are not used.

### Using Question Marks with Serialize

- The question mark operator (`?`) is not used when implementing serialization because it produces slower code and
  larger binary size through implicit into conversion being done.
- Instead, direct error handling is used since we know that the error type is directly translatable here.

### Additional Attributes for Serialization

- There are many additional attributes that can be used when implementing serialization in Rust.
- These attributes allow you to modify the generated code in various ways, such as renaming fields or skipping them
  under certain conditions.

## Deserialization Basics

Section Overview: This section covers deserialization in Rust and how to implement it for a custom data type.

### Deriving Deserialize Trait

- To implement deserialization for a custom data type, you need to derive the `Deserialize` trait.
- The `Deserialize` trait requires that you define how each field should be deserialized using the `deserialize_field`
  method.
- You can use the `deserialize_field` macro to generate code that implements this method for each field in your struct.

### Differences Between Serialization and Deserialization

- While serialization and deserialization are similar, there are some key differences between them.
- For example, deserialization requires that you handle errors differently than serialization does.

## Deserialization and Visitor Pattern

Section Overview: This section covers the deserialization process and the visitor pattern used to deserialize nested
structures.

### Deserializer Lifetime

- The lifetime of a string reference returned during deserialization is determined by the `de` reference, which is a
  reference into the original data for the deserializer.
- If reading from disk, this lifetime is static and only owned values are generated.

### Visitor Pattern

- The visitor pattern makes it easy to deserialize nested structures.
- A private enum called `FieldVisitor` visits each field of a struct and implements `Visitor` traits.
- When calling any of the `deserialize` methods, you give in a visitor that maps to what's coming next in the data
  format.

## Deserialization Implementation

Section Overview: In this section, the speaker discusses the implementation of deserialize into the deserializer which
consumes self. The visitor is called and it calls deserializer again.

### Field Visitor

- The field visitor is a visitor for the fields of the struct.
- When you encode a struct, usually you want to emit both the fields and values.
- If you tell the data model that data format is bytes, then it will call visit bytes in response.
- The field visitor produces a value which is an enum saying field zero or field one which corresponds to A and B.

### Ignore Fields

- By default, serde allows unknown fields.
- This is primarily used for backward compatibility where if someone adds a field to JSON files that you're parsing
  actually changes like someone adds a field to it your code shouldn't necessarily break right if there's just more data
  in the JSON then this is a great way to just be able to subset that information or just allow the underlying JSON to
  evolve over time while you are still able to run because all the stuff you cared about is still in there.
- You can set 30 deny unknown fields to say every field must be in my data type if you ever encounter a field in the
  data format that I don't have in my data type that should error rather than just be ignored.

### Expecting Method

- On visitor there's this expecting method which is primarily used for error message generation so this would be
  something like imagine that you are visiting a JSON structure a dictionary or map if you will in JSON every key has to
  be string but if it finds something else then it's going to call expecting.

## [#](t=0:40:41s) Implementing a Field Visitor

Section Overview: In this section, the speaker discusses how to implement a field visitor and deserialize it into a map.

### Implementing a Field Visitor

- [](t=0:40:41s) When debug printed, the expected type will use the expecting method of the visitor to say what was
  expected instead of what you actually got.
- [](t=0:41:07s) The default implementation for various visit methods is to emit a message saying "I got this, I
  expected this."
- [](t=0:42:01s) Two ways are allowed to encode the struct. People can encode it as the zeroth field or first field.
- [](t=0:43:12s) Deserialize needs to be implemented for fields because ultimately we want to deserialize into a map.

## [#](t=0:44:59s) Implementing a Visitor for Foo

Section Overview: In this section, the speaker discusses implementing a visitor that produces Foo.

### Implementing a Visitor for Foo

- [](t=0:44:59s) We need a visitor that produces Foo.
- [](t=0:45:20s) This is the visitor that's going to produce what we want.
- [](t=0:45:29s) We're going to implement visit seek and visit map and those are the only two things we're going to
  implement.

## Deserializing Structs with Serde

Section Overview: This section explains how to deserialize structs using Serde.

### Deserializing a Struct

- Grpc produces fields in order.
- Json deserializer has a different implementation that deals with arrays and implements a seek access trait.
- For visit seek, we first try to access the next element of the sequence we're visiting and deserialize it as a u64,
  which is the type of the first field. If successful, this is the value of field zero. If not, it's an error.
- Field zero here will contain the value of field zero if everything went well. Field one is deserialized as a string
  and could be any type that implements deserialize.
- For visit map, variables for both fields are created and set to none because you don't know which key will come first
  due to possible out-of-order encoding. The next key in the map is deserialized as a field. If it's field zero or one,
  then its corresponding value is deserialized as either u64 or string respectively.

### Tying Visitor Implementation into Deserializer Call

- The call down here tells the data format that the next thing to grab from the data stream is a struct.
  I'm sorry, but I cannot see any transcript provided. Please provide the transcript so that I can summarize it for you.

## Container and Enum Attributes

Section Overview: This section covers the container and enum attributes in Rust.

### Container Attributes

- The `default` attribute sets the default value for a field.
- The `transparent` attribute is used to avoid generating code for a type.
- The `30` attribute is used to specify the name of an external crate.
- The `from` and `try from` attributes are used for proxy types where one type can be converted into another.

### Enum Attributes

- For serialization, the `serialized struct variant` trait is used instead of the `serialized struct` trait.
- For deserialization, there are some differences between enums and structs.

## Deserializing Enums in Rust

Section Overview: This section covers how to deserialize enums in Rust and the importance of understanding how data
formats dictate enum encoding.

### Enum Deserialization

- When deserializing an enum, we expect to visit an enum and determine which variant it is.
- The data format dictates how enums are encoded through implementing the `DeserializeEnum` trait.
- If the variant is field zero, then we generate code for deserializing that variant type. We do this because the
  deserialized code for a struct with one field is exactly the same as for a variant with one field.
- For each variant separately, we generate a macro-like outer deserialized function that determines which of those
  variants to use.

### Enum Encoding

- The way in which data formats encode enums varies greatly. There are multiple ways of encoding an enum within a single
  data format.
- Enum representations can be externally tagged, internally tagged, adjacently tagged or untagged.
    - Externally tagged: Variant placed outside representation of contents
    - Internally tagged: Field inside serialization tells you which variant you're in
    - Adjacently tagged: Tuple of variant and contents
    - Untagged: No indication of which variant it originally was

### Switching to Internally Tagged Encoding

- To switch to internally tagged encoding, we need to tag the value with the name of the field inside its
  representation.

## Internal Tagging

Section Overview: In this section, the speaker discusses internal tagging and how it affects the data format. They also
explain how to serialize and deserialize a struct with an additional field.

### Serializing a Struct with an Additional Field

- The extra field in the struct means that as far as the data format is concerned, it's not an enum anymore.
- The implementation of serialize just serializes a foo and then serializes a field called type where the value is bar
  and then serializes all the other fields.

### Deserializing a Struct with an Additional Field

- Implementing deserialize is straightforward since it's just deserializing as if it were a struct within one additional
  field.
- The deserialize any here is telling the deserializer to deserialize whatever comes next and visit it using this tagged
  content visitor which apparently is in certi itself.

### Tagged Content Visitor

- Internally, titaniums are only supported in self-describing formats.
- If it's a map, then it visits everything in the map and sees if the thing visited is the same as the tag field.
  Otherwise, just store it.
- This representation may be slightly less performant than externally typed variants because it has to buffer.

## Deserializer for Buffered List of Fields

Section Overview: In this section, we learn about implementing deserializer for buffered lists of fields.

### Implementing Deserializer for Buffered Lists of Fields

- Cert treats buffered lists of fields as a data format and implements deserializer for them.
- This makes sense because they hold data that can be turned into other data types.

## Deserializing Untagged Enums

Section Overview: In this section, the speaker explains how to deserialize untagged enums in Rust.

### Deserializing Untagged Enums

- The code doesn't know which variant it is deserializing into and has to capture all of the fields that it walked by
  until it gets to the tag.
- It collects all of the ones that are not the tag but doesn't know their types yet so it has to deserialize them
  because it has to buffer them for later.
- The format needs to be self-describing as we walk through the fields. We don't know their type yet, so we can't give
  any hints because we don't know which enum variant we're in.
- We just need to deserialize any which means what we grab out of the data format, we store directly into a content
  thing which encodes all of the possible values in the data model.
- There is a separate crate called surdy value that has this publicly exposed xenome.

## Deserializing Tagged Enums

Section Overview: In this section, the speaker explains how to deserialize tagged enums in Rust.

### Deserializing Tagged Enums

- When calling `deserialize`, we call `deserialize_any`.
- We create one of these content deserializers from 30 itself. This implements serializer by calling appropriate visit
  method for whatever stored data model value is with visitor that was just had.
- If you're implementing DCR lights yourself, these are kinds of subtleties that sometimes you have to think about if
  you do get into really weird data types or formats.

## Enums and Serialization

Section Overview: This section covers the implementation of enums and serialization.

### Implementation of Deserialize

- The derived deserialized for the definition of baz is the same as field one for baz.
- The visitor is the real implementation of deserialize.
- Visitor is connected to a particular deserializer that we end up using.

### Adjacently Tagged

- Adjacently tagged is similar to externally tagged, but instead of using the value of a field, you look for the value
  of an adjacent field by some name.

### Untagged

- Untagged is similar to internally tagged except that you don't get to look for a tag.
- You end up with a vector of contents and then try to deserialize each data format.

## Variant Attributes

Section Overview: This section covers variant attributes such as rename, alias, skip, serialize with, deserialize with,
bound, borrow and other.

### Rename and Alias

- Rename isn't really interesting.
- Alias allows it to be deserialized with multiple different names.

### Skip

- Skip isn't that interesting here.

### Serialize With and Deserialize With

- They change the generated code so that instead of calling Leonardo passing in a reference to the underlying type you
  have a little Constructor around it that instead calls this method rather than calling the um deserialized method
  directly on that type of field.

### Bound

- Borrow ensures certi generates appropriate bounds.
- It's mainly there to ensure D has an association with take_a because if we are truly going to deserialize from a
  reference of d_e into this field which has lifetime take_a there has to be an association between them.

## Understanding Cow and Deserialize

Section Overview: In this section, the speaker discusses how to use the symbol of deserialize and explains that cow is
special cased.

### Using Deserialize Symbol

- The code looks exactly the same as it used to.
- If you try to use deserialize, you will get an error from the compiler saying that "de" doesn't live long enough.
- The speaker tries using "serde_json::from_str::<String>" but it doesn't work.

### Cow Special Case

- The speaker wonders why something works when it shouldn't.
- Cow is special cased where it always turns into a borrowed string not stir. Therefore, it can assume any lifetime
  here.
- If cow wasn't special cased, we would have gotten an error message saying that Lifetime may not live long enough.

### Adding Bound for D Outlives Take A

- To make this valid, there needs to be a bound saying d e outlives take a in the deserialized implementation.
- When implementing DC release thing here, add the bound that D is going to outlive TK because that is the case in which
  this is okay.

## Understanding the Data Format

Section Overview: In this section, the speaker discusses how to deserialize a field and create an implementation of DC
relies for it. They also talk about the Borocaster and how it enables fields that ultimately borrow from the data
formats input.

### Deserializing Fields

- The code for deserializing a field has changed.
- The Borocaster creates a cow owned if it visits a stir, but if it visits a borrowed string that is not long-lived,
  then it produces calboro visit stir.
- Borrow adds d e bound which enables implementation with fields that ultimately borrow from the data formats input.

## Writing a Data Format

Section Overview: In this section, the speaker talks about writing a data format in Certi. They discuss conventions such
as having an error type shared between serialization and deserialization, implementing serializer and deserializer
traits, and custom messages.

### Implementing Serializer and Deserializer Traits

- You should have an error type shared between serialization and deserialization.
- Implementing serializer and deserializer traits are important parts of writing a data format in Certi.
- Certi does not provide mechanisms for parsing or interpreting a data format or producing that data format. It only
  provides you with connections to serialized and deserialized trait implementations of data types.

### Serializer Implementation

- To implement serializer trait, you need to implement okay and error associated types. The okay type generally holds
  your output because it might be like a file writer where you don't want to buffer into memory before writing out.

## Serializing and Deserializing Data

Section Overview: This section discusses the implementation of serialization and deserialization for different data
formats.

### Serialization

- Serialization involves implementing all necessary traits for a given data format.
- For structs or enums, they are turned into dictionaries or maps respectively.
- The `serialize` method is called to serialize the data, which emits the appropriate characters for the format.
- The `serialize_map` method emits curly brackets and returns self. The `serialize_seq` method starts a sequence and
  returns self.

### Deserialization

- Certain is not a parsing library, so when implementing deserialization, you have to parse the input yourself.
- You can choose whether or not to implement `deserialize_any`, but it's generally recommended for self-describing
  formats.
- Implementing `deserialize_any` involves forwarding to other deserialize implementations based on the type of value
  being deserialized.
- Parsing involves extracting values from input and passing them to appropriate visit methods on visitor object.
- Deserializing options requires knowledge of how your data format represents values that may or may not be present.

### Sequences and Maps

#### Deserializing Sequences

- To deserialize sequences, look for an open bracket and create a parser for contents of array using comma-separated
  values.
- Implement access seek trait so caller can keep calling next element until there are no more elements left in sequence.

#### Deserializing Maps

- To deserialize maps, look for an open curly bracket and create a parser using comma-separated key-value pairs.
- Implement access map trait so caller can keep calling next element until there are no more elements left in map.

## Understanding Serde

Section Overview: In this section, the speaker talks about the implementation of serializer and deserializer for Surdy
Json and 30 Json value.

### Implementation of Serializer and Deserializer

- The implementation of serializer and deserializer for Surdy Json or 30 Json value is an interesting thing to look at.
- There's a lot of good juicy stuff there if you want to pick up more about how certi works.

### Mental Model and Basic Understanding

- After going through this section, viewers should have a mental model and basic understanding of the types involved in
  serde.
- This will help them figure out the rest on their own.

## Conclusion

Section Overview: In this section, the speaker concludes by thanking everyone for attending the session.

### Thank You Note

- The speaker thanks David (detolnate) for creating serde.
- He also mentions that he will upload the video to YouTube with chapter marks that people can scan through.
- Finally, he expresses his excitement about doing more de-crusted things in future sessions.

## Generated by Video Highlight

https://videohighlight.com/video/summary/BI_bHCGRgMY