# Struct visibility: an extra level of visibility with their fields

Structs have an extra level of visibility with their fields:

- The visibility defaults to private
- and can be overridden with the `pub` modifier.

~~~admonish tip title="This visibility only matters when a struct is accessed from outside the module where it is defined, and has the goal of hiding information (encapsulation)." collapsible=true
```rust,editable
mod my {
    // A public struct with a public field of generic type `T`
    pub struct OpenBox<T> {
        pub contents: T,
    }

    // A public struct with a private field of generic type `T`
    pub struct ClosedBox<T> {
        contents: T,
    }

    impl<T> ClosedBox<T> {
        // A public constructor method
        pub fn new(contents: T) -> ClosedBox<T> {
            ClosedBox {
                contents: contents,
            }
        }
    }
}

fn main() {
    // Public structs with public fields can be constructed as usual
    let open_box = my::OpenBox { contents: "public information" };

    // and their fields can be normally accessed.
    println!("The open box contains: {}", open_box.contents);

    // Public structs with private fields cannot be constructed using field names.
    // Error! `ClosedBox` has private fields
    //let closed_box = my::ClosedBox { contents: "classified information" };
    // TODO ^ Try uncommenting this line

    // However, structs with private fields can be created using
    // public constructors
    let _closed_box = my::ClosedBox::new("classified information");

    // and the private fields of a public struct cannot be accessed.
    // Error! The `contents` field is private
    //println!("The closed box contains: {}", _closed_box.contents);
    // TODO ^ Try uncommenting this line
}
```
~~~

1. A public struct with a public field of generic type `T`
2. A public struct with a private field of generic type `T`
3. A public constructor method
4. Public structs with public fields can be constructed as usual, and their fields can be normally accessed.
5. Public structs with private fields cannot be constructed using field names.
6. However, structs with private fields can be created using public constructors
7. and the private fields of a public struct cannot be accessed.

### See also:

[generics][generics] and [methods][methods]

[generics]: ../generics.md

[methods]: ../fn/methods.md
