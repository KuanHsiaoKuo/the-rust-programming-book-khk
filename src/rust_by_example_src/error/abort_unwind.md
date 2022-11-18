# `abort` and `unwind`

The previous section illustrates the error handling mechanism `panic`.

Different code paths can be conditionally compiled based on the panic setting.

The current values available are `abort` and `unwind`.

## abort

~~~admonish tip title="Building on the prior lemonade example, we explicitly use the panic strategy to exercise different lines of code. " collapsible=true 
```rust,editable,mdbook-runnable

fn drink(beverage: &str) {
   // You shouldn't drink too much sugary beverages.
    if beverage == "lemonade" {
        if cfg!(panic="abort"){ println!("This is not your party. Run!!!!");}
        else{ println!("Spit it out!!!!");}
    }
    else{ println!("Some refreshing {} is all I need.", beverage); }
}

fn main() {
    drink("water");
    drink("lemonade");
}
```
~~~

## unwind

~~~admonish tip title="Here is another example focusing on rewriting *drink()* and explicitly use the *unwind* keyword." collapsible=true 
```rust,editable

#[cfg(panic = "unwind")]
fn ah(){ println!("Spit it out!!!!");}

#[cfg(not(panic="unwind"))]
fn ah(){ println!("This is not your party. Run!!!!");}

fn drink(beverage: &str){
    if beverage == "lemonade"{ ah();}
    else{println!("Some refreshing {} is all I need.", beverage);}
}

fn main() {
    drink("water");
    drink("lemonade");
}
```

The panic strategy can be set from the command line by using `abort` or `unwind`.

```console
rustc  lemonade.rs -C panic=abort
```
~~~

## cfg! or #cfg

```rust, ignore
#[cfg(panic = "unwind")]
fn ah(){ println!("Spit it out!!!!");}

cfg!(panic="abort"){ println!("This is not your party. Run!!!!");}
```