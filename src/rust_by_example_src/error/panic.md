# `panic`: The simplest error handling mechanism

The simplest error handling mechanism we will see is `panic`.

> Just like try...except mechanism in python

It prints an error message, starts unwinding the stack, and usually exits the program.

~~~admonish tip title="Here, we explicitly call *panic* on our error condition:" collapsible=true 
```rust,editable,ignore,mdbook-runnable
fn drink(beverage: &str) {
    // You shouldn't drink too much sugary beverages.
    if beverage == "lemonade" { panic!("AAAaaaaa!!!!"); }

    println!("Some refreshing {} is all I need.", beverage);
}

fn main() {
    drink("water");
    drink("lemonade");
}
```
~~~