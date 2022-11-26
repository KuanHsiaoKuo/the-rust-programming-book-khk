# The `use` declaration: bind a full path to a new name

The `use` declaration can be used to bind a full path to a new name, for easier
access.

~~~admonish tip title="It is often used like this:" collapsible=true
```rust,editable,ignore
use crate::deeply::nested::{
    my_first_function,
    my_second_function,
    AndATraitType
};

fn main() {
    my_first_function();
}
```
~~~

~~~admonish tip title="You can use the *as* keyword to bind imports to a different name:" collapsible=true

```rust,editable
// Bind the `deeply::nested::function` path to `other_function`.
use deeply::nested::function as other_function;

fn function() {
    println!("called `function()`");
}

mod deeply {
    pub mod nested {
        pub fn function() {
            println!("called `deeply::nested::function()`");
        }
    }
}

fn main() {
    // Easier access to `deeply::nested::function`
    other_function();

    println!("Entering block");
    {
        // This is equivalent to `use deeply::nested::function as function`.
        // This `function()` will shadow the outer one.
        use crate::deeply::nested::function;

        // `use` bindings have a local scope. In this case, the
        // shadowing of `function()` is only in this block.
        function();

        println!("Leaving block");
    }

    function();
}
```
~~~

1. use .. as .. : Bind the `deeply::nested::function` path to `other_function`.
2. `use` bindings have a local scope. In this case, the shadowing of `function()` is only in this block.
