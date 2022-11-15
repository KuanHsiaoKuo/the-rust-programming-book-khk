# Associated items: an extension to `trait` generics

- "Associated Items" refers to a set of rules pertaining to [`items`][items]
of various types. 
- It is an extension to `trait` generics
- and allows `traits` to internally define new items.

One such item is called an *associated type*, providing simpler usage 
patterns when the `trait` is generic over its container type.

### See also:

[RFC][RFC]

[items]: https://doc.rust-lang.org/reference/items.html
[RFC]: https://github.com/rust-lang/rfcs/blob/master/text/0195-associated-items.md
