# Custom conditionals

~~~admonish tip title="Some conditionals like *target_os* are implicitly provided by *rustc*, but custom conditionals must be passed to *rustc* using the *--cfg* flag." collapsible=true
```rust,editable,ignore,mdbook-runnable
#[cfg(some_condition)]
fn conditional_function() {
    println!("condition met!");
}

fn main() {
    conditional_function();
}
```
~~~

Try to run this to see what happens without the custom `cfg` flag.

~~~admonish tip title="With the custom *cfg* flag:" collapsible=true
```shell
$ rustc --cfg some_condition custom.rs && ./custom
condition met!
```
~~~
