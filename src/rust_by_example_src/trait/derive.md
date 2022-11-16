# Derive Macro Traits

The compiler is capable of providing basic implementations for some traits via
the `#[derive]` [attribute][attribute].

> These traits can still be manually implemented if a more complex behavior is required.

The following is a list of derivable traits:

* Comparison traits: [`Eq`][eq], [`PartialEq`][partial-eq], [`Ord`][ord], [`PartialOrd`][partial-ord].
* [`Clone`][clone], to create `T` from `&T` via a copy.
* [`Copy`][copy], to give a type 'copy semantics' instead of 'move semantics'.
* [`Hash`][hash], to compute a hash from `&T`.
* [`Default`][default], to create an empty instance of a data type.
* [`Debug`][debug], to format a value using the `{:?}` formatter.

## Mix Example

~~~admonish tip title="Usage Example of PartialEq, PartialOrd, Debug" collapsible=true 
```rust,editable
// `Centimeters`, a tuple struct that can be compared
#[derive(PartialEq, PartialOrd)]
struct Centimeters(f64);

// `Inches`, a tuple struct that can be printed
#[derive(Debug)]
struct Inches(i32);

impl Inches {
    fn to_centimeters(&self) -> Centimeters {
        let &Inches(inches) = self;

        Centimeters(inches as f64 * 2.54)
    }
}

// `Seconds`, a tuple struct with no additional attributes
struct Seconds(i32);

fn main() {
    let _one_second = Seconds(1);

    // Error: `Seconds` can't be printed; it doesn't implement the `Debug` trait
    //println!("One second looks like: {:?}", _one_second);
    // TODO ^ Try uncommenting this line

    // Error: `Seconds` can't be compared; it doesn't implement the `PartialEq` trait
    //let _this_is_true = (_one_second == _one_second);
    // TODO ^ Try uncommenting this line

    let foot = Inches(12);

    println!("One foot equals {:?}", foot);

    let meter = Centimeters(100.0);

    let cmp =
        if foot.to_centimeters() < meter {
            "smaller"
        } else {
            "bigger"
        };

    println!("One foot is {} than one meter.", cmp);
}
```
~~~

## Clone

When dealing with resources, the default behavior is to transfer them during
assignments or function calls.

> However, sometimes we need to make a copy of the resource as well.

~~~admonish tip title="The [*Clone*][clone] trait helps us do exactly this. Most commonly, we can use the *.clone()* method defined by the *Clone* trait." collapsible=true 
```rust,editable
// A unit struct without resources
#[derive(Debug, Clone, Copy)]
struct Unit;

// A tuple struct with resources that implements the `Clone` trait
#[derive(Clone, Debug)]
struct Pair(Box<i32>, Box<i32>);

fn main() {
    // Instantiate `Unit`
    let unit = Unit;
    // Copy `Unit`, there are no resources to move
    let copied_unit = unit;

    // Both `Unit`s can be used independently
    println!("original: {:?}", unit);
    println!("copy: {:?}", copied_unit);

    // Instantiate `Pair`
    let pair = Pair(Box::new(1), Box::new(2));
    println!("original: {:?}", pair);

    // Move `pair` into `moved_pair`, moves resources
    let moved_pair = pair;
    println!("moved: {:?}", moved_pair);

    // Error! `pair` has lost its resources
    //println!("original: {:?}", pair);
    // TODO ^ Try uncommenting this line

    // Clone `moved_pair` into `cloned_pair` (resources are included)
    let cloned_pair = moved_pair.clone();
    // Drop the original pair using std::mem::drop
    drop(moved_pair);

    // Error! `moved_pair` has been dropped
    //println!("copy: {:?}", moved_pair);
    // TODO ^ Try uncommenting this line

    // The result from .clone() can still be used!
    println!("clone: {:?}", cloned_pair);
}
```
~~~

## See also:

[`derive`][derive]

[attribute]: ../attribute.md

[eq]: https://doc.rust-lang.org/std/cmp/trait.Eq.html

[partial-eq]: https://doc.rust-lang.org/std/cmp/trait.PartialEq.html

[ord]: https://doc.rust-lang.org/std/cmp/trait.Ord.html

[partial-ord]: https://doc.rust-lang.org/std/cmp/trait.PartialOrd.html

[clone]: https://doc.rust-lang.org/std/clone/trait.Clone.html

[copy]: https://doc.rust-lang.org/core/marker/trait.Copy.html

[hash]: https://doc.rust-lang.org/std/hash/trait.Hash.html

[default]: https://doc.rust-lang.org/std/default/trait.Default.html

[debug]: https://doc.rust-lang.org/std/fmt/trait.Debug.html

[derive]: https://doc.rust-lang.org/reference/attributes.html#derive
